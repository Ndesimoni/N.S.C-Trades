//! One level: a band, and everything known about it.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::band::Band;
use super::hidden::NotDrawn;
use super::origin::Origin;
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

    /// Who decided this price was a level.
    ///
    /// A hand-drawn one has no touch count and no touch dates, so those
    /// accessors give `None` rather than a made-up number.
    origin: Origin,

    /// Why this level does not get a line, if it does not.
    ///
    /// A drawing rule, not a deletion. The level is still here, still counted,
    /// and the engine still knows the price matters. Two timeframes turning at
    /// one price is confluence, and confluence is the thing worth trading —
    /// delete it and you throw away the reason the price is good.
    not_drawn: Option<NotDrawn>,
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
            origin: Origin::Found,
            not_drawn: None,
        })
    }

    /// A level the trader drew himself, read from `config/levels/`.
    ///
    /// Far fewer checks than [`Level::new`], because far less is claimed. A
    /// hand-drawn level has a band, a timeframe and the day it was drawn.
    /// There is no touch count to validate — he drew it because a big move
    /// ended there, not because price turned some number of times.
    ///
    /// `from` is the day he drew it, and the level does not exist before it.
    /// That is the same guard the found levels obey: a level drawn today
    /// knows what price did last year, so using it on last year's candles
    /// would make a backtest look better than anything tradeable.
    pub fn drawn_by_hand(band: Band, timeframe: Timeframe, from: DateTime<Utc>) -> Self {
        Self {
            band,
            timeframe,
            touches: 0,
            first_touch: from,
            last_touch: from,
            confirmed_at: from,
            origin: Origin::DrawnByHand,
            not_drawn: None,
        }
    }

    /// Marks this level as covered by one from a higher timeframe.
    ///
    /// Refuses a timeframe that is not bigger. The rule is that the bigger
    /// timeframe always wins, and this is where that gets enforced rather than
    /// remembered — a 4-hour level can never swallow a weekly one, however the
    /// calling code is written.
    pub fn covered_by(self, higher: Timeframe) -> Result<Self, CoreError> {
        if higher <= self.timeframe {
            return Err(CoreError::ImpossibleLevel {
                detail: format!(
                    "a {} level cannot be covered by {higher} — the bigger timeframe wins",
                    self.timeframe
                ),
            });
        }

        Ok(Self {
            not_drawn: Some(NotDrawn::CoveredBy(higher)),
            ..self
        })
    }

    /// Marks this level as crowded out by a better one on its own timeframe.
    ///
    /// The consolidation case: price chopped around one area and turned a
    /// dozen times, so there is a level at every turn. The most-touched one
    /// keeps the line and the rest are crowded out.
    pub fn crowded_out(self) -> Self {
        Self {
            not_drawn: Some(NotDrawn::CrowdedOut),
            ..self
        }
    }

    pub fn band(self) -> Band {
        self.band
    }

    pub fn timeframe(self) -> Timeframe {
        self.timeframe
    }

    pub fn origin(self) -> Origin {
        self.origin
    }

    /// How many times price has turned here, if that is why it is a level.
    ///
    /// `None` for a hand-drawn level. It was drawn because a big move ended
    /// there, and that does not have a count.
    ///
    /// A number, not a verdict. What counts as a lot is a trading decision
    /// and it is set in `config/strategy.toml`.
    pub fn touches(self) -> Option<u32> {
        match self.origin {
            Origin::Found => Some(self.touches),
            Origin::DrawnByHand => None,
        }
    }

    /// When price first turned here. `None` for a hand-drawn level.
    pub fn first_touch(self) -> Option<DateTime<Utc>> {
        match self.origin {
            Origin::Found => Some(self.first_touch),
            Origin::DrawnByHand => None,
        }
    }

    /// When price last turned here. `None` for a hand-drawn level.
    pub fn last_touch(self) -> Option<DateTime<Utc>> {
        match self.origin {
            Origin::Found => Some(self.last_touch),
            Origin::DrawnByHand => None,
        }
    }

    pub fn confirmed_at(self) -> DateTime<Utc> {
        self.confirmed_at
    }

    /// Why this level has no line, if it has none.
    pub fn not_drawn(self) -> Option<NotDrawn> {
        self.not_drawn
    }

    /// The bigger timeframe covering this price, if that is why it is hidden.
    pub fn absorbed_by(self) -> Option<Timeframe> {
        self.not_drawn.and_then(NotDrawn::covering_timeframe)
    }

    /// Should this level be drawn on the chart?
    ///
    /// False when a bigger timeframe already covers the price, or when a
    /// better level on this same timeframe sits too near. The level still
    /// exists and still counts either way.
    pub fn is_drawn(self) -> bool {
        self.not_drawn.is_none()
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
