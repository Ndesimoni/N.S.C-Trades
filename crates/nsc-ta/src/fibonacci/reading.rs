//! The four levels of a move, and where price sits against them.

use nsc_core::fib::FibRetracement;
use nsc_core::price::Price;

use crate::config::FibSettings;
use crate::error::TaError;

/// A move, its four levels, and how far price has come back.
///
/// **It reports, it does not judge.** Whether being in the golden zone is a
/// reason to buy needs the trend, the level underneath and the candle printing
/// there — all of which live in `nsc-strategy`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FibReading {
    of: FibRetracement,

    strong_trend: Price,
    golden_from: Price,
    golden_to: Price,
    stop: Price,

    depth: f64,
}

impl FibReading {
    /// Measures a move against the settings, at the price it has come back to.
    pub fn take(
        of: FibRetracement,
        price_now: Price,
        settings: &FibSettings,
    ) -> Result<Option<Self>, TaError> {
        settings.validate()?;

        let [shallow, deep] = settings.golden_zone;

        let Some(depth) = of.depth_at(price_now) else {
            return Ok(None);
        };

        Ok(Some(Self {
            of,
            strong_trend: of.level(settings.strong_trend_level)?,
            golden_from: of.level(shallow)?,
            golden_to: of.level(deep)?,
            stop: of.level(settings.stop_level)?,
            depth,
        }))
    }

    /// The move the levels were drawn from.
    ///
    /// Kept and handed on, because when a Fibonacci signal looks wrong the
    /// move it chose is nearly always the disagreement — and an argument about
    /// a move is one that can be settled by looking at a chart.
    pub fn of(self) -> FibRetracement {
        self.of
    }

    /// How far price has come back, as a share of the move.
    pub fn depth(self) -> f64 {
        self.depth
    }

    /// Where the trend-strength level sits. Price turning at or before it
    /// means the market barely paused.
    pub fn strong_trend(self) -> Price {
        self.strong_trend
    }

    /// The shallow edge of the golden zone.
    pub fn golden_from(self) -> Price {
        self.golden_from
    }

    /// The deep edge of the golden zone.
    pub fn golden_to(self) -> Price {
        self.golden_to
    }

    /// Where a stop gets looked at. Not where it goes — that is the
    /// invalidation layer's call, weighing this against the rest of the chart.
    pub fn stop(self) -> Price {
        self.stop
    }

    /// Is price inside the golden zone right now?
    ///
    /// A fact about where price is, and nothing more. The edges count as
    /// inside, because a zone that excludes its own boundary is a zone price
    /// misses by a tick.
    pub fn in_golden_zone(self, settings: &FibSettings) -> bool {
        let [shallow, deep] = settings.golden_zone;

        self.depth >= shallow && self.depth <= deep
    }

    /// Has price come back further than the stop level?
    ///
    /// Past there the move it was drawn from is in real trouble — but what to
    /// do about that is a rule, not a reading.
    pub fn past_the_stop(self, settings: &FibSettings) -> bool {
        self.depth > settings.stop_level
    }
}
