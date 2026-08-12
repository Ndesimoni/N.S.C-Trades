//! Does it measure the right move, and put the levels in the right places?

use chrono::{DateTime, TimeDelta, Utc};
use nsc_core::price::Price;
use nsc_core::swing::{Swing, SwingKind};
use rust_decimal::Decimal;

use super::{FibReading, last_move};
use crate::config::FibSettings;

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

fn settings() -> FibSettings {
    FibSettings {
        golden_zone: [0.5, 0.618],
        strong_trend_level: 0.382,
        stop_level: 0.786,
        extensions: [1.272, 1.618],
    }
}

/// A low at 100, then a high at 200. A hundred-point move, so every share is
/// a round number.
fn a_leg_up() -> Vec<Swing> {
    vec![
        swing(SwingKind::High, 1, 150),
        swing(SwingKind::Low, 3, 100),
        swing(SwingKind::High, 7, 200),
    ]
}

// ── Picking the move ──

#[test]
fn the_move_measured_is_the_last_completed_leg() {
    let measured = last_move(&a_leg_up(), at(20))
        .expect("valid")
        .expect("a move");

    assert_eq!(measured.from(), price(100), "the low it came off");
    assert_eq!(measured.to(), price(200), "the high it reached");
    assert!(measured.is_up());
}

#[test]
fn a_history_with_only_one_swing_measures_nothing() {
    let one = vec![swing(SwingKind::Low, 3, 100)];

    assert!(last_move(&one, at(20)).expect("valid").is_none());
}

// A move drawn from a swing that had not confirmed is a move that had not
// happened.
#[test]
fn swings_that_have_not_confirmed_yet_are_ignored() {
    let swings = a_leg_up();

    // The last swing sits at hour 7 and confirms at hour 9. Before then, the
    // move on offer is the one before it.
    let earlier = last_move(&swings, at(8)).expect("valid").expect("a move");

    assert_eq!(earlier.to(), price(100), "the leg down, not the leg up");
}

#[test]
fn nothing_is_measured_before_two_swings_have_confirmed() {
    assert!(last_move(&a_leg_up(), at(2)).expect("valid").is_none());
}

// ── The levels ──

#[test]
fn the_four_levels_land_where_they_should() {
    let measured = last_move(&a_leg_up(), at(20))
        .expect("valid")
        .expect("a move");
    let reading = FibReading::take(measured, price(150), &settings())
        .expect("valid")
        .expect("a reading");

    assert_eq!(reading.strong_trend(), Price::new(Decimal::new(1618, 1)));
    assert_eq!(reading.golden_from(), price(150));
    assert_eq!(reading.golden_to(), Price::new(Decimal::new(1382, 1)));
    assert_eq!(reading.stop(), Price::new(Decimal::new(1214, 1)));
}

#[test]
fn depth_says_how_far_price_has_come_back() {
    let measured = last_move(&a_leg_up(), at(20))
        .expect("valid")
        .expect("a move");

    let shallow = FibReading::take(measured, price(180), &settings())
        .expect("valid")
        .expect("a reading");

    assert_eq!(shallow.depth(), 0.2);
    assert!(!shallow.in_golden_zone(&settings()));
}

#[test]
fn price_between_the_two_golden_levels_is_in_the_zone() {
    let measured = last_move(&a_leg_up(), at(20))
        .expect("valid")
        .expect("a move");

    for back_to in [150, 145, 140] {
        let reading = FibReading::take(measured, price(back_to), &settings())
            .expect("valid")
            .expect("a reading");

        assert!(
            reading.in_golden_zone(&settings()),
            "{back_to} should be in the zone at depth {}",
            reading.depth()
        );
    }
}

// A zone that excludes its own boundary is a zone price misses by a tick.
#[test]
fn the_edges_of_the_zone_count_as_inside_it() {
    let measured = last_move(&a_leg_up(), at(20))
        .expect("valid")
        .expect("a move");

    let shallow_edge = FibReading::take(measured, price(150), &settings())
        .expect("valid")
        .expect("a reading");

    assert!(shallow_edge.in_golden_zone(&settings()));
}

#[test]
fn price_past_the_stop_level_says_so() {
    let measured = last_move(&a_leg_up(), at(20))
        .expect("valid")
        .expect("a move");
    let deep = FibReading::take(measured, price(110), &settings())
        .expect("valid")
        .expect("a reading");

    assert!(deep.past_the_stop(&settings()));
    assert!(!deep.in_golden_zone(&settings()));
}

// ── Settings that make no sense ──

#[test]
fn a_strong_trend_level_at_or_past_the_zone_is_refused() {
    let mut broken = settings();
    broken.strong_trend_level = 0.6;

    assert!(broken.validate().is_err(), "it would say nothing new");
}

// A stop inside the zone would be hit by the entry it is supposed to protect.
#[test]
fn a_stop_level_inside_the_zone_is_refused() {
    let mut broken = settings();
    broken.stop_level = 0.55;

    assert!(broken.validate().is_err());
}

#[test]
fn a_golden_zone_the_wrong_way_round_is_refused() {
    let mut broken = settings();
    broken.golden_zone = [0.618, 0.5];

    assert!(broken.validate().is_err());
}

// Cannot happen with the current finder, which alternates. It would be silent
// nonsense if it ever did — two highs are not a leg — so it is refused rather
// than measured, and the refusal is pinned here.
#[test]
fn two_swings_the_same_way_round_are_not_a_leg() {
    let both_highs = vec![
        swing(SwingKind::High, 1, 150),
        swing(SwingKind::High, 5, 200),
    ];

    assert!(last_move(&both_highs, at(20)).expect("valid").is_none());
}
