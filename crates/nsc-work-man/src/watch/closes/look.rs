//! The ten-minute check, and what it decides to ask about.

use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Utc};
use nsc_core::levels::Thickness;
use nsc_core::when::Rules;
use nsc_data::source::Interval;
use nsc_data::sources::ibkr::IbkrConnection;
use tokio::time::{Duration, Instant, sleep_until};

use super::due::{when_next, worth_asking_again};
use super::fetch::{REPORT_ON, fetch};
use super::said::Said;
use crate::watch::{Watching, pulse};

/// The longest it will go without waking.
///
/// **This is housekeeping, not the candle check.** The heartbeat, the calendar
/// and the levels-changed look all ride on this tick. When a candle is due
/// sooner than this, the tick is pulled forward to meet it.
const LOOK_EVERY: Duration = Duration::from_secs(600);

pub struct Closes {
    /// The last candle already reported, per pair, per interval, per kind, per
    /// zone. Keyed by the feed's own stamp, so nothing is reported twice.
    pub(super) told: HashMap<Said, String>,

    /// When each pair and interval is next worth a request.
    ///
    /// **This is what stopped it asking every ten minutes.** A 4-hour candle
    /// closes six times a day; asked on a timer, about 140 of every 144 asks
    /// found nothing new. The feed's own stamp says when the next one is due,
    /// so it waits for that instead.
    due: HashMap<(String, Interval), DateTime<Utc>>,

    next: Instant,
}

impl Closes {
    pub fn new() -> Self {
        Closes {
            told: HashMap::new(),
            due: HashMap::new(),
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

    /// Pulls the next wake forward to meet a candle that is due sooner.
    ///
    /// **Never past `LOOK_EVERY`**, because the heartbeat and the calendar
    /// ride on the same tick, and never sooner than now — a moment already
    /// past would spin the loop.
    fn wake_by(&mut self, at: DateTime<Utc>) {
        let Ok(from_now) = (at - Utc::now()).to_std() else {
            return;
        };

        self.next = self.next.min(Instant::now() + from_now);
    }

    /// Asks about every pair price is currently at a zone of.
    pub async fn look(
        &mut self,
        client: &reqwest::Client,
        ibkr: &IbkrConnection,
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

            for interval in REPORT_ON {
                let minutes = interval.minutes();
                let waiting_on = (seen.pair.symbol.clone(), interval);

                // **Nothing can have changed yet.** The feed already told us
                // when this pair's next candle is due; asking before then is
                // a request spent to be told the same thing again.
                if let Some(due) = self.due.get(&waiting_on)
                    && Utc::now() < *due
                {
                    self.wake_by(*due);
                    continue;
                }

                // ONE REQUEST SERVES BOTH. The reply carries the candle that
                // has just finished and the one still running, so the
                // mid-candle look costs nothing on top of the close.
                //
                // **A candle that will not fetch is not the price line
                // breaking.** This used to travel out of here, out of `listen`
                // and into `run`, which dropped a perfectly good socket and —
                // once the trouble had lasted five minutes — told him the
                // price line was down. It is a different connection entirely.
                //
                // Nothing is remembered, so the next look asks again.
                let bars = match fetch(ibkr, &seen.pair.symbol, interval).await {
                    Ok(bars) => bars,
                    Err(trouble) => {
                        eprintln!(
                            "Could not get the {} candle for {}: {trouble:#}",
                            interval.spoken(),
                            seen.pair.symbol
                        );
                        continue;
                    }
                };

                let now = Utc::now();
                let waited = minutes * calendar.look_in_minutes / 60;

                let ask_again = worth_asking_again(&bars, minutes, waited, now);

                let finished = bars
                    .iter()
                    .find(|bar| bar.finished_by(now, minutes).unwrap_or(false));

                let mut all_sent = true;

                if let Some(bar) = finished {
                    all_sent &= self
                        .say(client, seen, &live, bar, thickness, interval, false, pulse)
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
                let running = bars.iter().find(|bar| {
                    !bar.finished_by(now, minutes).unwrap_or(true)
                        && bar.finished_by(now, waited).unwrap_or(false)
                });

                if let Some(bar) = running {
                    all_sent &= self
                        .say(client, seen, &live, bar, thickness, interval, true, pulse)
                        .await;
                }

                // **Set after the cards have gone, not before.** Pushed
                // forward first, a card that would not send waited for the
                // NEXT candle to come round — four hours on the 4-hour, for
                // something the bot had already read. Nothing is marked as
                // told when a send fails, so coming back in a minute puts it
                // right.
                let ask_again = when_next(all_sent, ask_again, Utc::now());

                self.due.insert(waiting_on, ask_again);
                self.wake_by(ask_again);
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
