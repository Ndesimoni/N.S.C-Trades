//! Rung 2 — what a candle did at a zone, once it has finished.
//!
//! **Only pairs with price at a zone are ever fetched.** A quiet week costs
//! nothing. That is the same principle that killed the design where a candle
//! was fetched on every close on every pair whether anything had happened or
//! not.
//!
//! ## It never works out when a candle closes
//!
//! It asks, on the hour, for the newest candle and lets the feed's own stamp
//! say whether that is one it has already reported.
//!
//! Working out the boundaries here would mean knowing where the feed puts its
//! 4-hour candles, which nobody has measured. Guessing wrong reports a candle
//! that has not happened, and that is the mistake that makes results look
//! better rather than broken.

use std::collections::HashMap;

use anyhow::{Context, Result};
use chrono::Utc;
use nsc_core::candle::Bar;
use nsc_core::levels::{Band, Thickness, action, what_it_did};
use nsc_core::when::Rules;
use nsc_work_man::{feed, retry::keep_trying};
use tokio::time::{Duration, Instant, sleep_until};

use super::{BREATHE, Watching, pulse, say};

/// The timeframes he executes on. The level's own timeframe says how thick the
/// band is; these say which candles report.
const REPORT_ON: [(&str, i64); 2] = [("1h", 60), ("4h", 240)];

/// How many candles to ask for. Two is enough to find a finished one whether
/// or not the newest is still forming.
const FEW: usize = 3;

/// How often to look. On the hour would need a clock that agrees with the
/// feed's; every ten minutes just asks.
const LOOK_EVERY: Duration = Duration::from_secs(600);

/// Which of the two things was said about a candle.
///
/// A candle gets spoken about **twice** — once part-way through, once when it
/// finishes — and they must be remembered apart, or the look silences the
/// close that follows it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Kind {
    /// Part-way through, while it was still running.
    SoFar,
    /// Finished.
    Closed,
}

pub struct Closes {
    /// The last candle already reported, per pair, per interval, per kind.
    /// Keyed by the feed's own stamp, so nothing is reported twice.
    told: HashMap<(String, &'static str, Kind), String>,
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
                let bars = fetch(client, &seen.pair.symbol, interval).await?;
                let now = Utc::now();

                let finished = bars
                    .iter()
                    .find(|bar| bar.finished_by(now, minutes).unwrap_or(false));

                if let Some(bar) = finished {
                    let key = (seen.pair.symbol.clone(), interval, Kind::Closed);
                    if self.fresh(key, &bar.datetime) {
                        self.report(client, seen, &live, bar, thickness, interval, false, pulse)
                            .await?;
                    }
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
                    let key = (seen.pair.symbol.clone(), interval, Kind::SoFar);
                    if self.fresh(key, &bar.datetime) {
                        self.report(client, seen, &live, bar, thickness, interval, true, pulse)
                            .await?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Have we already spoken about this candle, in this way?
    fn fresh(&mut self, key: (String, &'static str, Kind), stamp: &str) -> bool {
        if self.told.get(&key).is_some_and(|told| told == stamp) {
            return false;
        }

        self.told.insert(key, stamp.to_string());
        true
    }

    /// Says what one candle did at every zone price is at.
    #[allow(clippy::too_many_arguments)]
    async fn report(
        &self,
        client: &reqwest::Client,
        seen: &Watching,
        live: &[Band],
        bar: &Bar,
        thickness: Thickness,
        interval: &'static str,
        forming: bool,
        pulse: &mut pulse::Pulse,
    ) -> Result<()> {
        for band in live {
            let did = what_it_did(band, bar);
            if !did.worth_saying() {
                continue;
            }

            let was = action(band, bar, thickness.kiss_depth);
            let what = if forming { "so far" } else { "closed" };

            println!(
                "{} {interval} candle {} — {what} {was:?} at {}",
                seen.pair.symbol, bar.datetime, band.price
            );

            say::closed(client, &seen.pair, band, bar, did, was, interval, forming).await?;
            pulse.spoke(Utc::now());
        }

        Ok(())
    }
}

/// The last few candles, newest first.
///
/// **Which one has finished is asked of the clock, never of position in the
/// list.** The newest is usually still running, but not always — ask at
/// 16:00:02 and you get either the 16:00 candle already open, if a price has
/// landed, or the 15:00 one now finished, if none has.
async fn fetch(client: &reqwest::Client, symbol: &str, interval: &str) -> Result<Vec<Bar>> {
    let series = keep_trying(3, || feed::for_pair(client, symbol, interval, FEW))
        .await
        .with_context(|| format!("could not get the {interval} candle for {symbol}"))?;

    tokio::time::sleep(BREATHE).await;

    Ok(series.values)
}
