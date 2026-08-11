//! Where the trading day and the trading week begin.

use chrono::{DateTime, Datelike, Days, LocalResult, NaiveDate, NaiveTime, TimeZone, Utc, Weekday};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

use crate::error::CoreError;

/// When the trading day ends, and which day the week starts on.
///
/// Built once from `config/app.toml` and passed around. Everything in this
/// system that needs to know where a day begins asks this, so there is
/// exactly one answer.
///
/// Holding a named timezone rather than a fixed offset is the whole point.
/// 5pm New York is 21:00 UTC in summer and 22:00 UTC in winter. Store the
/// offset and your daily candles are an hour out for roughly five months of
/// the year — with nothing to tell you, because no error ever fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DayBoundary {
    close: NaiveTime,
    tz: Tz,
    week_starts_on: Weekday,
}

impl DayBoundary {
    /// `hour`/`minute` are local to `tz`, not UTC. For forex that is normally
    /// 17:00 in `America/New_York`, with the week starting on Sunday.
    pub fn new(hour: u32, minute: u32, tz: Tz, week_starts_on: Weekday) -> Result<Self, CoreError> {
        let close = NaiveTime::from_hms_opt(hour, minute, 0)
            .ok_or(CoreError::InvalidTimeOfDay { hour, minute })?;

        Ok(Self {
            close,
            tz,
            week_starts_on,
        })
    }

    /// The instant the trading day containing `at` began.
    ///
    /// Note what this means for a Sunday evening. The market opens Sunday at
    /// 5pm New York, so a moment at 6pm on Sunday is already inside the
    /// trading day that ends Monday afternoon — the one traders call Monday's
    /// session, even though the calendar still says Sunday.
    ///
    /// That is the day this returns, and it is why a "no trading on Mondays"
    /// rule has to mean the trading day rather than the calendar day.
    pub fn day_start(&self, at: DateTime<Utc>) -> Result<DateTime<Utc>, CoreError> {
        let local = at.with_timezone(&self.tz);
        let mut date = local.date_naive();

        // Before today's close, we are still in the day that opened
        // yesterday afternoon.
        if local.time() < self.close {
            date = date.pred_opt().ok_or(CoreError::DateOutOfRange)?;
        }

        self.close_on(date)
    }

    /// The instant the next trading day begins.
    ///
    /// Worked out from the calendar date rather than by adding 24 hours,
    /// because on the two days a year the clocks change, a trading day is 23
    /// or 25 hours long.
    pub fn next_day_start(&self, at: DateTime<Utc>) -> Result<DateTime<Utc>, CoreError> {
        let start = self.day_start(at)?;
        let date = start.with_timezone(&self.tz).date_naive();
        let next = date.succ_opt().ok_or(CoreError::DateOutOfRange)?;

        self.close_on(next)
    }

    /// The instant the trading week containing `at` began.
    ///
    /// Walks back to the most recent week-start day. For forex that is the
    /// Sunday afternoon open, so a weekly candle covers Sunday 5pm to Friday
    /// 5pm — five days, not seven.
    pub fn week_start(&self, at: DateTime<Utc>) -> Result<DateTime<Utc>, CoreError> {
        let day_start = self.day_start(at)?;
        let mut date = day_start.with_timezone(&self.tz).date_naive();

        // Bounded loop: a week has seven days, so this always finishes.
        for _ in 0..7 {
            if date.weekday() == self.week_starts_on {
                return self.close_on(date);
            }
            date = date.pred_opt().ok_or(CoreError::DateOutOfRange)?;
        }

        Err(CoreError::DateOutOfRange)
    }

    /// The instant the next trading week begins.
    pub fn next_week_start(&self, at: DateTime<Utc>) -> Result<DateTime<Utc>, CoreError> {
        let start = self.week_start(at)?;
        let date = start.with_timezone(&self.tz).date_naive();
        let next = date
            .checked_add_days(Days::new(7))
            .ok_or(CoreError::DateOutOfRange)?;

        self.close_on(next)
    }

    /// Turns "the close, on this local date" into a real moment in UTC.
    ///
    /// The two odd cases are both about clocks changing:
    ///
    /// - **Ambiguous** — clocks went back, so the local time happens twice.
    ///   We take the first. Arbitrary, but it has to be the same answer every
    ///   run or the backtester stops being repeatable.
    /// - **Does not exist** — clocks jumped forward over it. Only possible if
    ///   the daily close is set to something inside the gap, around 2am. At
    ///   5pm it cannot happen, but it is reported rather than guessed at.
    fn close_on(&self, date: NaiveDate) -> Result<DateTime<Utc>, CoreError> {
        let local = date.and_time(self.close);

        match self.tz.from_local_datetime(&local) {
            LocalResult::Single(dt) => Ok(dt.with_timezone(&Utc)),
            LocalResult::Ambiguous(first, _) => Ok(first.with_timezone(&Utc)),
            LocalResult::None => Err(CoreError::LocalTimeDoesNotExist {
                local: local.to_string(),
                tz: self.tz.to_string(),
            }),
        }
    }
}
