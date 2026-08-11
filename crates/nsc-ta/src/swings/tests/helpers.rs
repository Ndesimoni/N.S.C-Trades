//! Building charts to test with.
//!
//! Whole numbers throughout, so every run and every give-back can be checked
//! by hand. Prices are plain integers: a run from 100 to 300 is a run of 200,
//! and half of it is 100.

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

/// A candle that simply sits at one price. Its high, low, open and close are
/// all the same, so a chart built from these is a plain line and the only
/// thing being tested is the rule.
pub fn tick(index: i64, at_price: i64) -> Candle {
    Candle::new(
        at(index),
        price(at_price),
        price(at_price),
        price(at_price),
        price(at_price),
        None,
        true,
    )
    .expect("valid candle")
}

/// A candle with a real high and low, for the cases where that matters.
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

/// One candle per price, in order.
pub fn path(prices: &[i64]) -> Vec<Candle> {
    prices
        .iter()
        .enumerate()
        .map(|(index, at_price)| tick(index as i64, *at_price))
        .collect()
}

/// The settings in config/ta.toml today.
pub fn settings() -> SwingSettings {
    SwingSettings {
        confirm_retracement: 0.5,
        shallow_retracement: 0.382,
        min_run_fraction: 0.5,
        run_memory_legs: 5,
    }
}
