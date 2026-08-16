//! The report itself, and what it remembers.

use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use nsc_core::levels::{News, Thickness, nearness};
use nsc_core::when::{Rules, opened, settled};

use crate::watch::{Watching, pulse, say};

/// How long to leave it before trying the greeting again.
///
/// **This is asked on every price**, and prices arrive about once a second. So
/// a greeting that will not send has to wait, or one failure becomes a request
/// a second at Telegram for as long as it stays broken.
const BEFORE_TRYING_AGAIN: Duration = Duration::minutes(1);

/// Whether this session has been greeted yet.
pub struct Awake {
    /// Which session each pair has already been reported for.
    ///
    /// **Per session, not per run.** It was a plain "have I greeted?" flag set
    /// once and never cleared, so a bot left running over a weekend greeted on
    /// the Friday and then never again — and the report of where price stands
    /// is exactly what he wants on the Sunday open, after two days of silence.
    ///
    /// **And per pair, not per bot.** A pair he sends a level for mid-session
    /// gets its bands built fresh, and is owed its own report — see `forget`.
    pub(super) greeted: HashMap<String, DateTime<Utc>>,

    /// When it was last attempted, so a failure waits rather than being
    /// retried on the next price.
    tried: Option<DateTime<Utc>>,
}

impl Awake {
    pub fn new() -> Self {
        Awake {
            greeted: HashMap::new(),
            tried: None,
        }
    }

    /// **This pair is owed a fresh report.**
    ///
    /// Called when its bands have been rebuilt, which happens when he sends a
    /// level for it. He usually draws a level because price is NEAR it, and a
    /// rebuilt `Watch` treats its first price as a baseline — so without this
    /// he got "your levels are live" and then nothing at all about the zone
    /// price was already sitting in.
    pub fn forget(&mut self, symbol: &str) {
        self.greeted.remove(symbol);
    }

    /// Reports every zone price is already at, once a session.
    ///
    /// Says nothing until the opening hours are over. Called on every price,
    /// so it is cheap to ask and answers "not yet" nearly always.
    pub async fn greet(
        &mut self,
        client: &reqwest::Client,
        watching: &HashMap<String, Watching>,
        thickness: Thickness,
        calendar: &Rules,
        pulse: &mut pulse::Pulse,
    ) -> Result<()> {
        let now = Utc::now();
        let session = opened(now, calendar);

        // **Only pairs a price has actually arrived for.**
        //
        // The report is of which zones price is RESTING IN, and nothing rests
        // anywhere until a price has been fed in. Reported before that, it
        // finds nothing, says nothing, and marks the session done — so the one
        // message this file exists to send never comes.
        //
        // `line.rs` feeds the price in first. This is here so that staying
        // true does not depend on two lines staying in that order.
        let owed: Vec<&String> = watching
            .iter()
            .filter(|(_, seen)| seen.watch.last_price().is_some())
            .map(|(symbol, _)| symbol)
            .filter(|symbol| self.greeted.get(*symbol) != Some(&session))
            .collect();

        if owed.is_empty() {
            return Ok(());
        }

        // The opening hours. Prices are still watched and remembered — this is
        // only about not saying anything yet.
        if !settled(now, calendar) {
            return Ok(());
        }

        // Tried recently and it did not go. Leave it alone — the next price is
        // a second away, not a new opportunity.
        if self
            .tried
            .is_some_and(|last| now - last < BEFORE_TRYING_AGAIN)
        {
            return Ok(());
        }

        self.tried = Some(now);

        let owed: Vec<String> = owed.into_iter().cloned().collect();

        for symbol in owed {
            let Some(seen) = watching.get(&symbol) else {
                continue;
            };

            let reach = seen.pair.reach(thickness);
            let mut all_said = true;

            for band in seen.watch.resting_at() {
                // Where price is now. `resting_at` says WHICH bands, the last
                // price says where inside them — and the socket may not send
                // another for a second or two.
                let at = seen.watch.last_price().unwrap_or(band.price);

                println!("{} was already at {}", seen.pair.symbol, band.price);

                match say::alert(
                    client,
                    &seen.pair,
                    &band,
                    nearness(&band, at, reach),
                    News::Already,
                    at,
                    reach,
                )
                .await
                {
                    Ok(()) => pulse.spoke(Utc::now()),
                    Err(trouble) => {
                        eprintln!("Could not say what was already there: {trouble:#}");
                        all_said = false;
                    }
                }
            }

            // **Marked only once it has actually been said**, and per pair, so
            // one card failing does not cost the others their report. Marked
            // first, a failed send left the session greeted with nothing sent
            // — and it is greeted once a session, so the zones price was
            // sitting in would never have been mentioned at all.
            if all_said {
                self.greeted.insert(symbol, session);
            }
        }

        Ok(())
    }
}
