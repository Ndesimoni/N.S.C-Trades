//! The shape of a candle, as shares of its own height.

use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};

use super::bar::Candle;

/// How a candle divides up: body, wick above, wick below.
///
/// Shares of the candle's own height, so they add up to one. That is what
/// makes them the right yardstick for shape: a body that is a fifth of its
/// candle is a fifth on EURUSD and a fifth on gold, with no ATR and no pip
/// size involved.
///
/// Size is a different question — whether a candle is big or small at all is
/// measured in ATR, like everything else in this project.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Proportions {
    body: f64,
    upper_wick: f64,
    lower_wick: f64,
}

impl Proportions {
    /// The share taken by the body, open to close.
    pub fn body(self) -> f64 {
        self.body
    }

    /// The share above the body.
    pub fn upper_wick(self) -> f64 {
        self.upper_wick
    }

    /// The share below the body.
    pub fn lower_wick(self) -> f64 {
        self.lower_wick
    }

    /// How many times the body the longer wick is.
    ///
    /// `None` when the body is nothing at all — the answer would be infinity,
    /// and a caller that wants "a very long wick on a very small body" should
    /// be checking the body share instead.
    pub fn tail_to_body(self) -> Option<f64> {
        if self.body <= 0.0 {
            return None;
        }

        Some(self.longer_wick() / self.body)
    }

    pub fn longer_wick(self) -> f64 {
        self.upper_wick.max(self.lower_wick)
    }

    pub fn shorter_wick(self) -> f64 {
        self.upper_wick.min(self.lower_wick)
    }

    /// Is the long wick underneath? That is the shape of a hammer.
    pub fn tail_points_down(self) -> bool {
        self.lower_wick > self.upper_wick
    }
}

impl Candle {
    /// How this candle divides up.
    ///
    /// `None` for a candle with no height at all — one where the high and the
    /// low are the same price. It happens on thin instruments in quiet hours,
    /// it is not a shape, and every share of it would be a division by zero.
    pub fn proportions(&self) -> Option<Proportions> {
        let range = self.range().value();

        if range <= Decimal::ZERO {
            return None;
        }

        let top_of_body = self.open().max(self.close());
        let bottom_of_body = self.close().min(self.open());

        let body = (self.body().abs().value() / range).to_f64()?;
        let upper_wick = ((self.high() - top_of_body).value() / range).to_f64()?;
        let lower_wick = ((bottom_of_body - self.low()).value() / range).to_f64()?;

        Some(Proportions {
            body,
            upper_wick,
            lower_wick,
        })
    }

    /// Did it close higher than it opened?
    pub fn is_up(&self) -> bool {
        self.body().value() > Decimal::ZERO
    }

    /// Did it close lower than it opened?
    ///
    /// A candle that closed exactly where it opened is neither, which is why
    /// this is not the opposite of [`is_up`](Self::is_up).
    pub fn is_down(&self) -> bool {
        self.body().value() < Decimal::ZERO
    }
}
