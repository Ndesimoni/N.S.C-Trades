//! The ten-minute check, and what it decides to ask about.

use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Utc};
use nsc_core::candle::Bar;
use nsc_core::levels::Thickness;
use nsc_core::when::Rules;
use nsc_data::source::Interval;
use nsc_data::sources::ibkr::IbkrConnection;
use nsc_data::store::{self, Store};
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

    /// The two settings rung 3 needs, read once at startup.
    ///
    /// **`None` turns rung 3 off and leaves everything else running.** A
    /// missing or unreadable `config/strategy.toml` must not cost him the
    /// alerts and the closes — those are the job, and the shape at the level
    /// is what was added on top. It is said out loud at startup rather than
    /// swallowed, because rules that quietly never fire look exactly like a
    /// quiet week.
    rung_three: Option<(nsc_strategy::Rules, nsc_ta::pattern::Rules)>,

    /// Where finished candles are kept, when there is somewhere to keep them.
    ///
    /// **`None` runs the whole bot with no database.** Losing a row is worth
    /// far less than losing an alert, and the record is a record — nothing
    /// reads it while the bot is running.
    record: Option<Store>,
}

impl Closes {
    pub fn new(
        rung_three: Option<(nsc_strategy::Rules, nsc_ta::pattern::Rules)>,
        record: Option<Store>,
    ) -> Self {
        Closes {
            rung_three,
            record,
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
    /// Puts the finished candles in the record.
    ///
    /// **Only finished ones.** A candle still forming is not a candle yet, and
    /// storing it would put a half-drawn bar in the one place that is supposed
    /// to be the truth — where a backtest would later read it as settled.
    ///
    /// **Nothing here can stop an alert.** A database that will not answer is
    /// a lost row; a blocked price loop is a missed setup. It says so once and
    /// carries on.
    async fn keep(
        &self,
        symbol: &str,
        interval: Interval,
        bars: &[Bar],
        now: DateTime<Utc>,
        minutes: i64,
    ) {
        let Some(record) = &self.record else {
            return;
        };

        let done = finished_only(bars, now, minutes);

        if done.is_empty() {
            return;
        }

        if let Err(trouble) = store::write(record, symbol, interval, &done).await {
            eprintln!(
                "Could not keep the {} candles for {symbol}: {trouble}",
                interval.spoken()
            );
        }
    }

    pub async fn look(
        &mut self,
        client: &reqwest::Client,
        ibkr: &IbkrConnection,
        watching: &mut HashMap<String, Watching>,
        thickness: Thickness,
        _calendar: &Rules,
        pulse: &mut pulse::Pulse,
    ) -> Result<()> {
        for seen in watching.values_mut() {
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

                // **Every finished candle goes into the record, here.**
                //
                // The bot used to read a candle, judge it and throw it away,
                // so the history stopped on the day it was last downloaded by
                // hand. Charts were never affected — those come from IBKR —
                // but the record went stale and the backtester would have read
                // years with a hole at the end.
                //
                // **The whole reply is written, not just the newest one.** It
                // costs one statement either way, and it means a bot that was
                // off for a day fills that day in on its next look. The key is
                // `(symbol, interval, opened_at)` and the write is
                // `ON CONFLICT DO UPDATE`, so re-writing the same 399 candles
                // repairs rather than duplicates.
                self.keep(&seen.pair.symbol, interval, &bars, now, minutes)
                    .await;

                let ask_again = worth_asking_again(&bars, minutes, now);

                // **Newest first, so the first one that has finished is the
                // one that just did.** Anything after the newest is proved
                // finished by the candle that opened after it — see
                // `has_finished`, and the quarter of 4-hour candles that do
                // not last four hours.
                let finished = bars
                    .iter()
                    .enumerate()
                    .find(|(at, bar)| has_finished(bar, *at, now, minutes))
                    .map(|(_, bar)| bar);

                let mut all_sent = true;

                if let Some(bar) = finished {
                    // ── RUNG 2: only on the timeframes he asked for ──
                    //
                    // His call, 31 August 2026: *"we don't want those
                    // notifications from the one hour. The only notification we
                    // want from the one hour should be a setup."*
                    //
                    // **The 1-hour is still watched, and still fetched.** It
                    // has to be, because rung 3 runs on it — a candlestick
                    // pattern at a zone is the whole reason the 1-hour is here.
                    // What stops is only the card about what the candle did.
                    if thickness.says_closes_on(interval.stored()) {
                        all_sent &= self
                            .say(client, seen, &live, bar, thickness, interval, pulse)
                            .await;
                    }

                    // **Rung 3, and only ever on a finished candle.** A shape
                    // halfway through a candle is not a shape, and one that
                    // un-forms before the close would have been a message
                    // about something that never happened.
                    //
                    // It carries its own key, so a candle worth both messages
                    // — what it did at the band, and the shape it completed —
                    // sends both.
                    // Copied out first — both are small `Copy` structs, and
                    // holding a borrow of `self` across a call that needs
                    // `&mut self` is what the borrow checker is for.
                    if let Some((rules, patterns)) = self.rung_three {
                        self.setup(
                            client, seen, &live, &bars, bar, interval, &patterns, &rules, pulse,
                        )
                        .await;
                    }
                }

                // **NOTHING HERE READS AN UNFINISHED CANDLE ANY MORE.**
                //
                // There used to be a mid-candle look — a third of the way in,
                // marked "so far" on the card's face. It was the one place in
                // this project that read a candle still running.
                //
                // Taken out on 27 August 2026, his call. Two messages per zone
                // visit and never three: the alert when price arrives, and the
                // close when the candle finishes OUTSIDE the band. A third one
                // in between was a heads-up about a heads-up.
                //
                // **Its going is worth more than the card was.** The rule that
                // matters most here is that a candle still forming is
                // invisible to the analysis, and the exception was the only
                // thing standing between that rule and being true everywhere.

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

/// The candles that have **finished**, and only those.
///
/// **A candle still forming is not a candle yet.** Storing one would put a
/// half-drawn bar in the one place that is meant to be the truth, where a
/// backtest reads it months later as settled — and a backtest that reads a
/// price the market had not printed does not look broken, it looks *better*.
///
/// The reply from the feed carries both: the candle that has just closed and
/// the one still running. This is the line between them.
pub(super) fn finished_only(bars: &[Bar], now: DateTime<Utc>, minutes: i64) -> Vec<Bar> {
    bars.iter()
        .enumerate()
        .filter(|(at, bar)| has_finished(bar, *at, now, minutes))
        .map(|(_, bar)| bar.clone())
        .collect()
}

/// Has this candle finished?
///
/// `bars` come back **newest first**, and `at` is where this one sits in that
/// list.
///
/// ## A LATER CANDLE IS THE PROOF, NOT A STOPWATCH
///
/// Anything after the newest has a candle that opened after it, and the feed
/// does not open a candle until the one before it is done. **That is the
/// broker's own answer, and it needs no arithmetic.**
///
/// **Asking a stopwatch instead was wrong for a quarter of them.** IBKR ends
/// its forex day at 17:15 New York and prints short candles around the
/// boundary — measured on 30,000 AUD/USD 4-hour candles: 21,446 ran the full
/// 240 minutes and **7,675 did not**, some as short as 75.
///
/// `opened_at + 240 <= now` calls a 75-minute candle unfinished for another
/// 165 minutes after it has closed. Nothing errors. The close card simply
/// arrives late — up to two and three-quarter hours late, twice a day, on
/// every pair.
///
/// **It was late, never early**, which is the safe direction and is why it
/// went unseen: an early read would be price the market had not printed.
///
/// **Only the newest has nothing after it**, and there the clock is all there
/// is. It stays conservative there for the same reason — better a candle
/// reported late than one reported before it closed.
fn has_finished(bar: &Bar, at: usize, now: DateTime<Utc>, minutes: i64) -> bool {
    // A stamp that will not read is not a candle. Never guessed at.
    if bar.opened_at().is_err() {
        return false;
    }

    at > 0 || bar.finished_by(now, minutes).unwrap_or(false)
}
