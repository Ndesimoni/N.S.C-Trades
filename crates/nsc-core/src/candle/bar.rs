//! One candle.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::error::CoreError;
use crate::price::{Price, PriceDistance};

/// One candle: four prices and the moment it started.
///
/// The fields are hidden. You read them through the small functions below.
///
/// That is on purpose. If the fields were open, anyone could build a candle
/// directly and skip the checks in `new` — and then a candle with a high
/// below its low could get into the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Candle {
    /// When the candle STARTED, in UTC.
    ///
    /// Not when it ended. Storing the close time instead gives you
    /// off-by-one-candle bugs that are horrible to find, because everything
    /// still looks plausible — just shifted by one bar.
    open_time: DateTime<Utc>,

    open: Price,
    high: Price,
    low: Price,
    close: Price,

    /// Almost always `None`.
    ///
    /// Cash forex has no traded volume, and neither do CFDs. Since every
    /// instrument in symbols.toml is one or the other, nothing in this
    /// project ever has real volume. The column exists in the database, so
    /// the field exists here — but no rule may depend on it.
    volume: Option<Decimal>,

    /// `false` while the candle is still forming — the one on the right of a
    /// live chart that keeps moving.
    ///
    /// Analysis must never touch one. Its high and low have not finished
    /// happening yet, so the candle you signalled on is not the candle that
    /// ends up in the history.
    complete: bool,
}

impl Candle {
    /// Builds a candle, refusing one that cannot be real.
    ///
    /// The checks run on every single candle the feed sends — millions of
    /// them when you download a year of history. That is worth it.
    ///
    /// A broken candle does not cause an error later. It quietly becomes a
    /// swing high that never happened, then a level, then a trade. Nothing
    /// ever tells you where it came from.
    ///
    /// **What we do not check: that prices are positive.** In April 2020
    /// oil traded at minus 37 dollars. Refusing negative prices would delete
    /// a real week of history. The rule is: reject what is impossible, not
    /// what is surprising.
    pub fn new(
        open_time: DateTime<Utc>,
        open: Price,
        high: Price,
        low: Price,
        close: Price,
        volume: Option<Decimal>,
        complete: bool,
    ) -> Result<Self, CoreError> {
        if high < low {
            return Err(CoreError::ImpossibleCandle {
                open_time,
                detail: format!("high {high} is below low {low}"),
            });
        }

        if open > high || open < low {
            return Err(CoreError::ImpossibleCandle {
                open_time,
                detail: format!("open {open} is outside the range {low} to {high}"),
            });
        }

        if close > high || close < low {
            return Err(CoreError::ImpossibleCandle {
                open_time,
                detail: format!("close {close} is outside the range {low} to {high}"),
            });
        }

        Ok(Self {
            open_time,
            open,
            high,
            low,
            close,
            volume,
            complete,
        })
    }

    pub fn open_time(&self) -> DateTime<Utc> {
        self.open_time
    }

    pub fn open(&self) -> Price {
        self.open
    }

    pub fn high(&self) -> Price {
        self.high
    }

    pub fn low(&self) -> Price {
        self.low
    }

    pub fn close(&self) -> Price {
        self.close
    }

    pub fn volume(&self) -> Option<Decimal> {
        self.volume
    }

    /// Has this candle finished?
    ///
    /// **Check this before you use the candle for anything.** An unfinished
    /// candle is still moving. Its high and low have not happened yet.
    ///
    /// `nsc-backtest::guards` stops the whole run if an unfinished candle
    /// reaches the analysis. This is how you avoid that.
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// How tall the candle is, top to bottom, wicks included.
    ///
    /// ATR is built from this. ATR is the size of a normal candle, and it is
    /// the yardstick nearly every setting in this system is measured
    /// against.
    pub fn range(&self) -> PriceDistance {
        self.high - self.low
    }

    /// How far the candle travelled, open to close, ignoring the wicks.
    ///
    /// This keeps its sign. Positive means it closed up, negative means it
    /// closed down — that is how you know the direction, so do not throw it
    /// away here. Anyone who only wants the size can ask for `.abs()`.
    pub fn body(&self) -> PriceDistance {
        self.close - self.open
    }
}
