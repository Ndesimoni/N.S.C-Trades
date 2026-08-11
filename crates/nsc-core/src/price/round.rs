//! Round numbers — the prices people can say out loud.
//!
//! 0.8000. 78.00. 91,000. Traders put orders on them because everyone else can
//! see them too, so they behave like levels without ever having been touched.
//!
//! That is what makes them different from everything in `nsc-ta::levels`. A
//! level there is earned: price had to turn at it, more than once. A round
//! number is there before price arrives and needs no history at all — which is
//! why it lives here, as something a price knows about itself.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::point::Price;
use crate::error::CoreError;

/// The gap between one round number and the next, on one instrument.
///
/// It is not the same everywhere. Sterling steps by 0.0100 — 0.8000, 0.8100,
/// 0.8200. The yen steps by 1.00 — 78.00, 79.00. An instrument trading near
/// ninety thousand steps by 1000.
///
/// Which is why this is a setting per instrument rather than a calculation. As
/// a rough check when adding a new one: a step usually lands near one percent
/// of the price.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RoundStep(Decimal);

impl RoundStep {
    /// Refuses a step of zero or less. A step of zero would make every price
    /// round, and then "price is at a round number" would always be true —
    /// which is the same as the check not being there.
    pub fn new(value: Decimal) -> Result<Self, CoreError> {
        if value <= Decimal::ZERO {
            return Err(CoreError::InvalidRoundStep {
                step: value.to_string(),
            });
        }

        Ok(Self(value))
    }

    pub fn value(self) -> Decimal {
        self.0
    }

    /// The round number at or below this price.
    pub fn below(self, price: Price) -> Price {
        Price::new((price.value() / self.0).floor() * self.0)
    }

    /// The round number above this price. Exactly one step up from
    /// [`below`](Self::below), so a price sitting exactly on a round number
    /// gets the next one up rather than itself.
    pub fn above(self, price: Price) -> Price {
        Price::new(self.below(price).value() + self.0)
    }

    /// The nearer of the two.
    ///
    /// Exactly halfway between them, the one above wins. That case has to be
    /// decided by something, and picking one in writing beats letting it
    /// depend on how the division rounded.
    pub fn nearest(self, price: Price) -> Price {
        let below = self.below(price);
        let above = self.above(price);

        if (price - below).value() < (above - price).value() {
            below
        } else {
            above
        }
    }

    /// Is this price exactly on a round number?
    ///
    /// **Rarely the question you want.** Price almost never lands exactly on
    /// one, and "close enough to count" is measured in normal candles by
    /// whoever is asking. Use [`distance_from`](Self::distance_from) and
    /// compare that against your own tolerance.
    pub fn is_round(self, price: Price) -> bool {
        (price.value() % self.0).is_zero()
    }
}

/// Every step that counts on one instrument, weakest first.
///
/// Round numbers are not all equal. 0.8000 is a stronger number than 0.8800,
/// which is stronger than 0.8050 — the more zeros a price ends in, the more
/// people are watching it and the more orders sit there.
///
/// So an instrument has a ladder rather than one step. Sterling might be:
///
/// ```text
///     0.0050    the halves      0.8050
///     0.0100    the hundreds    0.8800
///     0.1000    the big figure  0.8000
/// ```
///
/// A price sitting on the top rung sits on all the ones below it too, and that
/// is what makes it strong.
///
/// **How strong is worth trading is not decided here.** This reports which
/// rung a number reaches; `nsc-strategy` decides what that is worth, the same
/// way it decides what a level's touch count is worth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RoundLadder(Vec<RoundStep>);

impl RoundLadder {
    /// Refuses an empty ladder, and one whose steps are not strictly
    /// increasing.
    ///
    /// Out of order, "how round is this" would depend on the order someone
    /// happened to type the settings in, and two instruments with the same
    /// steps could score the same price differently.
    pub fn new(steps: Vec<RoundStep>) -> Result<Self, CoreError> {
        let Some(first) = steps.first() else {
            return Err(CoreError::InvalidRoundLadder {
                detail: "a ladder with no steps in it makes no number round".into(),
            });
        };

        let mut previous = *first;
        for step in steps.iter().skip(1) {
            if step.value() <= previous.value() {
                return Err(CoreError::InvalidRoundLadder {
                    detail: format!(
                        "steps must go from smallest to largest, but {} follows {}",
                        step.value(),
                        previous.value()
                    ),
                });
            }
            previous = *step;
        }

        Ok(Self(steps))
    }

    /// The steps, weakest first.
    pub fn steps(&self) -> &[RoundStep] {
        &self.0
    }

    /// How round this price is: how many rungs it sits on.
    ///
    /// Zero means it is not a round number at all. On the sterling ladder
    /// above, 0.8050 scores 1, 0.8800 scores 2, and 0.8000 scores 3.
    pub fn rank(&self, price: Price) -> usize {
        self.0.iter().filter(|step| step.is_round(price)).count()
    }

    /// The nearest round number to this price, and how round it is.
    ///
    /// Measured on the finest rung, because that is the closest candidate. If
    /// what you want is the nearest *strong* number, walk [`steps`](Self::steps)
    /// and ask the rung you care about — a weak number nearby and a strong one
    /// further off are a real choice, and this type does not make it for you.
    pub fn nearest(&self, price: Price) -> Option<(Price, usize)> {
        let finest = self.0.first()?;
        let nearest = finest.nearest(price);

        Some((nearest, self.rank(nearest)))
    }
}

impl Price {
    /// How far this price is from the nearest round number, and which side of
    /// it price sits on.
    ///
    /// Positive means price is above the round number, negative means below,
    /// zero means exactly on it. The sign is kept because approaching 0.8000
    /// from underneath is a different trade from falling towards it.
    ///
    /// The tolerance — how close counts as "at the number" — is not decided
    /// here. It belongs to whoever is asking, in normal candles, the same as
    /// every other distance in this project.
    pub fn distance_from_round(self, step: RoundStep) -> super::distance::PriceDistance {
        self - step.nearest(self)
    }
}
