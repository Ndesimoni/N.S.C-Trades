//! One old extreme, properly taken out.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};

use crate::error::CoreError;
use crate::price::{Price, PriceDistance};
use crate::swing::SwingKind;

/// A high or a low that price took out and then carried on past.
///
/// Two separate events, and both are needed. Crossing the old extreme starts
/// it; carrying far enough past finishes it. Everything in between is price
/// poking at a level, which is the most ordinary thing a chart does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructureBreak {
    /// Which kind of extreme was taken. A high taken out is bullish.
    kind: SwingKind,

    /// The price that was taken.
    broken: Price,

    /// The candle the broken extreme sat on.
    broken_at: DateTime<Utc>,

    /// The run that made that extreme — the move up to a high, or down to a
    /// low. Everything here is measured against it.
    previous_run: PriceDistance,

    /// How far past the old extreme price went, measured from the extreme
    /// itself rather than from where the pullback started.
    carried: PriceDistance,

    /// The candle where price had carried far enough. The first moment this
    /// break could be acted on.
    at: DateTime<Utc>,
}

impl StructureBreak {
    /// Refuses a break that cannot be real.
    ///
    /// The date check is the important one. A break has to happen after the
    /// extreme it breaks, and if that is ever untrue then whatever built it
    /// read the chart backwards.
    pub fn new(
        kind: SwingKind,
        broken: Price,
        broken_at: DateTime<Utc>,
        previous_run: PriceDistance,
        carried: PriceDistance,
        at: DateTime<Utc>,
    ) -> Result<Self, CoreError> {
        if at <= broken_at {
            return Err(CoreError::ImpossibleStructureBreak {
                detail: format!("a break at {at} cannot take out an extreme from {broken_at}"),
            });
        }

        if previous_run.value() <= Decimal::ZERO {
            return Err(CoreError::ImpossibleStructureBreak {
                detail: "an extreme with no run behind it has nothing to measure against".into(),
            });
        }

        if carried.value() <= Decimal::ZERO {
            return Err(CoreError::ImpossibleStructureBreak {
                detail: "price did not carry past the extreme at all".into(),
            });
        }

        Ok(Self {
            kind,
            broken,
            broken_at,
            previous_run,
            carried,
            at,
        })
    }

    pub fn kind(self) -> SwingKind {
        self.kind
    }

    pub fn broken(self) -> Price {
        self.broken
    }

    pub fn broken_at(self) -> DateTime<Utc> {
        self.broken_at
    }

    pub fn previous_run(self) -> PriceDistance {
        self.previous_run
    }

    pub fn carried(self) -> PriceDistance {
        self.carried
    }

    /// When the break completed.
    pub fn at(self) -> DateTime<Utc> {
        self.at
    }

    /// How far past the old extreme price went, as a share of the run that
    /// made it. The number that had to reach the threshold.
    ///
    /// **Worth keeping rather than throwing away once the test has passed.**
    /// A break that carries twice the previous run is a different-strength
    /// event from one that scrapes past the minimum, and the rules layer wants
    /// to know which it was.
    pub fn share_of_run(self) -> Option<f64> {
        (self.carried.value() / self.previous_run.value()).to_f64()
    }
}
