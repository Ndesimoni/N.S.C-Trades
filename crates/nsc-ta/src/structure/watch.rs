//! An extreme being watched, and what price does at it.

use chrono::{DateTime, Utc};
use nsc_core::price::{Price, PriceDistance};
use nsc_core::structure::{FailedAttempt, StructureBreak, StructureEvent};
use nsc_core::swing::{Swing, SwingKind};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::error::TaError;

/// A push past the extreme that has not finished yet.
#[derive(Debug, Clone, Copy)]
struct Attempt {
    /// The candle price first went past the extreme on.
    from: DateTime<Utc>,

    /// The furthest past it has got.
    best: PriceDistance,
}

/// A swing that has not been taken out yet, and the run that made it.
///
/// The run travels with it because everything is measured against that
/// particular move. Keeping them together means the two cannot be paired up
/// wrongly later.
#[derive(Debug, Clone, Copy)]
pub(super) struct Marker {
    swing: Swing,
    run: PriceDistance,
    attempt: Option<Attempt>,
}

impl Marker {
    pub fn new(swing: Swing, started_at: Price) -> Self {
        Self {
            swing,
            run: (swing.price() - started_at).abs(),
            attempt: None,
        }
    }

    pub fn kind(&self) -> SwingKind {
        self.swing.kind()
    }

    /// What this candle did at the extreme.
    ///
    /// `reached` is the furthest price got on this candle in the direction
    /// that would break it — the high for a high, the low for a low.
    ///
    /// Three outcomes. Far enough past and the extreme is taken. Past it but
    /// not far enough and an attempt is under way, which says nothing yet.
    /// Back under it with an attempt under way and that attempt has failed,
    /// which is worth recording.
    pub fn saw(
        &mut self,
        reached: Price,
        now: DateTime<Utc>,
        min_follow_through: f64,
    ) -> Result<Option<StructureEvent>, TaError> {
        let carried = match self.swing.kind() {
            SwingKind::High => reached - self.swing.price(),
            SwingKind::Low => self.swing.price() - reached,
        };

        if self.far_enough(carried, min_follow_through) {
            return Ok(Some(StructureEvent::Taken(self.taken(carried, now)?)));
        }

        if carried.value() > Decimal::ZERO {
            self.push(carried, now);
            return Ok(None);
        }

        self.gave_up(now)
            .map(|failed| failed.map(StructureEvent::Failed))
    }

    /// Price is past the extreme but short of the follow-through. Nothing has
    /// been proved, so this only keeps track of how far it has got.
    fn push(&mut self, carried: PriceDistance, now: DateTime<Utc>) {
        match &mut self.attempt {
            Some(attempt) => attempt.best = attempt.best.max(carried),
            None => {
                self.attempt = Some(Attempt {
                    from: now,
                    best: carried,
                })
            }
        }
    }

    /// Price is back under the extreme. If it had been past, that push is over
    /// and it failed.
    fn gave_up(&mut self, now: DateTime<Utc>) -> Result<Option<FailedAttempt>, TaError> {
        let Some(attempt) = self.attempt.take() else {
            return Ok(None);
        };

        Ok(Some(FailedAttempt::new(
            self.swing.kind(),
            self.swing.price(),
            self.swing.bar_time(),
            self.run,
            attempt.best,
            attempt.from,
            now,
        )?))
    }

    fn taken(&self, carried: PriceDistance, now: DateTime<Utc>) -> Result<StructureBreak, TaError> {
        Ok(StructureBreak::new(
            self.swing.kind(),
            self.swing.price(),
            self.swing.bar_time(),
            self.run,
            carried,
            now,
        )?)
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
