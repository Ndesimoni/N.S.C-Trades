//! Does it refuse what it should refuse?

use nsc_core::candle::Candle;
use nsc_core::pattern::CandleShape;
use nsc_core::price::PriceDistance;
use rust_decimal::Decimal;

use super::helpers::*;
use crate::candles::{find_patterns, look_at};
use crate::error::TaError;

// An unfinished candle's shape has not finished happening. Reading one means
// naming a pattern off prices that may never print.
#[test]
fn an_unfinished_candle_is_refused() {
    let still_forming = Candle::new(
        at(0),
        price(80),
        price(100),
        price(0),
        price(90),
        None,
        false,
    )
    .expect("valid candle");

    assert!(matches!(
        look_at(&[still_forming], atr(), &settings()),
        Err(TaError::IncompleteCandle { .. })
    ));
}

#[test]
fn an_empty_window_finds_nothing() {
    assert!(look_at(&[], atr(), &settings()).expect("valid").is_empty());
}

// A candle whose high and low are the same price has no shape at all, and
// every share of it would be a division by zero.
#[test]
fn a_candle_with_no_height_produces_nothing() {
    assert!(seen(&[candle(0, 50, 50, 50, 50)]).is_empty());
}

// One candle alone still works. It simply cannot make a two-candle shape,
// because there is nothing for it to engulf.
#[test]
fn a_single_candle_makes_no_two_candle_shapes() {
    let shapes: Vec<_> = seen(&[candle(0, 45, 100, 30, 70)])
        .iter()
        .map(|s| s.shape())
        .collect();

    assert!(!shapes.contains(&CandleShape::Engulfing));
    assert!(!shapes.contains(&CandleShape::Tweezers));
}

// Without ATR there is no idea how big a normal candle is, so "a long candle"
// and "near enough the same price" cannot be answered. Better to find nothing
// than to guess.
#[test]
fn the_shapes_that_need_a_yardstick_go_quiet_without_one() {
    let flat = PriceDistance::new(Decimal::ZERO);
    let window = [candle(0, 20, 100, 10, 90), candle(1, 90, 98, 20, 30)];

    let shapes: Vec<_> = look_at(&window, flat, &settings())
        .expect("valid")
        .iter()
        .map(|s| s.shape())
        .collect();

    assert!(!shapes.contains(&CandleShape::BeltHold), "got {shapes:?}");
    assert!(!shapes.contains(&CandleShape::Tweezers), "got {shapes:?}");
}

// Every sighting is dated to the candle it completed on, which is the first
// moment it could have been acted on.
#[test]
fn a_two_candle_shape_is_dated_to_the_second_candle() {
    let down = candle(0, 60, 70, 40, 50);
    let up = candle(1, 45, 100, 30, 70);

    let seen = found(&[down, up], CandleShape::Engulfing).expect("engulfing");

    assert_eq!(seen.at(), at(1));
}

// Judging a quiet week by this week's volatility would find belt-holds that
// were not there, so ATR is taken as it was at the time.
#[test]
fn a_history_finds_nothing_before_atr_has_warmed_up() {
    let candles: Vec<_> = (0..5).map(|i| candle(i, 45, 100, 30, 70)).collect();

    let found = find_patterns(&candles, &settings(), 14).expect("valid");

    assert!(found.is_empty(), "got {found:?}");
}
