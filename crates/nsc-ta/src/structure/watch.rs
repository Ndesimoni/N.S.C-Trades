//! An extreme being watched, and whether price has taken it.

use chrono::{DateTime, Utc};
use nsc_core::price::{Price, PriceDistance};
use nsc_core::structure::StructureBreak;
use nsc_core::swing::{Swing, SwingKind};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::error::TaError;

/// A swing that has not been taken out yet, and the run that made it.
///
/// The run travels with it because everything is measured against that
/// particular move. Keeping them together means the two cannot be paired up
/// wrongly later.
#[derive(Debug, Clone, Copy)]
pub(super) struct Marker {
    swing: Swing,
    run: PriceDistance,
}

impl Marker {
    pub fn new(swing: Swing, started_at: Price) -> Self {
        Self {
            swing,
            run: (swing.price() - started_at).abs(),
        }
    }

    pub fn kind(&self) -> SwingKind {
        self.swing.kind()
    }

    /// Has price gone past this extreme, and then far enough past it?
    ///
    /// `reached` is the furthest price got on this candle in the direction
    /// that would break it — the high for a high, the low for a low.
    pub fn taken(
        &self,
        reached: Price,
        now: DateTime<Utc>,
        min_follow_through: f64,
    ) -> Result<Option<StructureBreak>, TaError> {
        let carried = match self.swing.kind() {
            SwingKind::High => reached - self.swing.price(),
            SwingKind::Low => self.swing.price() - reached,
        };

        if !self.far_enough(carried, min_follow_through) {
            return Ok(None);
        }

        Ok(Some(StructureBreak::new(
            self.swing.kind(),
            self.swing.price(),
            self.swing.bar_time(),
            self.run,
            carried,
            now,
        )?))
    }

    /// Did price carry a big enough share of the run past the extreme?
    ///
    /// Plain decimal maths rather than borrowing `AtrMultiple`. The thing
    /// being scaled here is a run, not the size of a normal candle, and a type
    /// whose name does not match what it holds is how the next person gets
    /// misled.
    fn far_enough(&self, carried: PriceDistance, needed: f64) -> bool {
        if self.run.value() <= Decimal::ZERO {
            return false;
        }

        let Some(needed) = Decimal::from_f64(needed) else {
            return false;
        };

        carried.value() >= self.run.value() * needed
    }
}
