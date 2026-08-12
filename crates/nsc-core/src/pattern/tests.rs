use chrono::{DateTime, TimeDelta, Utc};
use rust_decimal::Decimal;

use super::*;
use crate::candle::Candle;
use crate::error::CoreError;
use crate::price::Price;

fn at(index: i64) -> DateTime<Utc> {
    let start = "2026-08-12T00:00:00Z"
        .parse::<DateTime<Utc>>()
        .expect("valid timestamp");

    start + TimeDelta::try_minutes(index * 15).expect("in range")
}

fn price(n: i64) -> Price {
    Price::new(Decimal::from(n))
}

fn candle(open: i64, high: i64, low: i64, close: i64) -> Candle {
    Candle::new(
        at(0),
        price(open),
        price(high),
        price(low),
        price(close),
        None,
        true,
    )
    .expect("valid candle")
}

// ── Proportions ──

#[test]
fn a_candle_divides_into_body_and_two_wicks() {
    // Range 100. Body 20 in the middle, 40 above, 40 below.
    let shape = candle(40, 100, 0, 60).proportions().expect("has height");

    assert_eq!(shape.body(), 0.2);
    assert_eq!(shape.upper_wick(), 0.4);
    assert_eq!(shape.lower_wick(), 0.4);
}

#[test]
fn a_hammer_shape_has_its_tail_underneath() {
    // Small body at the top, long wick below.
    let shape = candle(80, 100, 0, 90).proportions().expect("has height");

    assert!(shape.tail_points_down());
    assert_eq!(shape.tail_to_body(), Some(8.0), "80 of wick on 10 of body");
}

// A candle with no height at all is not a shape, and every share of it would
// be a division by zero.
#[test]
fn a_candle_with_no_height_has_no_proportions() {
    assert!(candle(50, 50, 50, 50).proportions().is_none());
}

#[test]
fn a_body_of_nothing_has_no_wick_to_body_ratio() {
    let shape = candle(50, 100, 0, 50).proportions().expect("has height");

    assert_eq!(shape.body(), 0.0);
    assert_eq!(shape.tail_to_body(), None, "infinity is not an answer");
}

#[test]
fn direction_comes_from_the_body() {
    assert!(candle(40, 100, 0, 60).is_up());
    assert!(candle(60, 100, 0, 40).is_down());

    let flat = candle(50, 100, 0, 50);
    assert!(!flat.is_up(), "closing where it opened is neither");
    assert!(!flat.is_down());
}

// ── Sightings ──

#[test]
fn a_sighting_carries_the_measurements_that_made_it() {
    let shape = candle(80, 100, 0, 90).proportions().expect("has height");
    let seen = PatternSighting::new(CandleShape::PinBar, Bias::Bullish, at(3), 1, shape)
        .expect("valid sighting");

    assert_eq!(seen.shape(), CandleShape::PinBar);
    assert!(seen.bias().is_bullish());
    assert_eq!(seen.spans(), 1);
    assert_eq!(seen.proportions().tail_to_body(), Some(8.0));
}

#[test]
fn a_pattern_made_of_no_candles_is_refused() {
    let shape = candle(40, 100, 0, 60).proportions().expect("has height");
    let refused = PatternSighting::new(CandleShape::Engulfing, Bias::Bullish, at(3), 0, shape);

    assert!(matches!(refused, Err(CoreError::ImpossiblePattern { .. })));
}

#[test]
fn a_doji_points_nowhere_on_its_own() {
    assert!(!Bias::Neutral.is_bullish());
    assert!(!Bias::Neutral.is_bearish());
}
