//! The ten-minute check, and what it decides to ask about.

use std::collections::HashMap;

use anyhow::Result;
use chrono::Utc;
use nsc_core::levels::Thickness;
use nsc_core::when::Rules;
use tokio::time::{Duration, Instant, sleep_until};

use super::fetch::{REPORT_ON, fetch};
use super::said::Said;
use crate::watch::{Watching, pulse};

/// How often to look. On the hour would need a clock that agrees with the
/// feed's; every ten minutes just asks.
const LOOK_EVERY: Duration = Duration::from_secs(600);

pub struct Closes {
    /// The last candle already reported, per pair, per interval, per kind, per
    /// zone. Keyed by the feed's own stamp, so nothing is reported twice.
    pub(super) told: HashMap<Said, String>,
    next: Instant,
}

impl Closes {
    pub fn new() -> Self {
        Closes {
            told: HashMap::new(),
            // Not immediately. The bands were just sized, and the rate limit
            // does not care that these are different requests.
            next: Instant::now() + LOOK_EVERY,
        }
    }

    /// Sleeps until the next check is due.
    pub async fn next_check(&self) {
        sleep_until(self.next).await
    }

    /// Push the next check out.
    ///
    /// **Called the moment the timer fires, before anything decides to skip
    /// the check.** Left until after the work, a day that skips it — a Monday,
    /// say — leaves the deadline in the past and the loop spins as fast as the
    /// processor will go.
    pub fn tick(&mut self) {
        self.next = Instant::now() + LOOK_EVERY;
    }

    /// Asks about every pair price is currently at a zone of.
    pub async fn look(
        &mut self,
        client: &reqwest::Client,
        watching: &HashMap<String, Watching>,
        thickness: Thickness,
        calendar: &Rules,
        pulse: &mut pulse::Pulse,
    ) -> Result<()> {
        for seen in watching.values() {
            let live = seen.watch.resting_at();
            if live.is_empty() {
                continue;
            }

            for (interval, minutes) in REPORT_ON {
                // ONE REQUEST SERVES BOTH. The reply carries the candle that
                // has just finished and the one still running, so the
                // mid-candle look costs nothing on top of the close.
                // **A candle that will not fetch is not the price line
                // breaking.** This used to travel out of here, out of `listen`
                // and into `run`, which dropped a perfectly good socket and —
                // once the trouble had lasted five minutes — told him the
                // price line was down. It is a different connection entirely.
                //
                // Nothing is remembered, so the next look asks again.
                let bars = match fetch(client, &seen.pair.symbol, interval).await {
                    Ok(bars) => bars,
                    Err(trouble) => {
                        eprintln!(
                            "Could not get the {interval} candle for {}: {trouble:#}",
                            seen.pair.symbol
                        );
                        continue;
                    }
                };

                let now = Utc::now();

                let finished = bars
                    .iter()
                    .find(|bar| bar.finished_by(now, minutes).unwrap_or(false));

                if let Some(bar) = finished {
                    self.say(client, seen, &live, bar, thickness, interval, false, pulse)
                        .await;
                }

                // ── the look into a candle still running ──
                //
                // THE ONE PLACE IN THIS PROJECT THAT READS AN UNFINISHED
                // CANDLE. It is allowed here for the reason the price alert is
                // allowed: it is a heads-up and nothing more, and the card says
                // so on its face. IT MUST NEVER REACH A STRATEGY.
                //
                // Not on the open, either. Spot forex runs without a break so
                // an open IS the last close — that message would repeat what
                // arrived a minute earlier.
                let waited = minutes * calendar.look_in_minutes / 60;

                let running = bars.iter().find(|bar| {
                    !bar.finished_by(now, minutes).unwrap_or(true)
                        && bar.finished_by(now, waited).unwrap_or(false)
                });

                if let Some(bar) = running {
                    self.say(client, seen, &live, bar, thickness, interval, true, pulse)
                        .await;
                }
            }
        }

        Ok(())
    }

    /// Have we already spoken about this candle, in this way?
    ///
    /// **It only asks.** Marking it before the card has actually gone was a
    /// way to lose a close for good: a hiccup reaching Telegram, and the
    /// candle is remembered as reported and never tried again. A close is the
    /// thing he is waiting for.
    pub(super) fn already_said(&self, key: &Said, stamp: &str) -> bool {
        self.told.get(key).is_some_and(|told| told == stamp)
    }
}
