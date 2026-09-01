//! What it has downloaded, and what it has already said.

use std::collections::HashSet;

use chrono::{DateTime, Duration, Utc};
use nsc_core::news::{Event, Rules, due_at, together};
use nsc_data::news::fetch;

use super::saying;

/// How often to wake and look.
///
/// A minute. The window is thirty minutes wide, so nothing is missed — and
/// waking costs a glance at a list already in memory. It only reaches the
/// network every `refresh_hours`.
const LOOK: std::time::Duration = std::time::Duration::from_secs(60);

/// Watches the calendar forever.
///
/// **Never returns.** Spawned at startup beside the inbox, and every trouble
/// inside is reported rather than raised — a news source that cannot be
/// reached must not take the price watcher down with it.
pub async fn run(client: reqwest::Client, rules: Rules) {
    let mut held = Held::new(rules);

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

    /// **What stops him hearing the same release all afternoon.** The file is
    /// downloaded every few hours and the same event is in every copy of it,
    /// so "already said" has to survive a re-read. `Event::key` is what makes
    /// that possible.
    said: HashSet<String>,
}

impl Held {
    fn new(rules: Rules) -> Self {
        Held {
            rules,
            events: Vec::new(),
            fetched: None,
            said: HashSet::new(),
        }
    }

    async fn once(&mut self, client: &reqwest::Client) {
        let now = Utc::now();

        self.maybe_download(client, now).await;
        self.maybe_speak(client, now).await;
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
/// **The mark is part of it**, so a heads-up half an hour out does not silence
/// the last call five minutes out. They are two different messages about the
/// same release and he asked for both.
fn told(event: &Event, mark: i64) -> String {
    format!("{}@{mark}", event.key())
}
