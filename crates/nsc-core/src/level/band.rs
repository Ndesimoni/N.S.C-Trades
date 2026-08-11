//! The strip of price a level covers.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::error::CoreError;
use crate::price::{Price, PriceDistance};

/// A bottom and a top, with the level somewhere inside.
///
/// Every band found on a timeframe is the same thickness. It gets slid up and
/// down to catch the most touches; it never gets stretched to reach one more.
///
/// That is how these get drawn by hand, and it matters: a band that stretches
/// to swallow whatever is near it ends up wide enough to contain half the
/// chart, and then every price is at every level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Band {
    low: Price,
    high: Price,
}

impl Band {
    /// Builds a band from its bottom and top.
    pub fn new(low: Price, high: Price) -> Result<Self, CoreError> {
        if high < low {
            return Err(CoreError::ImpossibleLevel {
                detail: format!("the top {high} is below the bottom {low}"),
            });
        }

        Ok(Self { low, high })
    }

    /// A band of the given thickness, sitting centred on a price.
    ///
    /// This is the one the level finder uses. It picks where the middle goes;
    /// the thickness is decided once, for the whole timeframe.
    pub fn around(centre: Price, thickness: PriceDistance) -> Result<Self, CoreError> {
        if thickness.value() < Decimal::ZERO {
            return Err(CoreError::ImpossibleLevel {
                detail: format!("a band cannot be {} thick", thickness.value()),
            });
        }

        let half = PriceDistance::new(thickness.value() / Decimal::from(2));

        Ok(Self {
            low: centre - half,
            high: centre + half,
        })
    }

    pub fn low(self) -> Price {
        self.low
    }

    pub fn high(self) -> Price {
        self.high
    }

    /// Bottom to top.
    pub fn thickness(self) -> PriceDistance {
        self.high - self.low
    }

    /// The middle of the band. What you would call "the level" if someone
    /// made you name one number.
    pub fn centre(self) -> Price {
        self.low + PriceDistance::new(self.thickness().value() / Decimal::from(2))
    }

    /// Is this price inside the band? The edges count as inside.
    pub fn contains(self, price: Price) -> bool {
        price >= self.low && price <= self.high
    }

    /// How far this price is from the band, and which side it is on.
    ///
    /// Positive means price is above the band, negative means below, and zero
    /// means inside it. The sign is kept because which side price sits on is
    /// what turns one level into support or resistance today.
    ///
    /// Measured to the nearest edge, not to the middle. A wick that reaches
    /// into the band has arrived, so its distance is zero.
    pub fn distance_to(self, price: Price) -> PriceDistance {
        if price > self.high {
            price - self.high
        } else if price < self.low {
            price - self.low
        } else {
            PriceDistance::new(Decimal::ZERO)
        }
    }
}
