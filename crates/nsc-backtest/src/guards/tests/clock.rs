//! The mistake that was made here first: judging by the stamp on a thing
//! instead of the clock time it became true.

use nsc_core::timeframe::Timeframe;
use nsc_data::events::BarClosed;

use super::super::Guard;
use super::support::*;

// ── The clock, not the stamp ──

// This is the bug the second read found, and this test is what pins the fix.
//
// A 4-hour candle running 21:00 to 01:00 is stamped 21:00. A swing it confirms
// carries that same 21:00. But nobody knew what that candle would do until
// 01:00.
//
// Compare stamps and the swing looks knowable from 21:00 — four hours early.
#[test]
fn a_four_hour_swing_is_not_knowable_until_its_candle_closes() {
    // A peak at 17:00, confirmed by the 4-hour candle that opens at 21:00 and
    // closes at 01:00.
    let peak = swing(-240, 0);

    // Two hours in, that candle has not closed. Its stamp says 21:00 and the
    // clock says 23:00, so a stamp comparison would wave it through.
    caught(guard(120).swing(peak, Timeframe::H4));

    // At 01:00 it has closed.
    assert!(guard(240).swing(peak, Timeframe::H4).is_ok());
}

// The other half of the same mistake. Standing at the close of a 4-hour bar,
// a 15-minute swing from inside that same four hours HAS happened. Comparing
// stamps would throw it out.
#[test]
fn a_fresh_smaller_swing_survives_a_bigger_bars_guard() {
    let symbol = gold();
    // The 4-hour bar covering 21:00 to 01:00, stamped 21:00.
    let bar = BarClosed::new(symbol, Timeframe::H4, candle(0, true)).expect("valid");
    let guard = Guard::at(&bar, boundary()).expect("valid");

    assert_eq!(guard.now(), at(240), "the guard stands at the bar's close");

    // A 15-minute swing confirmed by the candle opening 00:45, which closed at
    // 01:00. It happened. It must not be thrown out.
    assert!(guard.swing(swing(180, 225), Timeframe::M15).is_ok());
}
