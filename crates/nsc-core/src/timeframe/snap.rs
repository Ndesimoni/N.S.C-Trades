//! Which candle a given moment belongs to.
//!
//! Snapping is the operation everything else is built on — every candle we
//! store, every aggregation from small candles into big ones, every "has this
//! candle finished yet" check.

use chrono::{DateTime, TimeDelta, Utc};

use super::boundary::DayBoundary;
use super::kind::Timeframe;
use crate::error::CoreError;

impl Timeframe {
    /// The start of the candle that `at` falls inside.
    ///
    /// Give it 14:37 on a 15-minute chart and it answers 14:30.
    pub fn candle_start(
        self,
        at: DateTime<Utc>,
        boundary: &DayBoundary,
    ) -> Result<DateTime<Utc>, CoreError> {
        match self.intraday_minutes() {
            // Daily and weekly are boundaries in their own right.
            None => match self {
                Timeframe::W1 => boundary.week_start(at),
                _ => boundary.day_start(at),
            },

            // Intraday: count minutes from the daily close, then round down
            // to the nearest whole candle. Counting from the close rather
            // than from midnight is what makes six 4-hour candles fill
            // exactly one day.
            Some(minutes) => {
                let day_start = boundary.day_start(at)?;
                let elapsed = (at - day_start).num_minutes();
                let into_candle = elapsed.rem_euclid(minutes);

                let step = TimeDelta::try_minutes(elapsed - into_candle)
                    .ok_or(CoreError::DateOutOfRange)?;

                Ok(day_start + step)
            }
        }
    }

    /// When the candle containing `at` finishes — which is the same instant
    /// the next one starts.
    ///
    /// Used to answer "is this candle complete?". A candle is only complete
    /// once the clock has passed this point, and unfinished candles must
    /// never reach the analysis.
    pub fn candle_end(
        self,
        at: DateTime<Utc>,
        boundary: &DayBoundary,
    ) -> Result<DateTime<Utc>, CoreError> {
        match self.intraday_minutes() {
            None => match self {
                Timeframe::W1 => boundary.next_week_start(at),
                _ => boundary.next_day_start(at),
            },

            Some(minutes) => {
                let start = self.candle_start(at, boundary)?;
                let step = TimeDelta::try_minutes(minutes).ok_or(CoreError::DateOutOfRange)?;

                Ok(start + step)
            }
        }
    }
}
