//! Candles the two scans are run over.

use chrono::{DateTime, TimeDelta, Utc, Weekday};
use chrono_tz::America::New_York;
use nsc_core::candle::Candle;
use nsc_core::price::Price;
use nsc_core::timeframe::DayBoundary;
use rust_decimal::Decimal;

pub(super) fn boundary() -> DayBoundary {
    DayBoundary::new(17, 0, New_York, Weekday::Sun).expect("17:00 is a real time")
}

pub(super) fn p(n: i64) -> Price {
    Price::new(Decimal::from(n))
}

/// Thursday 13 August 2026, 21:00 UTC — 17:00 in New York, so the start of a
/// trading day, mid-week.
pub(super) fn thursday(minutes: i64) -> DateTime<Utc> {
    "2026-08-13T21:00:00Z"
        .parse::<DateTime<Utc>>()
        .expect("valid timestamp")
        + TimeDelta::try_minutes(minutes).expect("in range")
}

pub(super) fn utc(text: &str) -> DateTime<Utc> {
    text.parse::<DateTime<Utc>>().expect("valid timestamp")
}

/// A candle that moved: high above low.
pub(super) fn moving(open_time: DateTime<Utc>) -> Candle {
    Candle::new(open_time, p(4300), p(4310), p(4290), p(4305), None, true).expect("valid candle")
}

/// A candle that never moved: one price, no range at all.
///
/// Not missing data. The market was open and nothing traded, and the broker
/// printed this correctly.
pub(super) fn flat(open_time: DateTime<Utc>, price: i64) -> Candle {
    Candle::new(
        open_time,
        p(price),
        p(price),
        p(price),
        p(price),
        None,
        true,
    )
    .expect("valid candle")
}

/// `count` candles, 15 minutes apart, all moving.
pub(super) fn clean(count: i64) -> Vec<Candle> {
    (0..count).map(|i| moving(thursday(i * 15))).collect()
}

/// Flattens a stretch of a history to one price, leaving the times alone.
pub(super) fn flatten(candles: &mut [Candle], from: usize, to: usize, price: i64) {
    for (offset, candle) in candles[from..to].iter_mut().enumerate() {
        *candle = flat(thursday((from + offset) as i64 * 15), price);
    }
}
