use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use super::*;
use crate::error::CoreError;
use crate::price::Price;

fn at(text: &str) -> DateTime<Utc> {
    text.parse::<DateTime<Utc>>().expect("valid timestamp")
}

/// Four whole numbers at four decimal places: 10850 means 1.0850.
fn candle(open: i64, high: i64, low: i64, close: i64) -> Result<Candle, CoreError> {
    let p = |n: i64| Price::new(Decimal::new(n, 4));

    Candle::new(
        at("2026-08-10T14:30:00Z"),
        p(open),
        p(high),
        p(low),
        p(close),
        None,
        true,
    )
}

#[test]
fn a_normal_candle_is_accepted() {
    let c = candle(10820, 10860, 10810, 10850).expect("this candle is fine");

    assert_eq!(c.open_time(), at("2026-08-10T14:30:00Z"));
    assert!(c.is_complete());
    assert_eq!(c.volume(), None);
}

#[test]
fn a_high_below_the_low_is_refused() {
    let err = candle(10820, 10840, 10850, 10830).expect_err("high is under the low");

    match err {
        CoreError::ImpossibleCandle { detail, .. } => {
            // The message has to name both numbers, or it is useless at 3am.
            assert!(detail.contains("1.0840"), "detail was: {detail}");
            assert!(detail.contains("1.0850"), "detail was: {detail}");
        }
        other => panic!("wrong error: {other}"),
    }
}

#[test]
fn an_open_outside_the_range_is_refused() {
    let err = candle(10900, 10860, 10810, 10850).expect_err("open is above the high");
    assert!(matches!(err, CoreError::ImpossibleCandle { .. }));
}

#[test]
fn a_close_outside_the_range_is_refused() {
    let err = candle(10820, 10860, 10810, 10800).expect_err("close is below the low");
    assert!(matches!(err, CoreError::ImpossibleCandle { .. }));
}

// ── The one that stops someone "helpfully" adding a positivity check ──

#[test]
fn negative_prices_are_allowed() {
    let p = |n: i64| Price::new(Decimal::new(n, 2));

    // WTI crude, 20 April 2020. Producers paid people to take oil away.
    let c = Candle::new(
        at("2020-04-20T18:00:00Z"),
        p(1022),  //  10.22
        p(1050),  //  10.50
        p(-3763), // -37.63
        p(-3712), // -37.12
        None,
        true,
    );

    assert!(c.is_ok(), "a real week of oil history must not be refused");
}

#[test]
fn range_is_the_full_height() {
    let c = candle(10820, 10860, 10810, 10850).expect("valid");

    // 1.0860 - 1.0810
    assert_eq!(c.range().value(), Decimal::new(50, 4));
}

#[test]
fn body_keeps_its_sign() {
    let up = candle(10820, 10860, 10810, 10850).expect("valid");
    let down = candle(10850, 10860, 10810, 10820).expect("valid");

    assert_eq!(up.body().value(), Decimal::new(30, 4)); //  0.0030
    assert_eq!(down.body().value(), Decimal::new(-30, 4)); // -0.0030

    // Callers who only want the size ask for it.
    assert_eq!(up.body().abs(), down.body().abs());
}

#[test]
fn an_unfinished_candle_says_so() {
    let p = |n: i64| Price::new(Decimal::new(n, 4));

    let forming = Candle::new(
        at("2026-08-10T14:30:00Z"),
        p(10820),
        p(10860),
        p(10810),
        p(10850),
        None,
        false,
    )
    .expect("valid");

    assert!(!forming.is_complete());
}
