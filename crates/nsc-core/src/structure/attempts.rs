//! An old extreme that price crossed and could not hold past.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};

use crate::error::CoreError;
use crate::price::{Price, PriceDistance};
use crate::swing::SwingKind;

/// Price went past an old high, ran out of steam short of the follow-through,
/// and came back under it.
///
/// **Not a break, and not nothing either.** The market tried and failed there,
/// and that is worth as much as the times it succeeded — it is the "do not
/// take this" side of the training data, and it cannot be collected
/// retrospectively.
///
/// The extreme stays on the books afterwards. A later attempt that does carry
/// far enough still completes the break, and this record sits alongside it as
/// what happened first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FailedAttempt {
    /// Which kind of extreme was attempted. A high attempted is a failed push
    /// upward.
    kind: SwingKind,

    /// The price price could not hold past.
    attempted: Price,

    /// The candle the attempted extreme sat on.
    attempted_at: DateTime<Utc>,

    /// The run that made that extreme. What the attempt is measured against.
    previous_run: PriceDistance,

    /// The furthest past the extreme price managed before giving up.
    best: PriceDistance,

    /// When price first went past the extreme.
    from: DateTime<Utc>,

    /// The candle where price was back under it, so the attempt was over.
    to: DateTime<Utc>,
}

impl FailedAttempt {
    /// Refuses an attempt that cannot be real.
    pub fn new(
        kind: SwingKind,
        attempted: Price,
        attempted_at: DateTime<Utc>,
        previous_run: PriceDistance,
        best: PriceDistance,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Self, CoreError> {
        if from <= attempted_at {
            return Err(CoreError::ImpossibleStructureBreak {
                detail: format!("an attempt at {from} cannot be on an extreme from {attempted_at}"),
            });
        }

        if to < from {
            return Err(CoreError::ImpossibleStructureBreak {
                detail: format!("an attempt cannot end at {to}, before it began at {from}"),
            });
        }

        if best.value() <= Decimal::ZERO {
            return Err(CoreError::ImpossibleStructureBreak {
                detail: "price never went past the extreme, so there was no attempt".into(),
            });
        }

        if previous_run.value() <= Decimal::ZERO {
            return Err(CoreError::ImpossibleStructureBreak {
                detail: "an extreme with no run behind it has nothing to measure against".into(),
            });
        }

        Ok(Self {
            kind,
            attempted,
            attempted_at,
            previous_run,
            best,
            from,
            to,
        })
    }

    pub fn kind(self) -> SwingKind {
        self.kind
    }

    pub fn attempted(self) -> Price {
        self.attempted
    }

    pub fn attempted_at(self) -> DateTime<Utc> {
        self.attempted_at
    }

    pub fn previous_run(self) -> PriceDistance {
        self.previous_run
    }

    /// The furthest past the extreme price got.
    pub fn best(self) -> PriceDistance {
        self.best
    }

    pub fn from(self) -> DateTime<Utc> {
        self.from
    }

    pub fn to(self) -> DateTime<Utc> {
        self.to
    }

    /// How far it got, as a share of the run that made the extreme.
    ///
    /// The number to sort these by later. An attempt that reached 35% and
    /// failed is a near miss; one that reached 5% barely happened.
    pub fn share_of_run(self) -> Option<f64> {
        (self.best.value() / self.previous_run.value()).to_f64()
    }
}
