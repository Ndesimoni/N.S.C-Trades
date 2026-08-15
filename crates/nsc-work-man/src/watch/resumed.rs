//! What to say when watching starts again.
//!
//! **The first thing after silence is a report of what was FOUND**, never of
//! anything arriving.
//!
//! Price can walk into a zone during Monday's silence and still be sitting
//! there when Tuesday opens. The watcher fires on a *change*, so nothing would
//! ever be said about it — and saying "arrived" would put a Monday move on a
//! Tuesday clock.

use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use nsc_core::levels::{News, Thickness, nearness};

use super::{Watching, pulse, say};

/// How long to leave it before trying the greeting again.
///
/// **This is asked on every price**, and prices arrive about once a second. So
/// a greeting that will not send has to wait, or one failure becomes a request
/// a second at Telegram for as long as it stays broken.
const BEFORE_TRYING_AGAIN: Duration = Duration::minutes(1);

/// Whether this session has been greeted yet.
pub enum Awake {
    /// Nothing said yet.
    ///
    /// `since` is when it was last attempted, so a failure waits rather than
    /// being retried on the next price.
    Waiting { since: Option<DateTime<Utc>> },

    /// Already reported. Normal watching from here.
    Greeted,
}

impl Awake {
    pub fn new() -> Self {
        Awake::Waiting { since: None }
    }

    /// Reports every zone price is already at, once.
    pub async fn greet(
        &mut self,
        client: &reqwest::Client,
        watching: &HashMap<String, Watching>,
        thickness: Thickness,
        pulse: &mut pulse::Pulse,
    ) -> Result<()> {
        let now = Utc::now();

        match self {
            Awake::Greeted => return Ok(()),

            // Tried recently and it did not go. Leave it alone — the next
            // price is a second away, not a new opportunity.
            Awake::Waiting { since: Some(last) } if now - *last < BEFORE_TRYING_AGAIN => {
                return Ok(());
            }

            Awake::Waiting { since } => *since = Some(now),
        }

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
            *self = Awake::Greeted;
        }

        Ok(())
    }
}
