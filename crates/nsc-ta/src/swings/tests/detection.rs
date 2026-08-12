//! Does the finder find the swings the rule describes?

use nsc_core::swing::{Swing, SwingKind};

use super::helpers::*;
use crate::swings::find_swings;

fn swings_on(prices: &[i64]) -> Vec<Swing> {
    find_swings(&path(prices), settings()).expect("valid")
}

// ── The depth route ──

#[test]
fn giving_back_half_the_run_confirms_the_peak() {
    // Up 100 to 300, a run of 200. Then back to 200, which is half of it.
    let swings = swings_on(&[100, 200, 300, 250, 200]);

    assert_eq!(swings.len(), 1, "got {swings:?}");
    assert_eq!(swings[0].kind(), SwingKind::High);
    assert_eq!(swings[0].price(), price(300));
}

#[test]
fn giving_back_less_than_half_confirms_nothing() {
    // The same run of 200, but only 60 given back.
    let swings = swings_on(&[100, 200, 300, 260, 240]);

    assert!(swings.is_empty(), "got {swings:?}");
}

#[test]
fn the_peak_is_dated_to_its_own_candle_not_the_one_that_proved_it() {
    let swings = swings_on(&[100, 200, 300, 250, 200]);

    assert_eq!(swings[0].bar_time(), at(2), "the peak is candle 2");
    assert!(
        swings[0].confirmed_at() > at(2),
        "and it could not have been known there"
    );
}

// ── The shallow route ──

// A strong trend barely pauses. Waiting for half would miss the structure in
// exactly the market worth trading.
#[test]
fn a_shallow_pause_counts_once_price_takes_the_peak_out() {
    // Up 100 to 300. Back to 220 — that is 80 of 200, so 40%: past the
    // shallow share but short of half. Then up through 300.
    let swings = swings_on(&[100, 200, 300, 260, 220, 280, 340]);

    assert_eq!(swings.len(), 2, "the peak and the bottom of the pause");
    assert_eq!(swings[0].kind(), SwingKind::High);
    assert_eq!(swings[0].price(), price(300));
    assert_eq!(swings[1].kind(), SwingKind::Low);
    assert_eq!(swings[1].price(), price(220));
}

#[test]
fn a_pause_too_shallow_for_either_route_is_not_a_swing() {
    // Only 40 given back out of 200 — a fifth. Price then runs on, and the
    // pause was never structure.
    let swings = swings_on(&[100, 200, 300, 280, 260, 320, 400]);

    assert!(swings.is_empty(), "got {swings:?}");
}

// ── Alternating ──

#[test]
fn swings_alternate_high_low_high() {
    let swings = swings_on(&[
        100, 200, 300, // up
        200, 100, // down through half
        200, 300, 400, // up again
        300, 200, // and back
    ]);

    let kinds: Vec<_> = swings.iter().map(|s| s.kind()).collect();

    assert!(kinds.len() >= 2, "got {swings:?}");
    for pair in kinds.windows(2) {
        assert_ne!(pair[0], pair[1], "two of the same in a row: {kinds:?}");
    }
}

#[test]
fn a_fall_confirms_a_low_the_same_way() {
    // Down 300 to 100, a run of 200, then back up 100 — half of it.
    let swings = swings_on(&[300, 200, 100, 150, 200]);

    assert_eq!(swings.len(), 1, "got {swings:?}");
    assert_eq!(swings[0].kind(), SwingKind::Low);
    assert_eq!(swings[0].price(), price(100));
}

// ── The run floor ──

#[test]
fn a_wobble_after_a_big_run_is_not_a_swing() {
    // A run of 200 confirms. Then a 20-point wiggle, which is nowhere near
    // half of it, so nothing else counts however cleanly it turns.
    let swings = swings_on(&[
        100, 200, 300, 200, 100, // a real run, down through half
        120, 110, 120, 110, 120,
    ]);

    assert_eq!(swings.len(), 1, "only the real one: {swings:?}");
}

// Each leg is 60% of the one before. Measured against the last one alone they
// would all pass and the chart would fill with noise.
#[test]
fn the_ratchet_stops() {
    let swings = swings_on(&[
        1000, 3000, // 2000 up
        1800, // back 1200, over half — confirms
        3000, // 1200 up
        2280, // back 720, over half — confirms
        2712, // 432 up, which is under half of 2000
        2453, 2600, 2445,
    ]);

    assert!(
        swings.len() <= 3,
        "the shrinking legs should stop counting: {swings:?}"
    );
}

// A crash straight through the start of the run gives back more than all of
// it, so the peak it left behind is certainly a swing. The finder used to
// throw the whole leg away instead, because it noticed the wreckage before it
// asked what had been proved.
#[test]
fn a_peak_still_counts_when_price_crashes_past_where_the_run_began() {
    let swings = swings_on(&[100, 200, 300, 90]);

    assert_eq!(swings.len(), 1, "got {swings:?}");
    assert_eq!(swings[0].kind(), SwingKind::High);
    assert_eq!(swings[0].price(), price(300));
}

// One candle's own height is not a run. Without that, the whole of it gets
// given back inside the next candle and every share test passes — so a dead
// flat market would open with a swing.
#[test]
fn a_flat_start_produces_no_swings() {
    let candles: Vec<_> = (0..10).map(|i| candle(i, 105, 95)).collect();

    let swings = find_swings(&candles, settings()).expect("valid");

    assert!(swings.is_empty(), "got {swings:?}");
}

// ── Wicks ──

#[test]
fn the_swing_sits_on_the_wick() {
    let mut candles = vec![tick(0, 100), tick(1, 200)];
    // A candle whose body is at 250 but whose wick reaches 300.
    candles.push(candle(2, 300, 240));
    candles.extend([tick(3, 250), tick(4, 200)]);

    let swings = find_swings(&candles, settings()).expect("valid");

    assert_eq!(swings[0].price(), price(300), "the wick, not the body");
}
