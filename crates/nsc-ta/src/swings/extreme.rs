//! A price and the candle it happened on, and the arithmetic of a run.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

/// A price, and the candle it happened on.
///
/// **Both halves travel together on purpose.** A price without its candle
/// cannot be turned into a swing, because a swing has to say when it could
/// first have been known.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Extreme {
    pub(super) price: Decimal,
    pub(super) bar_time: DateTime<Utc>,
}

impl Extreme {
    pub(super) fn new(price: Decimal, bar_time: DateTime<Utc>) -> Self {
        Extreme { price, bar_time }
    }
}

/// How far apart two prices are, whichever way round they are.
pub(super) fn span(from: Decimal, to: Decimal) -> Decimal {
    (to - from).abs()
}

/// What share of `whole` is `part`?
///
/// **`None` when the whole is nothing.** A run of nothing cannot be given
/// back, and dividing by it would be the wrong kind of answer rather than an
/// error worth stopping a backtest for.
pub(super) fn share(part: Decimal, whole: Decimal) -> Option<Decimal> {
    (whole > Decimal::ZERO).then(|| part / whole)
}
