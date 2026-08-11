//! The three ways of measuring a gap between two prices.
//!
//! The same gap on EURUSD can be described as 0.0050, or 50 pips, or 1.8
//! normal candles. All three are the same distance. They are separate types
//! so that one cannot be quietly used where another was meant.
//!
//! Neither conversion is a plain `From`, because both need outside
//! information: pips need the instrument's pip size, ATR multiples need the
//! current ATR. So they are functions that take what they need.

use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

use crate::error::CoreError;

/// The distance between two prices, in the instrument's own units.
///
/// Signed on purpose: `a - b` tells you direction as well as size. Use
/// `abs()` when you only care how far.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PriceDistance(pub(super) Decimal);

impl PriceDistance {
    pub fn new(value: Decimal) -> Self {
        Self(value)
    }

    pub fn value(self) -> Decimal {
        self.0
    }

    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    pub fn is_zero(self) -> bool {
        self.0.is_zero()
    }

    /// Counts this distance in pips. Needs the instrument, because a pip is
    /// 0.0001 on EURUSD and 0.01 on USDJPY.
    pub fn to_pips(self, pip_size: Decimal) -> Result<Pips, CoreError> {
        if pip_size.is_zero() {
            return Err(CoreError::ZeroPipSize);
        }
        Ok(Pips(self.0 / pip_size))
    }

    /// Counts this distance in normal candles.
    ///
    /// This is the one that matters. Nearly every threshold in this system is
    /// an ATR multiple rather than a pip count, because a pip setting that
    /// works on EURUSD is meaningless on gold.
    pub fn to_atr_multiple(self, atr: PriceDistance) -> Result<AtrMultiple, CoreError> {
        if atr.0 <= Decimal::ZERO {
            return Err(CoreError::ZeroAtr);
        }
        (self.0 / atr.0)
            .to_f64()
            .map(AtrMultiple)
            .ok_or(CoreError::RatioNotRepresentable)
    }
}

/// A distance in pips. For showing to a human, and for spread limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Pips(pub(super) Decimal);

impl Pips {
    pub fn new(value: Decimal) -> Self {
        Self(value)
    }

    pub fn value(self) -> Decimal {
        self.0
    }

    /// Back to a real distance. Cannot fail — multiplying always works.
    pub fn to_distance(self, pip_size: Decimal) -> PriceDistance {
        PriceDistance(self.0 * pip_size)
    }
}

/// A distance measured in normal candles. 1.5 means "one and a half times the
/// size of an ordinary candle right now".
///
/// `f64` rather than `Decimal` because this is a ratio, not money. It gets
/// multiplied, never compared for exact equality, and never stored as a price.
///
/// Only `PartialOrd`, not `Ord` — `f64` has NaN, so a total ordering would be
/// a lie.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AtrMultiple(pub(super) f64);

impl AtrMultiple {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(self) -> f64 {
        self.0
    }

    /// Turns "0.3 of a normal candle" into an actual distance you can add to
    /// a price — which is how every stop buffer in this system works.
    pub fn to_distance(self, atr: PriceDistance) -> Result<PriceDistance, CoreError> {
        let multiple =
            Decimal::from_f64(self.0).ok_or(CoreError::NotRepresentable { value: self.0 })?;
        Ok(PriceDistance(multiple * atr.0))
    }
}
