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
use nsc_core::levels::{Thickness, action, what_it_did};
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

pub struct Closes {
    /// The last candle already reported, per pair, per interval. Keyed by the
    /// feed's own stamp, so nothing is reported twice.
    told: HashMap<(String, &'static str), String>,
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
        pulse: &mut pulse::Pulse,
    ) -> Result<()> {
        for seen in watching.values() {
            let live = seen.watch.resting_at();
            if live.is_empty() {
                continue;
            }

            for (interval, minutes) in REPORT_ON {
                let Some(bar) =
                    newest_finished(client, &seen.pair.symbol, interval, minutes).await?
                else {
                    continue;
                };

                let key = (seen.pair.symbol.clone(), interval);
                if self.told.get(&key) == Some(&bar.datetime) {
                    continue;
                }
                self.told.insert(key, bar.datetime.clone());

                for band in &live {
                    let did = what_it_did(band, &bar);
                    if !did.worth_saying() {
                        continue;
                    }

                    let was = action(band, &bar, thickness.kiss_depth);

                    println!(
                        "{} {interval} candle {} — {was:?} at {}",
                        seen.pair.symbol, bar.datetime, band.price
                    );

                    say::closed(client, &seen.pair, band, &bar, did, was, interval).await?;
                    pulse.spoke(Utc::now());
                }
            }
        }

        Ok(())
    }
}

/// The newest candle that has actually finished.
///
/// **Asked of the clock, never of position in the list.** The newest one is
/// usually still running, but not always — ask at 16:00:02 and you get either
/// the 16:00 candle already open, if a price has landed, or the 15:00 one now
/// finished, if none has.
async fn newest_finished(
    client: &reqwest::Client,
    symbol: &str,
    interval: &str,
    minutes: i64,
) -> Result<Option<Bar>> {
    let series = keep_trying(3, || feed::for_pair(client, symbol, interval, FEW))
        .await
        .with_context(|| format!("could not get the {interval} candle for {symbol}"))?;

    tokio::time::sleep(BREATHE).await;

    let now = Utc::now();

    Ok(series
        .values
        .into_iter()
        .find(|bar| bar.finished_by(now, minutes).unwrap_or(false)))
}
