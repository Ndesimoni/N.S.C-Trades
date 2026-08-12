//! Building charts to test with.
//!
//! Prices are plain integers, so every run and every follow-through can be
//! checked by hand.

use chrono::{DateTime, TimeDelta, Utc};
use nsc_core::candle::Candle;
use nsc_core::price::{Price, PriceDistance};
use rust_decimal::Decimal;

use crate::config::{StructureSettings, SwingSettings};
use nsc_core::structure::{FailedAttempt, StructureBreak, StructureEvent};

use crate::structure::read_structure;
use crate::swings::find_swings;

pub fn at(index: i64) -> DateTime<Utc> {
    let start = "2026-08-10T00:00:00Z"
        .parse::<DateTime<Utc>>()
        .expect("valid timestamp");

    start + TimeDelta::try_minutes(index * 15).expect("in range")
}

pub fn price(n: i64) -> Price {
    Price::new(Decimal::from(n))
}

pub fn distance(n: i64) -> PriceDistance {
    PriceDistance::new(Decimal::from(n))
}

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

pub fn path(prices: &[i64]) -> Vec<Candle> {
    prices
        .iter()
        .enumerate()
        .map(|(index, at_price)| tick(index as i64, *at_price))
        .collect()
}

pub fn swing_settings() -> SwingSettings {
    SwingSettings {
        confirm_retracement: 0.5,
        shallow_retracement: 0.382,
        min_run_fraction: 0.5,
        run_memory_legs: 5,
    }
}

pub fn settings() -> StructureSettings {
    StructureSettings {
        min_follow_through: 0.4,
    }
}

/// Candles in, everything that happened at an old extreme out.
pub fn events_on(prices: &[i64]) -> Vec<StructureEvent> {
    let candles = path(prices);
    let swings = find_swings(&candles, swing_settings()).expect("valid");

    read_structure(&candles, &swings, &settings()).expect("valid")
}

/// Only the extremes price actually took.
pub fn breaks_on(prices: &[i64]) -> Vec<StructureBreak> {
    events_on(prices)
        .into_iter()
        .filter_map(|event| match event {
            StructureEvent::Taken(broken) => Some(broken),
            StructureEvent::Failed(_) => None,
        })
        .collect()
}

/// Only the pushes that gave up.
pub fn failures_on(prices: &[i64]) -> Vec<FailedAttempt> {
    events_on(prices)
        .into_iter()
        .filter_map(|event| match event {
            StructureEvent::Failed(attempt) => Some(attempt),
            StructureEvent::Taken(_) => None,
        })
        .collect()
}
