//! A point on the chart.

use std::fmt;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// A price, exactly as the market printed it. 1.0850, or 4320.66.
///
/// Backed by `Decimal`, not `f64`, because prices get compared against levels
/// and stored in the database. In floating point `0.1 + 0.2` is not `0.3`, so
/// a level check that should say "price touched it" says "missed by a
/// billionth" instead.
///
/// You cannot add two prices together. See `ops.rs` for why that is the
/// point rather than an oversight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Price(pub(super) Decimal);

impl Price {
    pub fn new(value: Decimal) -> Self {
        Self(value)
    }

    pub fn value(self) -> Decimal {
        self.0
    }

    /// Rounds for showing to a human. **Display only.**
    ///
    /// Never round before comparing a price to a level. That is how a level
    /// check starts quietly lying to you.
    pub fn round_for_display(self, digits: u32) -> Decimal {
        self.0.round_dp(digits)
    }
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
