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
use nsc_core::levels::{News, Thickness, nearness};

use super::{Watching, pulse, say};

/// Whether this session has been greeted yet.
pub enum Awake {
    /// Nothing said yet. The next price triggers the report.
    Waiting,

    /// Already reported. Normal watching from here.
    Greeted,
}

impl Awake {
    pub fn new() -> Self {
        Awake::Waiting
    }

    /// Reports every zone price is already at, once.
    pub async fn greet(
        &mut self,
        client: &reqwest::Client,
        watching: &HashMap<String, Watching>,
        thickness: Thickness,
        pulse: &mut pulse::Pulse,
    ) -> Result<()> {
        if matches!(self, Awake::Greeted) {
            return Ok(());
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
                    Ok(()) => pulse.spoke(chrono::Utc::now()),
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
