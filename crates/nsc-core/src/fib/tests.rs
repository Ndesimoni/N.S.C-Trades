use chrono::{DateTime, TimeDelta, Utc};
use rust_decimal::Decimal;

use super::*;
use crate::error::CoreError;
use crate::price::Price;
use crate::swing::{Swing, SwingKind};

fn at(index: i64) -> DateTime<Utc> {
    let start = "2026-08-12T00:00:00Z"
        .parse::<DateTime<Utc>>()
        .expect("valid timestamp");

    start + TimeDelta::try_hours(index).expect("in range")
}

fn price(n: i64) -> Price {
    Price::new(Decimal::from(n))
}

fn swing(kind: SwingKind, index: i64, at_price: i64) -> Swing {
    Swing::new(kind, at(index), at(index + 2), price(at_price)).expect("valid swing")
}

/// A move up from 100 to 200. A hundred points, so every share is easy.
fn a_move_up() -> FibRetracement {
    FibRetracement::between(
        swing(SwingKind::Low, 1, 100),
        swing(SwingKind::High, 5, 200),
    )
    .expect("valid move")
}

#[test]
fn the_golden_zone_sits_where_it_should_on_a_move_up() {
    let move_up = a_move_up();

    assert_eq!(move_up.level(0.5).expect("valid"), price(150));
    assert_eq!(
        move_up.level(0.618).expect("valid"),
        Price::new(Decimal::new(1382, 1))
    );
    assert!(move_up.is_up());
}

#[test]
fn a_share_of_zero_is_the_end_of_the_move_and_one_is_the_start() {
    let move_up = a_move_up();

    assert_eq!(move_up.level(0.0).expect("valid"), price(200));
    assert_eq!(move_up.level(1.0).expect("valid"), price(100));
}

// A move down measures the same way, which is what stops a downtrend being
// read by different arithmetic from an uptrend.
#[test]
fn a_move_down_measures_the_same_way() {
    let move_down = FibRetracement::between(
        swing(SwingKind::High, 1, 200),
        swing(SwingKind::Low, 5, 100),
    )
    .expect("valid move");

    assert!(!move_down.is_up());
    assert_eq!(move_down.level(0.5).expect("valid"), price(150));
    assert_eq!(move_down.depth_at(price(150)), Some(0.5));
}

#[test]
fn depth_says_how_far_price_has_come_back() {
    let move_up = a_move_up();

    assert_eq!(move_up.depth_at(price(200)), Some(0.0), "at the extreme");
    assert_eq!(move_up.depth_at(price(150)), Some(0.5), "half way back");
    assert_eq!(move_up.depth_at(price(100)), Some(1.0), "all the way back");
}

// Past the start of the move, the number keeps going. It says the move was
// undone and then some, which is true and worth knowing.
#[test]
fn depth_past_the_start_of_the_move_keeps_counting() {
    assert_eq!(a_move_up().depth_at(price(50)), Some(1.5));
}

// ── What gets refused ──

#[test]
fn a_move_that_went_nowhere_is_refused() {
    let refused = FibRetracement::between(
        swing(SwingKind::Low, 1, 100),
        swing(SwingKind::High, 5, 100),
    );

    assert!(matches!(
        refused,
        Err(CoreError::ImpossibleRetracement { .. })
    ));
}

#[test]
fn a_move_that_ends_before_it_starts_is_refused() {
    let refused = FibRetracement::between(
        swing(SwingKind::Low, 5, 100),
        swing(SwingKind::High, 1, 200),
    );

    assert!(matches!(
        refused,
        Err(CoreError::ImpossibleRetracement { .. })
    ));
}

// Drawing levels off a move whose second swing had not confirmed is drawing
// them off a move that had not happened.
#[test]
fn a_move_cannot_be_used_before_both_ends_confirmed() {
    let move_up = a_move_up();

    // The later swing sits at hour 5 and confirms at hour 7.
    assert!(!move_up.is_known_at(at(6)));
    assert!(move_up.is_known_at(at(7)));
}
