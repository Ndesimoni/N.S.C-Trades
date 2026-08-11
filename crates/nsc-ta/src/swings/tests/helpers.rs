//! Building candles to test with.
//!
//! Whole numbers throughout, so the sums can be checked by hand. Flat candles
//! are 10 tall, which means ATR settles at exactly 10 and the noise filter is
//! easy to reason about: a filter of 0.5 needs a swing to stand out by 5.

use chrono::{DateTime, TimeDelta, Utc};
use nsc_core::candle::Candle;
use nsc_core::price::Price;
use rust_decimal::Decimal;

use crate::config::SwingSettings;

pub fn at(index: i64) -> DateTime<Utc> {
    let start = "2026-08-10T00:00:00Z"
        .parse::<DateTime<Utc>>()
        .expect("valid timestamp");

    start + TimeDelta::try_minutes(index * 15).expect("in range")
}

pub fn price(n: i64) -> Price {
    Price::new(Decimal::from(n))
}

/// A candle with the given high and low. Open and close sit in the middle,
/// which keeps the numbers easy to follow.
pub fn candle(index: i64, high: i64, low: i64) -> Candle {
    let middle = (high + low) / 2;

    Candle::new(
        at(index),
        price(middle),
        price(high),
        price(low),
        price(middle),
        None,
        true,
    )
    .expect("valid candle")
}

pub fn settings(lookback: usize, min_atr_multiple: f64) -> SwingSettings {
    SwingSettings {
        lookback,
        require_confirmed: true,
        min_atr_multiple,
    }
}

/// Boring candles, each 10 tall. ATR settles at 10 and none of them is a
/// swing.
pub fn flat(count: i64) -> Vec<Candle> {
    (0..count).map(|i| candle(i, 105, 95)).collect()
}

/// Twenty flat candles to warm ATR up, then `interesting`, then enough flat
/// ones after it for the finder to make up its mind.
pub fn around(interesting: Candle, trailing: i64) -> Vec<Candle> {
    let mut candles = flat(20);
    candles.push(interesting);
    candles.extend((21..21 + trailing).map(|i| candle(i, 105, 95)));
    candles
}
