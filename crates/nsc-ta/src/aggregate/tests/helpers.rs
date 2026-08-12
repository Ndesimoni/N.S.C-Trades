//! Building candles to aggregate.

use chrono::{DateTime, TimeDelta, Utc, Weekday};
use chrono_tz::America::New_York;
use nsc_core::candle::Candle;
use nsc_core::price::Price;
use nsc_core::timeframe::DayBoundary;
use rust_decimal::Decimal;

/// The daily close this project uses: 5pm New York, weeks starting Sunday.
pub fn boundary() -> DayBoundary {
    DayBoundary::new(17, 0, New_York, Weekday::Sun).expect("a real time of day")
}

/// A summer Monday, well after the 5pm New York close — so 21:00 UTC.
pub fn start() -> DateTime<Utc> {
    "2026-08-10T21:00:00Z"
        .parse::<DateTime<Utc>>()
        .expect("valid timestamp")
}

pub fn price(n: i64) -> Price {
    Price::new(Decimal::from(n))
}

/// One 15-minute candle, `nth` after the daily close.
pub fn candle(nth: i64, open: i64, high: i64, low: i64, close: i64) -> Candle {
    Candle::new(
        start() + TimeDelta::try_minutes(nth * 15).expect("in range"),
        price(open),
        price(high),
        price(low),
        price(close),
        None,
        true,
    )
    .expect("valid candle")
}

/// `count` quiet candles, each one point higher than the last.
pub fn drift(count: i64) -> Vec<Candle> {
    (0..count)
        .map(|nth| candle(nth, 100 + nth, 105 + nth, 95 + nth, 101 + nth))
        .collect()
}
