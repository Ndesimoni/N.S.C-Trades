//! Does it call a high taken only when price really went somewhere?

use nsc_core::structure::Trend;
use nsc_core::swing::SwingKind;

use super::helpers::*;
use crate::structure::StructureReader;
use crate::swings::find_swings;

// ── The rule ──

// Up 100 to 300, so the run is 200. Back to 200, which confirms the peak.
// Then up again: crossing 300 is not enough, and 380 is 40% of the run past
// it, which is.
#[test]
fn a_high_is_taken_only_once_price_carries_far_enough_past_it() {
    let breaks = breaks_on(&[100, 200, 300, 250, 200, 260, 320, 380]);

    assert_eq!(breaks.len(), 1, "got {breaks:?}");
    assert_eq!(breaks[0].kind(), SwingKind::High);
    assert_eq!(breaks[0].broken(), price(300));
    assert_eq!(breaks[0].share_of_run(), Some(0.4));
}

// The trap: price crosses the old high and stalls a few points above it.
#[test]
fn a_poke_past_the_high_is_not_a_break() {
    let breaks = breaks_on(&[100, 200, 300, 250, 200, 260, 310, 305, 290]);

    assert!(breaks.is_empty(), "got {breaks:?}");
}

// ── Failed attempts ──

// Not a break, and not nothing. The market tried there and could not hold it,
// which is the "do not take this" side of the training data and cannot be
// collected afterwards.
#[test]
fn a_push_that_gives_up_is_recorded_as_a_failed_attempt() {
    // Crosses 300, gets to 320 — 10% of the 200 run — then back under.
    let failures = failures_on(&[100, 200, 300, 250, 200, 260, 320, 305, 290]);

    assert_eq!(failures.len(), 1, "got {failures:?}");
    assert_eq!(failures[0].kind(), SwingKind::High);
    assert_eq!(failures[0].attempted(), price(300));
    assert_eq!(failures[0].best(), distance(20));
    assert_eq!(failures[0].share_of_run(), Some(0.1));
}

#[test]
fn the_failed_attempt_keeps_the_furthest_it_got() {
    // Wobbles above 300 twice in one push: 320, back to 305, up to 340, then
    // out. The push is one attempt and 40 is the best of it.
    let failures = failures_on(&[100, 200, 300, 250, 200, 260, 320, 305, 340, 290]);

    assert_eq!(failures.len(), 1, "one push, not three: {failures:?}");
    assert_eq!(failures[0].best(), distance(40));
}

// The extreme stays on the books after a failure, so a later push that does
// carry far enough still takes it.
#[test]
fn a_high_can_be_taken_after_a_failed_attempt_at_it() {
    let prices = [100, 200, 300, 250, 200, 260, 320, 290, 340, 380];

    assert_eq!(failures_on(&prices).len(), 1, "the first push gave up");

    let breaks = breaks_on(&prices);
    assert_eq!(breaks.len(), 1, "the second one did not: {breaks:?}");
    assert_eq!(breaks[0].broken(), price(300));
}

// A push can be interrupted by a newer swing forming above the old high
// before price ever comes back under it. The push still failed, and dropping
// the record because the chart moved on would lose exactly the evidence these
// are kept for.
#[test]
fn a_push_still_in_flight_is_recorded_when_a_newer_swing_replaces_the_extreme() {
    // 300 is taken out to 340 — 20% of the 200 run, short of the 40% needed.
    // Price turns there and comes back to 270. On that very candle the swing
    // high at 340 confirms, so 340 replaces 300 as the extreme to watch on the
    // same candle that the push at 300 finally gives up.
    let failures = failures_on(&[100, 200, 300, 250, 200, 250, 340, 305, 270]);

    assert!(
        failures
            .iter()
            .any(|attempt| attempt.attempted() == price(300)),
        "the push at 300 should still be on the record: {failures:?}"
    );
}

// A failure is evidence, not a direction.
#[test]
fn a_failed_attempt_does_not_move_the_trend() {
    let candles = path(&[100, 200, 300, 250, 200, 260, 320, 305, 290]);
    let swings = find_swings(&candles, swing_settings()).expect("valid");

    let mut reader = StructureReader::new(settings()).expect("valid settings");
    let mut next = 0;

    for candle in &candles {
        let from = next;
        while swings
            .get(next)
            .is_some_and(|swing| swing.confirmed_at() <= candle.open_time())
        {
            next += 1;
        }
        reader.update(candle, &swings[from..next]).expect("valid");
    }

    assert_eq!(reader.trend(), Trend::Unclear);
}

// A cross that stalls is not thrown away. If price comes back and carries far
// enough later, the break completes then — the test is about how far price
// got, not how quickly.
#[test]
fn a_stalled_cross_can_still_complete_later() {
    let breaks = breaks_on(&[100, 200, 300, 250, 200, 260, 310, 305, 340, 390]);

    assert_eq!(breaks.len(), 1, "got {breaks:?}");
    assert_eq!(breaks[0].broken(), price(300));
}

#[test]
fn lows_break_the_same_way_mirrored() {
    let breaks = breaks_on(&[300, 200, 100, 150, 200, 140, 80, 20]);

    assert_eq!(breaks.len(), 1, "got {breaks:?}");
    assert_eq!(breaks[0].kind(), SwingKind::Low);
    assert_eq!(breaks[0].broken(), price(100));
}

#[test]
fn how_far_past_it_carried_is_kept() {
    // 300 taken out by 100, on a run of 200 — half as much again as the
    // minimum, and the rules layer is entitled to know that.
    let breaks = breaks_on(&[100, 200, 300, 250, 200, 260, 320, 400]);

    assert_eq!(breaks[0].share_of_run(), Some(0.5));
}

// ── Trend ──

#[test]
fn the_trend_is_unclear_until_something_is_taken() {
    let mut reader = StructureReader::new(settings()).expect("valid settings");

    assert_eq!(reader.trend(), Trend::Unclear);

    for candle in path(&[100, 200, 300, 250, 200]) {
        reader.update(&candle, &[]).expect("valid");
    }

    assert_eq!(reader.trend(), Trend::Unclear, "no swings were fed in");
}

#[test]
fn taking_out_a_high_turns_the_trend_up() {
    let candles = path(&[100, 200, 300, 250, 200, 260, 320, 380]);
    let swings = find_swings(&candles, swing_settings()).expect("valid");

    let mut reader = StructureReader::new(settings()).expect("valid settings");
    let mut next = 0;

    for candle in &candles {
        let from = next;
        while swings
            .get(next)
            .is_some_and(|swing| swing.confirmed_at() <= candle.open_time())
        {
            next += 1;
        }
        reader.update(candle, &swings[from..next]).expect("valid");
    }

    assert_eq!(reader.trend(), Trend::Up);
}
