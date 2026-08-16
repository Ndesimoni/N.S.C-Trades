//! What to say when the opening hours are over.
//!
//! **The first thing each session is a report of what was FOUND**, never of
//! anything arriving.
//!
//! Price can walk into a zone while the bot is quiet — over a weekend, or
//! through the opening hours — and still be sitting there when it starts
//! looking. The watcher fires on a *change*, so nothing would ever be said
//! about it, and saying "arrived" would put Sunday's move on Monday's clock.
//!
//! ## Why it waits
//!
//! It used to go the moment the socket opened. The first hours of a day are
//! where a move gets faked and taken back, so that report was of a position
//! price had not committed to — and he would act on it, or learn to ignore it.
//!
//! Now it waits for the settle window to pass and then says where things
//! actually stand. That is the same moment a trade becomes allowed, which is
//! not a coincidence: it is the point the day is worth reading.

use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use nsc_core::levels::{News, Thickness, nearness};
use nsc_core::when::{Rules, opened, settled};

use super::{Watching, pulse, say};

/// How long to leave it before trying the greeting again.
///
/// **This is asked on every price**, and prices arrive about once a second. So
/// a greeting that will not send has to wait, or one failure becomes a request
/// a second at Telegram for as long as it stays broken.
const BEFORE_TRYING_AGAIN: Duration = Duration::minutes(1);

/// Whether this session has been greeted yet.
pub struct Awake {
    /// The session this has already reported for.
    ///
    /// **Per session, not per run.** It was a plain "have I greeted?" flag set
    /// once and never cleared, so a bot left running over a weekend greeted on
    /// the Friday and then never again — and the report of where price stands
    /// is exactly what he wants on the Sunday open, after two days of silence.
    greeted: Option<DateTime<Utc>>,

    /// When it was last attempted, so a failure waits rather than being
    /// retried on the next price.
    tried: Option<DateTime<Utc>>,
}

impl Awake {
    pub fn new() -> Self {
        Awake {
            greeted: None,
            tried: None,
        }
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

        if self.greeted == Some(session) {
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

        let mut all_said = true;

        for seen in watching.values() {
            let reach = seen.pair.reach(thickness);

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
        }

        // **Marked only once it has actually been said.** Marked first, a
        // failed send left the session greeted with nothing sent — and it is
        // greeted once, so the zones price was already sitting in would never
        // have been mentioned at all.
        if all_said {
            self.greeted = Some(session);
        }

        Ok(())
    }
}
