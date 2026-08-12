//! One move, and the two questions you ask of it.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

use crate::error::CoreError;
use crate::price::Price;
use crate::swing::Swing;

/// A move worth measuring a retracement over.
///
/// It holds the move rather than a list of prices, because the prices are
/// only ever a share of it. Storing the levels instead would lose the one
/// thing worth arguing about.
///
/// Two questions get asked of it, and everything else is built from them:
///
///   - **where is a given share?** — 0.618 of this move is what price
///   - **how deep is price now?** — as a share of the move
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FibRetracement {
    /// Where the move started — the swing it came off.
    from: Price,

    /// Where it got to. The extreme, and the point a retracement comes back
    /// from.
    to: Price,

    from_at: DateTime<Utc>,
    to_at: DateTime<Utc>,

    /// The first moment this move could have been drawn.
    ///
    /// A move is only knowable once both ends are confirmed swings, which is
    /// always later than the candle the second one sits on.
    known_at: DateTime<Utc>,
}

impl FibRetracement {
    /// Builds a move from the two swings that make it.
    ///
    /// Refuses a move of no size — nothing can be a share of nothing — and one
    /// whose ends are the wrong way round in time.
    pub fn between(from: Swing, to: Swing) -> Result<Self, CoreError> {
        if to.bar_time() <= from.bar_time() {
            return Err(CoreError::ImpossibleRetracement {
                detail: format!(
                    "a move cannot end at {} and start at {}",
                    to.bar_time(),
                    from.bar_time()
                ),
            });
        }

        if from.price() == to.price() {
            return Err(CoreError::ImpossibleRetracement {
                detail: "a move that went nowhere has no levels in it".into(),
            });
        }

        Ok(Self {
            from: from.price(),
            to: to.price(),
            from_at: from.bar_time(),
            to_at: to.bar_time(),
            // Both ends must be known, and the later swing is the later of
            // the two confirmations by construction — but taking the maximum
            // says so rather than assuming it.
            known_at: from.confirmed_at().max(to.confirmed_at()),
        })
    }

    pub fn from(self) -> Price {
        self.from
    }

    pub fn to(self) -> Price {
        self.to
    }

    pub fn from_at(self) -> DateTime<Utc> {
        self.from_at
    }

    pub fn to_at(self) -> DateTime<Utc> {
        self.to_at
    }

    pub fn known_at(self) -> DateTime<Utc> {
        self.known_at
    }

    /// Could this move have been drawn at `now`?
    ///
    /// **Ask before using it.** Drawing levels off a move whose second swing
    /// had not confirmed is drawing them off a move that had not happened.
    pub fn is_known_at(self, now: DateTime<Utc>) -> bool {
        now >= self.known_at
    }

    /// Was the move upward?
    pub fn is_up(self) -> bool {
        self.to > self.from
    }

    /// The price at a given share back from the end of the move.
    ///
    /// 0.0 is the end of the move, 1.0 is where it started. So on a move up
    /// from 100 to 200, 0.618 is 138 — most of the way back down.
    pub fn level(self, share: f64) -> Result<Price, CoreError> {
        let share = Decimal::from_f64(share).ok_or(CoreError::NotRepresentable { value: share })?;

        Ok(Price::new(
            self.to.value() - (self.to.value() - self.from.value()) * share,
        ))
    }

    /// How far price has come back, as a share of the move.
    ///
    /// Zero at the end of the move, one back at its start. Above one means
    /// price has gone past where the move began, and the number is still
    /// honest — it says the move was undone and then some.
    pub fn depth_at(self, price: Price) -> Option<f64> {
        let size = self.to.value() - self.from.value();

        if size.is_zero() {
            return None;
        }

        ((self.to.value() - price.value()) / size).to_f64()
    }
}
