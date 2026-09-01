//! What it has downloaded, and what it has already said.

use std::collections::HashSet;

use chrono::{DateTime, Duration, Utc};
use nsc_core::news::{Event, Rules, due_at, together};
use nsc_data::news::fetch;
use nsc_data::store::{self, Store};

use super::saying;

/// How often to wake and look.
///
/// A minute. Waking costs a glance at a list already in memory — it only
/// reaches the network every `refresh_hours`.
///
/// **THE NARROWEST WINDOW MUST STAY WIDER THAN THIS.** On his settings the
/// tightest is the last mark: one minute before a release through
/// `stale_minutes` after it, so six minutes wide. A tick cannot step over it.
///
/// It had thirty minutes of slack until 1 September 2026 and now has six,
/// which is still plenty — but it is the reason not to add a mark narrower
/// than a minute. A test pins it.
const LOOK: std::time::Duration = std::time::Duration::from_secs(60);

/// Watches the calendar forever.
///
/// **Never returns.** Spawned at startup beside the inbox, and every trouble
/// inside is reported rather than raised — a news source that cannot be
/// reached must not take the price watcher down with it.
pub async fn run(client: reqwest::Client, rules: Rules, record: Option<Store>) {
    let mut held = Held::new(rules, record);

    // **Before the first download.** A restart then has the whole week
    // immediately rather than after a round trip — and if the feed is
    // unreachable, it has one at all.
    held.read_the_record(Utc::now()).await;

    loop {
        held.once(&client).await;
        tokio::time::sleep(LOOK).await;
    }
}

/// The week's events, and which of them he has been told about.
struct Held {
    rules: Rules,
    events: Vec<Event>,
    fetched: Option<DateTime<Utc>>,

    /// Where the week is kept, when there is somewhere to keep it.
    ///
    /// **It is read once, at startup, and written every download after that.**
    /// Reading it means a restart has the whole week before the first request
    /// finishes — and that a feed which is unreachable at startup no longer
    /// means no news warnings at all.
    record: Option<Store>,

    /// **What stops him hearing the same release all afternoon.** The file is
    /// downloaded every few hours and the same event is in every copy of it,
    /// so "already said" has to survive a re-read. `Event::key` is what makes
    /// that possible.
    said: HashSet<String>,
}

impl Held {
    fn new(rules: Rules, record: Option<Store>) -> Self {
        Held {
            rules,
            events: Vec::new(),
            fetched: None,
            record,
            said: HashSet::new(),
        }
    }

    async fn once(&mut self, client: &reqwest::Client) {
        let now = Utc::now();

        self.maybe_download(client, now).await;
        self.maybe_speak(client, now).await;
    }

    /// **What the record already holds**, read once before the first download.
    ///
    /// It does not set `fetched`, so a download still happens straight after —
    /// this only covers the seconds until it lands, and the days when it does
    /// not land at all.
    ///
    /// A window rather than the lot: three days back covers the far edge of
    /// any warning, and eight days forward covers the week however it was
    /// written.
    async fn read_the_record(&mut self, now: DateTime<Utc>) {
        let Some(record) = &self.record else {
            return;
        };

        let from = now - Duration::days(3);
        let to = now + Duration::days(8);

        match store::news_between(record, from, to).await {
            Ok(events) if events.is_empty() => {}

            Ok(events) => {
                println!(
                    "Economic calendar: {} events read back from the record.",
                    events.len()
                );
                self.events = events;
            }

            // **Nothing here can end the run.** A record that will not answer
            // costs the seconds until the download lands.
            Err(trouble) => eprintln!("Could not read the calendar back: {trouble}"),
        }
    }

    /// Puts the week in the record.
    ///
    /// **Nothing here can end the run either.** A week that will not save is a
    /// gap in the history and a slower restart. It is not a reason to stop
    /// warning him about the news he can already see.
    async fn keep(&self, now: DateTime<Utc>) {
        let Some(record) = &self.record else {
            return;
        };

        match store::news_write(record, &self.events, now).await {
            Ok(rows) => println!("Kept {rows} calendar rows."),
            Err(trouble) => eprintln!("Could not keep the calendar: {trouble}"),
        }
    }

    /// Is it time to ask for the file again?
    fn stale(&self, now: DateTime<Utc>) -> bool {
        match self.fetched {
            None => true,
            Some(last) => now - last >= Duration::hours(self.rules.refresh_hours.max(1)),
        }
    }

    /// Downloads the week, and **keeps the old list if it cannot**.
    ///
    /// The failure is said out loud and that is all. Clearing what it already
    /// had would turn one refused download into a silent afternoon, and a
    /// silent afternoon looks exactly like a quiet week.
    async fn maybe_download(&mut self, client: &reqwest::Client, now: DateTime<Utc>) {
        if !self.stale(now) {
            return;
        }

        let parsed = match fetch(client, &self.rules.url).await {
            Ok(parsed) => parsed,
            Err(trouble) => {
                eprintln!("Could not read the economic calendar: {trouble}");
                return;
            }
        };

        if parsed.unreadable > 0 {
            eprintln!(
                "The calendar had {} row{} whose time made no sense. They were left out.",
                parsed.unreadable,
                if parsed.unreadable == 1 { "" } else { "s" }
            );
        }

        println!(
            "Economic calendar: {} events for the week.",
            parsed.events.len()
        );

        // **Forget what was said about a week that is over.** Otherwise this
        // grows for as long as the bot runs. Keeping only keys still in the
        // file prunes itself when the week rolls, with nothing to schedule.
        //
        // **Every mark of every event**, because what is remembered is "this
        // event, at this warning" and not just the event.
        let mut alive: HashSet<String> = HashSet::new();

        for event in &parsed.events {
            for mark in &self.rules.warn_at_minutes {
                alive.insert(told(event, *mark));
            }
        }

        self.said.retain(|key| alive.contains(key));

        self.events = parsed.events;
        self.fetched = Some(now);

        self.keep(now).await;
    }

    /// Sends a card for anything due that he has not been told about.
    async fn maybe_speak(&mut self, client: &reqwest::Client, now: DateTime<Utc>) {
        // **Which warning is live decides whether he has heard this already.**
        // Keyed on the event alone, the half-hour card would silence the
        // five-minute one — the card he most wants, since it is the one that
        // arrives while he still has time to act on it.
        let ready: Vec<&Event> = self
            .events
            .iter()
            .filter(|event| match due_at(event, now, &self.rules) {
                Some(mark) => !self.said.contains(&told(event, mark)),
                None => false,
            })
            .collect();

        if ready.is_empty() {
            return;
        }

        let mut spoken = Vec::new();

        for group in together(&ready) {
            match saying::card(client, &group, now).await {
                // **Marked only once it has gone.** Marked first, one failed
                // send would lose the warning for good — the same mistake the
                // heartbeat made, where marking it early silenced it for a
                // whole day.
                Ok(()) => spoken.extend(group.iter().filter_map(|event| {
                    due_at(event, now, &self.rules).map(|mark| told(event, mark))
                })),
                Err(trouble) => eprintln!("Could not send the news card: {trouble:#}"),
            }
        }

        self.said.extend(spoken);
    }
}

/// What one warning about one event is remembered as.
///
/// **The mark is part of it**, so the heads-up five minutes out does not
/// silence the last call a minute out. They are two different messages about the
/// same release and he asked for both.
fn told(event: &Event, mark: i64) -> String {
    format!("{}@{mark}", event.key())
}
