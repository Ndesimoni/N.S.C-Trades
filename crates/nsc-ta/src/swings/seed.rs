//! The very start of a history, before any swing has been confirmed.

use nsc_core::candle::Candle;
use nsc_core::swing::SwingKind;

use super::leg::Leg;
use super::memory::RunMemory;
use super::run::Extreme;
use super::step::Step;
use crate::config::SwingSettings;
use crate::error::TaError;

/// Where price has been since the first candle.
///
/// A leg needs to know which way it is going, and at the start nothing has
/// happened yet to say. So this keeps the highest and lowest so far, and
/// **whichever came later decides the direction**: a high after a low means
/// price has been rising, and the run is from that low up to that high.
///
/// It works out again from scratch on every candle, so a start that looked
/// like a rise and turns into a fall simply reads as a fall. Nothing is
/// committed until a swing actually confirms.
#[derive(Debug, Clone)]
pub(super) struct Seed {
    highest: Extreme,
    lowest: Extreme,

    /// The lowest price since the highest was set — the give-back, if price
    /// has been rising.
    since_highest: Extreme,

    /// And the other way round.
    since_lowest: Extreme,
}

impl Seed {
    pub fn new(candle: &Candle) -> Self {
        let now = candle.open_time();

        Self {
            highest: Extreme::new(candle.high(), now),
            lowest: Extreme::new(candle.low(), now),
            since_highest: Extreme::new(candle.low(), now),
            since_lowest: Extreme::new(candle.high(), now),
        }
    }

    pub fn take(
        &mut self,
        candle: &Candle,
        settings: &SwingSettings,
        memory: &RunMemory,
    ) -> Result<Step, TaError> {
        if let Some(step) = self.implied().look(candle, settings, memory)? {
            return Ok(step);
        }

        self.absorb(candle);

        Ok(Step::Continue)
    }

    /// The leg the market has been making so far.
    fn implied(&self) -> Leg {
        if self.highest.bar_time >= self.lowest.bar_time {
            Leg::new(
                SwingKind::High,
                self.lowest,
                self.highest,
                self.since_highest,
            )
        } else {
            Leg::new(SwingKind::Low, self.highest, self.lowest, self.since_lowest)
        }
    }

    fn absorb(&mut self, candle: &Candle) {
        let now = candle.open_time();

        if candle.high() > self.highest.price {
            self.highest = Extreme::new(candle.high(), now);
            self.since_highest = Extreme::new(candle.low(), now);
        } else if candle.low() < self.since_highest.price {
            self.since_highest = Extreme::new(candle.low(), now);
        }

        if candle.low() < self.lowest.price {
            self.lowest = Extreme::new(candle.low(), now);
            self.since_lowest = Extreme::new(candle.high(), now);
        } else if candle.high() > self.since_lowest.price {
            self.since_lowest = Extreme::new(candle.high(), now);
        }
    }
}
