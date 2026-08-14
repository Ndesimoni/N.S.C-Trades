//! Candles that are missing from the file.

use chrono::{DateTime, TimeDelta, Utc};
use nsc_core::candle::Candle;
use nsc_core::timeframe::{DayBoundary, Timeframe};

use crate::error::DataError;

/// A stretch of time where the file has no candles.
///
/// `from` is the open time of the last candle before the hole. `to` is the
/// open time of the first one after it. Everything between is absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hole {
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    missing: i64,
    reason: Reason,
}

/// Why the candles are not there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reason {
    /// The market was shut. A new trading week began inside this hole, so it
    /// covers a Friday-evening close and the Sunday-evening open.
    ///
    /// Expected, and not a fault.
    Weekend,

    /// The instrument's own daily break — the hour it shuts every weekday.
    ///
    /// Gold shuts at 17:00 New York and reopens at 18:00. So does silver,
    /// copper and oil. Spot forex does not shut at all; it runs straight
    /// through, and `break_minutes` is 0 for those.
    ///
    /// This was found in the real exports, not guessed. Gold's 15-minute file
    /// has a hole from 20:45 to 22:00 UTC every single weekday, and calling
    /// ten of those unexplained would have made the report not worth reading.
    DailyBreak,

    /// Nothing explains it. The market was open and the candles are not there.
    ///
    /// **Every one of these is worth looking at.** Some will be real market
    /// closures — Christmas Day, New Year's Day — which this cannot yet tell
    /// apart from a broker losing an afternoon. Saying "unexplained" and
    /// letting you look is honest. Guessing which is which is not.
    Unexplained,
}

impl Hole {
    /// Open time of the last candle before the hole.
    pub fn from(self) -> DateTime<Utc> {
        self.from
    }

    /// Open time of the first candle after the hole.
    pub fn to(self) -> DateTime<Utc> {
        self.to
    }

    /// How many candles are absent.
    pub fn missing(self) -> i64 {
        self.missing
    }

    /// Whether the market being shut accounts for it.
    pub fn reason(self) -> Reason {
        self.reason
    }
}

/// Walks the candles and reports every place the next one is not one step
/// along.
///
/// `step` is the timeframe the file is in. `boundary` is when a trading day and
/// week begin — 17:00 New York, from `config/app.toml`.
///
/// `break_minutes` is how long the instrument shuts for at the start of each
/// trading day: 60 for gold, silver, copper and oil, 0 for spot forex. It comes
/// from `daily_break_minutes` in `config/symbols.toml`, because it is a fact
/// about the instrument and it changes when a broker changes its hours.
///
/// Only works on intraday files. A daily or weekly file has no fixed number of
/// minutes between candles, so "one step along" is not a subtraction, and
/// pretending otherwise would report a hole at every long weekend.
///
/// Candles must already be in time order. The CSV reader sorts them.
pub fn find_holes(
    candles: &[Candle],
    step: Timeframe,
    boundary: &DayBoundary,
    break_minutes: i64,
) -> Result<Vec<Hole>, DataError> {
    let minutes = step
        .intraday_minutes()
        .ok_or_else(|| DataError::NotAFixedStep {
            timeframe: step.to_string(),
        })?;

    let stride =
        TimeDelta::try_minutes(minutes).ok_or(nsc_core::error::CoreError::DateOutOfRange)?;
    let mut holes = Vec::new();

    for pair in candles.windows(2) {
        let (before, after) = (pair[0], pair[1]);
        // Two candles at the same time, or one running backwards, is a broken
        // file rather than a hole. Skipping past it would let the rest of the
        // scan read as clean.
        if after.open_time() <= before.open_time() {
            return Err(DataError::OutOfOrder {
                previous: before.open_time(),
                at: after.open_time(),
            });
        }

        let expected = before.open_time() + stride;

        if after.open_time() <= expected {
            continue;
        }

        let absent = (after.open_time() - expected).num_minutes() / minutes;

        holes.push(Hole {
            from: before.open_time(),
            to: after.open_time(),
            missing: absent,
            reason: reason_for(expected, after.open_time(), boundary, break_minutes)?,
        });
    }

    Ok(holes)
}

/// Decides what accounts for a hole. `first_absent` is when the missing stretch
/// starts; `after` is the first candle back.
///
/// A new trading week starting inside it makes it a weekend. Otherwise, if it
/// runs from a trading-day start to exactly `break_minutes` later, it is the
/// instrument's nightly break.
///
/// Deliberately does **not** work out whether a weekend hole is *bigger* than a
/// weekend — a broker that also lost Friday afternoon is still called Weekend
/// here. Telling those apart needs the exact Friday close time, which belongs
/// in `nsc-core::timeframe` and is not there yet. Until it is, the candle count
/// on the hole is the thing to read: a normal weekend is a fixed number and a
/// bigger one stands out.
fn reason_for(
    first_absent: DateTime<Utc>,
    after: DateTime<Utc>,
    boundary: &DayBoundary,
    break_minutes: i64,
) -> Result<Reason, DataError> {
    let week_of_after = boundary.week_start(after)?;

    if week_of_after > first_absent && week_of_after <= after {
        return Ok(Reason::Weekend);
    }

    if break_minutes > 0 {
        let day = boundary.day_start(after)?;
        let shut = TimeDelta::try_minutes(break_minutes)
            .ok_or(nsc_core::error::CoreError::DateOutOfRange)?;

        if first_absent == day && after == day + shut {
            return Ok(Reason::DailyBreak);
        }
    }

    Ok(Reason::Unexplained)
}
