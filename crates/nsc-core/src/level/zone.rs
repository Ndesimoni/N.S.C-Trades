//! One level: a band, and everything known about it.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::band::Band;
use crate::error::CoreError;
use crate::price::{Price, PriceDistance};
use crate::timeframe::Timeframe;

/// A price band the market has turned at before.
///
/// Every field here is a fact off the chart. Nothing here says whether the
/// level will hold — see `mod.rs` for why that lives in `nsc-strategy`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Level {
    band: Band,

    /// The timeframe it was found on, and it keeps that tag everywhere.
    ///
    /// There is one set of levels, not a set per chart. A daily level is
    /// still a daily level when you are looking at the 4-hour, and that is
    /// exactly why it matters there.
    timeframe: Timeframe,

    /// How many swing points sit inside the band.
    touches: u32,

    /// The candle of the oldest touch.
    first_touch: DateTime<Utc>,

    /// The candle of the newest touch.
    last_touch: DateTime<Utc>,

    /// The first moment you could have known this level, with this many
    /// touches, was here.
    ///
    /// Always later than `last_touch`. A touch is a swing point, and a swing
    /// is only knowable a few candles after the candle it sits on.
    confirmed_at: DateTime<Utc>,
}

impl Level {
    /// Builds a level, refusing one that cannot be real.
    ///
    /// The `confirmed_at` check is the important one. If it fires, whatever
    /// built the level used a swing before that swing had confirmed — and
    /// that mistake never shows up as a bad result, only as a backtest that
    /// looks better than the bot can trade.
    pub fn new(
        band: Band,
        timeframe: Timeframe,
        touches: u32,
        first_touch: DateTime<Utc>,
        last_touch: DateTime<Utc>,
        confirmed_at: DateTime<Utc>,
    ) -> Result<Self, CoreError> {
        if touches == 0 {
            return Err(CoreError::ImpossibleLevel {
                detail: "a level with no touches is not a level".into(),
            });
        }

        if last_touch < first_touch {
            return Err(CoreError::ImpossibleLevel {
                detail: format!("the last touch {last_touch} comes before the first {first_touch}"),
            });
        }

        if confirmed_at <= last_touch {
            return Err(CoreError::LevelKnownTooEarly {
                last_touch,
                confirmed_at,
            });
        }

        Ok(Self {
            band,
            timeframe,
            touches,
            first_touch,
            last_touch,
            confirmed_at,
        })
    }

    pub fn band(self) -> Band {
        self.band
    }

    pub fn timeframe(self) -> Timeframe {
        self.timeframe
    }

    /// How many times price has turned here.
    ///
    /// A number, not a verdict. What counts as a lot is a trading decision
    /// and it is set in `config/strategy.toml`.
    pub fn touches(self) -> u32 {
        self.touches
    }

    pub fn first_touch(self) -> DateTime<Utc> {
        self.first_touch
    }

    pub fn last_touch(self) -> DateTime<Utc> {
        self.last_touch
    }

    pub fn confirmed_at(self) -> DateTime<Utc> {
        self.confirmed_at
    }

    /// The middle of the band.
    pub fn centre(self) -> Price {
        self.band.centre()
    }

    /// Is price inside the band?
    pub fn contains(self, price: Price) -> bool {
        self.band.contains(price)
    }

    /// How far price is from the band, and which side it is on. Zero means
    /// price is in it.
    pub fn distance_to(self, price: Price) -> PriceDistance {
        self.band.distance_to(price)
    }

    /// Could you have known about this level at `now`?
    ///
    /// **Call this before using a level for anything.** Trading a level built
    /// from a swing you had not seen yet is the quiet mistake this whole
    /// design exists to prevent.
    pub fn is_known_at(self, now: DateTime<Utc>) -> bool {
        now >= self.confirmed_at
    }
}
