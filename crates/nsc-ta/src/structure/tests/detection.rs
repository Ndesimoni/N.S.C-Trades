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
