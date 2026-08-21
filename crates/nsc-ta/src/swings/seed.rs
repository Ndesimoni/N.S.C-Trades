//! The very start of a history, before any swing has been confirmed.

use chrono::{DateTime, Utc};
use nsc_core::candle::Bar;
use nsc_core::swing::{SwingError, SwingKind};

use super::Rules;
use super::extreme::Extreme;
use super::leg::Leg;
use super::memory::RunMemory;
use super::step::Step;

/// Where price has been since the first candle.
///
/// A leg needs to know which way it is going, and at the start nothing has
/// happened yet to say. So this keeps the highest and lowest so far, and
/// **whichever came later decides the direction** — a high after a low means
/// price has been rising, and the run is from that low up to that high.
///
/// It works out again from scratch on every candle, so a start that looked
/// like a rise and turns into a fall simply reads as a fall. **Nothing is
/// committed until a swing actually confirms.**
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
    pub(super) fn new(bar: &Bar, now: DateTime<Utc>) -> Self {
        Seed {
            highest: Extreme::new(bar.high, now),
            lowest: Extreme::new(bar.low, now),
            since_highest: Extreme::new(bar.low, now),
            since_lowest: Extreme::new(bar.high, now),
        }
    }

    pub(super) fn take(
        &mut self,
        bar: &Bar,
        now: DateTime<Utc>,
        rules: &Rules,
        memory: &RunMemory,
    ) -> Result<Step, SwingError> {
        if let Some(step) = self.implied().look(bar, now, rules, memory)? {
            return Ok(step);
        }

        self.absorb(bar, now);

        Ok(Step::Continue)
    }

    /// The leg the market has been making so far.
    fn implied(&self) -> Leg {
        if self.highest.bar_time >= self.lowest.bar_time {
            // The high came later, so price has been rising.
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

    fn absorb(&mut self, bar: &Bar, now: DateTime<Utc>) {
        if bar.high > self.highest.price {
            self.highest = Extreme::new(bar.high, now);
            self.since_highest = Extreme::new(bar.low, now);
        } else if bar.low < self.since_highest.price {
            self.since_highest = Extreme::new(bar.low, now);
        }

        if bar.low < self.lowest.price {
            self.lowest = Extreme::new(bar.low, now);
            self.since_lowest = Extreme::new(bar.high, now);
        } else if bar.high > self.since_lowest.price {
            self.since_lowest = Extreme::new(bar.high, now);
        }
    }
}
