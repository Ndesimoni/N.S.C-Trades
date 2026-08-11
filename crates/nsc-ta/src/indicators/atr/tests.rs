use chrono::{DateTime, TimeDelta, Utc};
use nsc_core::candle::Candle;
use nsc_core::price::Price;
use rust_decimal::Decimal;

use super::*;
use crate::error::TaError;

fn at(index: i64) -> DateTime<Utc> {
    let start = "2026-08-10T00:00:00Z"
        .parse::<DateTime<Utc>>()
        .expect("valid timestamp");

    start + TimeDelta::try_minutes(index * 15).expect("in range")
}

fn price(n: i64) -> Price {
    Price::new(Decimal::from(n))
}

/// A finished candle. Whole numbers keep the sums easy to check by hand.
fn candle(index: i64, open: i64, high: i64, low: i64, close: i64) -> Candle {
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

/// `count` identical candles, each 10 tall, all closing where they opened.
/// True range is 10 every time, so ATR settles at exactly 10.
fn steady(count: i64) -> Vec<Candle> {
    (0..count).map(|i| candle(i, 100, 110, 100, 100)).collect()
}

#[test]
fn a_period_below_two_is_refused() {
    assert!(matches!(Atr::new(1), Err(TaError::BadSetting { .. })));
    assert!(Atr::new(2).is_ok());
}

#[test]
fn there_is_no_value_until_there_is_enough_history() {
    let mut atr = Atr::new(14).expect("valid period");

    for candle in steady(13) {
        assert_eq!(atr.update(&candle).expect("complete candle"), None);
    }

    assert_eq!(atr.candles_still_needed(), 1);

    let last = candle(13, 100, 110, 100, 100);
    assert!(atr.update(&last).expect("complete candle").is_some());
    assert_eq!(atr.candles_still_needed(), 0);
}

#[test]
fn a_steady_market_settles_on_the_candle_height() {
    let mut atr = Atr::new(14).expect("valid period");

    for candle in steady(20) {
        atr.update(&candle).expect("complete candle");
    }

    // Every candle was 10 tall, so a normal candle is 10.
    assert_eq!(atr.value().map(|v| v.value()), Some(Decimal::from(10)));
}

// ── True range is not just high minus low ──

#[test]
fn a_gap_counts_as_movement() {
    let mut atr = Atr::new(2).expect("valid period");

    // Closes at 105.
    atr.update(&candle(0, 100, 110, 100, 105))
        .expect("complete candle");

    // Opens 45 higher after a weekend. The candle itself is only 7 tall, but
    // price actually moved 50 from where it left off.
    atr.update(&candle(1, 150, 155, 148, 152))
        .expect("complete candle");

    // Seeded from the two true ranges: 10 and 50, so (10 + 50) / 2 = 30.
    // Using high minus low would have given (10 + 7) / 2 = 8.5, and every
    // stop measured against it would be far too tight.
    assert_eq!(atr.value().map(|v| v.value()), Some(Decimal::from(30)));
}

#[test]
fn one_violent_candle_only_moves_the_average_a_little() {
    let mut atr = Atr::new(14).expect("valid period");

    for candle in steady(14) {
        atr.update(&candle).expect("complete candle");
    }
    assert_eq!(atr.value().map(|v| v.value()), Some(Decimal::from(10)));

    // A candle four times the normal size.
    atr.update(&candle(14, 100, 140, 100, 100))
        .expect("complete candle");

    // (10 x 13 + 40) / 14 = 12.14...
    //
    // The point is what did NOT happen. ATR went from 10 to about 12, not to
    // 40. One frightening candle must not convince the system that every
    // candle is now frightening — otherwise every stop widens the moment
    // volatility spikes, which is exactly when you can least afford it.
    let value = atr.value().expect("has a value").value();
    assert!(value > Decimal::from(12), "ATR was {value}");
    assert!(value < Decimal::from(13), "ATR was {value}");
}

// ── The test that matters most ──

#[test]
fn one_at_a_time_matches_all_at_once() {
    let candles = steady(30);

    let all_at_once = atr_series(&candles, 14).expect("valid");

    let mut atr = Atr::new(14).expect("valid period");
    let mut one_at_a_time = Vec::new();
    for candle in &candles {
        one_at_a_time.push(atr.update(candle).expect("complete candle"));
    }

    // The live bot gets candles one at a time. The backtester gets them all
    // at once. If these two ever disagree, backtest results stop describing
    // the bot — and you would not notice, because the gap makes backtests
    // look better rather than broken.
    //
    // Right now they cannot disagree, because atr_series runs the same
    // struct. This test is here so that stays true if someone later decides
    // to "optimise" the bulk path into separate code.
    assert_eq!(all_at_once, one_at_a_time);
}

#[test]
fn an_unfinished_candle_is_refused() {
    let mut atr = Atr::new(2).expect("valid period");

    let forming = Candle::new(
        at(0),
        price(100),
        price(110),
        price(100),
        price(105),
        None,
        false,
    )
    .expect("valid candle");

    assert!(matches!(
        atr.update(&forming),
        Err(TaError::IncompleteCandle { .. })
    ));
}
