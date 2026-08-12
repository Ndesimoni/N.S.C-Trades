//! Building candles to test shapes with.
//!
//! Every candle here runs between 0 and 100, so a share of its height is a
//! percentage and every threshold can be checked in your head.

use chrono::{DateTime, TimeDelta, Utc};
use nsc_core::candle::Candle;
use nsc_core::pattern::{CandleShape, PatternSighting};
use nsc_core::price::{Price, PriceDistance};
use rust_decimal::Decimal;

use crate::config::CandleSettings;

pub fn at(index: i64) -> DateTime<Utc> {
    let start = "2026-08-12T00:00:00Z"
        .parse::<DateTime<Utc>>()
        .expect("valid timestamp");

    start + TimeDelta::try_minutes(index * 15).expect("in range")
}

pub fn price(n: i64) -> Price {
    Price::new(Decimal::from(n))
}

/// A normal candle is 100 tall, so ATR of 100 makes every test candle exactly
/// one normal candle.
pub fn atr() -> PriceDistance {
    PriceDistance::new(Decimal::from(100))
}

pub fn candle(index: i64, open: i64, high: i64, low: i64, close: i64) -> Candle {
    Candle::new(
        at(index),
        price(open),
        price(high),
        price(low),
        price(close),
        None,
        true,
    )
    .expect("valid candle")
}

/// The values in config/ta.toml today — the textbook ones.
pub fn settings() -> CandleSettings {
    CandleSettings {
        pin_min_tail_to_body: 2.0,
        pin_max_body_share: 0.33,
        pin_max_nose_share: 0.25,
        engulfing_min_first_body_share: 0.1,
        doji_max_body_share: 0.05,
        doji_max_missing_wick_share: 0.05,
        belt_hold_max_open_wick_share: 0.05,
        belt_hold_min_body_share: 0.6,
        belt_hold_min_atr_multiple: 1.0,
        tweezer_tolerance_atr: 0.05,
    }
}

/// Everything found on the last candle of the window.
pub fn seen(window: &[Candle]) -> Vec<PatternSighting> {
    crate::candles::look_at(window, atr(), &settings()).expect("valid")
}

/// Was this shape among them?
pub fn found(window: &[Candle], shape: CandleShape) -> Option<PatternSighting> {
    seen(window).into_iter().find(|s| s.shape() == shape)
}
