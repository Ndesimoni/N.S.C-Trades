//! What you are allowed to add and subtract — and what you are not.
//!
//! Subtracting two prices gives a DISTANCE, not a price. That is the whole
//! point of this module. Once the compiler knows the difference, it stops you
//! writing `stop = entry - atr_multiple` — which compiles fine when everything
//! is `f64`, and silently puts your stop in the wrong place on every trade.
//!
//! ## The missing impl is the feature
//!
//! There is deliberately no `Add<Price> for Price`. Adding two prices together
//! is meaningless, so it must not compile. If you ever find yourself wanting
//! it, what you actually want is a distance.

use std::ops::{Add, Neg, Sub};

use super::distance::PriceDistance;
use super::point::Price;

// ─── Distances combine with each other ─────────────────────────────────────

impl Neg for PriceDistance {
    type Output = PriceDistance;
    fn neg(self) -> PriceDistance {
        PriceDistance(-self.0)
    }
}

impl Add for PriceDistance {
    type Output = PriceDistance;
    fn add(self, rhs: PriceDistance) -> PriceDistance {
        PriceDistance(self.0 + rhs.0)
    }
}

impl Sub for PriceDistance {
    type Output = PriceDistance;
    fn sub(self, rhs: PriceDistance) -> PriceDistance {
        PriceDistance(self.0 - rhs.0)
    }
}

// ─── Prices and distances ──────────────────────────────────────────────────

/// Two prices give you the gap between them.
impl Sub for Price {
    type Output = PriceDistance;
    fn sub(self, rhs: Price) -> PriceDistance {
        PriceDistance(self.0 - rhs.0)
    }
}

/// A price plus a distance is another price — moving up from here.
impl Add<PriceDistance> for Price {
    type Output = Price;
    fn add(self, rhs: PriceDistance) -> Price {
        Price(self.0 + rhs.0)
    }
}

/// A price minus a distance is another price — moving down from here. This is
/// how every stop in the system is placed.
impl Sub<PriceDistance> for Price {
    type Output = Price;
    fn sub(self, rhs: PriceDistance) -> Price {
        Price(self.0 - rhs.0)
    }
}
