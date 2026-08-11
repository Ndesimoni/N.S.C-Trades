//! A run, and how much of it has been given back.

use chrono::{DateTime, Utc};
use nsc_core::price::{Price, PriceDistance};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

/// A price, and the candle it happened on.
///
/// Both halves travel together on purpose. A price without its candle cannot
/// be turned into a swing, because a swing has to say when it could first have
/// been known.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Extreme {
    pub price: Price,
    pub bar_time: DateTime<Utc>,
}

impl Extreme {
    pub fn new(price: Price, bar_time: DateTime<Utc>) -> Self {
        Self { price, bar_time }
    }
}

/// How far apart two prices are, ignoring which way round they are.
pub(super) fn span(from: Price, to: Price) -> PriceDistance {
    (to - from).abs()
}

/// What share of `whole` is `part`?
///
/// `None` when the whole is zero — a run of nothing cannot be given back, and
/// dividing by it would be the wrong kind of answer rather than an error worth
/// stopping a backtest for.
pub(super) fn share(part: PriceDistance, whole: PriceDistance) -> Option<f64> {
    if whole.value() <= Decimal::ZERO {
        return None;
    }

    (part.value() / whole.value()).to_f64()
}
