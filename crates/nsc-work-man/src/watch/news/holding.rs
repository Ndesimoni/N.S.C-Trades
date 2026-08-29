//! What it has downloaded, and what it has already said.

use std::collections::HashSet;

use chrono::{DateTime, Duration, Utc};
use nsc_core::news::{Event, Rules, due, together};
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
        let alive: HashSet<String> = parsed.events.iter().map(Event::key).collect();
        self.said.retain(|key| alive.contains(key));

        self.events = parsed.events;
        self.fetched = Some(now);
    }

    /// Sends a card for anything due that he has not been told about.
    async fn maybe_speak(&mut self, client: &reqwest::Client, now: DateTime<Utc>) {
        let ready: Vec<&Event> = self
            .events
            .iter()
            .filter(|event| due(event, now, &self.rules) && !self.said.contains(&event.key()))
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
                Ok(()) => spoken.extend(group.iter().map(|event| event.key())),
                Err(trouble) => eprintln!("Could not send the news card: {trouble:#}"),
            }
        }

        self.said.extend(spoken);
    }
}
