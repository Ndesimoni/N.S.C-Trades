//! Candles missing from the file.

use nsc_core::timeframe::Timeframe;

use super::super::{Reason, find_holes};
use super::support::*;

#[test]
fn a_history_with_nothing_missing_has_no_holes() {
    let holes = find_holes(&clean(40), Timeframe::M15, &boundary(), 0).expect("valid");

    assert!(holes.is_empty(), "{holes:?}");
}

// The market was open. The candles are not there. That is the broker losing
// data, and every one of these is worth a look.
#[test]
fn candles_missing_mid_week_are_unexplained() {
    let candles = vec![
        moving(thursday(0)),
        moving(thursday(15)),
        // 30, 45 and 60 are gone.
        moving(thursday(75)),
        moving(thursday(90)),
    ];

    let holes = find_holes(&candles, Timeframe::M15, &boundary(), 0).expect("valid");

    assert_eq!(holes.len(), 1);
    assert_eq!(holes[0].reason(), Reason::Unexplained);
}

#[test]
fn a_hole_says_how_many_candles_are_absent() {
    let candles = vec![moving(thursday(0)), moving(thursday(75))];

    let holes = find_holes(&candles, Timeframe::M15, &boundary(), 0).expect("valid");

    assert_eq!(holes[0].missing(), 4, "15, 30, 45 and 60");
    assert_eq!(holes[0].from(), thursday(0));
    assert_eq!(holes[0].to(), thursday(75));
}

// Friday 17:00 New York to Sunday 17:00 New York. The market is shut. Calling
// this a fault would put a hole in every week of every history, and then
// nobody would read the report at all.
#[test]
fn the_weekend_break_is_not_a_fault() {
    let candles = vec![
        moving(utc("2026-08-14T20:45:00Z")),
        moving(utc("2026-08-16T21:00:00Z")),
    ];

    let holes = find_holes(&candles, Timeframe::M15, &boundary(), 0).expect("valid");

    assert_eq!(holes.len(), 1);
    assert_eq!(holes[0].reason(), Reason::Weekend);
}

// Found in the real gold export, not guessed at. XAUUSD 15-minute candles stop
// at 20:45 UTC and start again at 22:00 UTC every single weekday — the metal
// shuts at 17:00 New York and reopens an hour later.
//
// Ten of those in a fortnight. Calling them unexplained would have buried the
// real ones underneath.
#[test]
fn the_nightly_break_gold_takes_is_not_a_fault() {
    let candles = vec![
        moving(utc("2026-08-04T20:45:00Z")),
        moving(utc("2026-08-04T22:00:00Z")),
    ];

    let gold = find_holes(&candles, Timeframe::M15, &boundary(), 60).expect("valid");
    assert_eq!(gold[0].reason(), Reason::DailyBreak);

    // Spot forex never shuts, so the same hole in a USDCAD file is a fault.
    let forex = find_holes(&candles, Timeframe::M15, &boundary(), 0).expect("valid");
    assert_eq!(forex[0].reason(), Reason::Unexplained);
}

// A daily candle is not a fixed number of minutes after the one before it —
// clocks change, and weekends are three days long. Subtracting would report a
// hole at every weekend and miss the real ones.
#[test]
fn a_file_of_daily_candles_is_refused_rather_than_guessed_at() {
    assert!(find_holes(&clean(3), Timeframe::D1, &boundary(), 0).is_err());
}

// ── A broken file, not a hole ──

#[test]
fn two_candles_at_the_same_time_stop_the_scan() {
    let candles = vec![moving(thursday(0)), moving(thursday(0))];

    assert!(find_holes(&candles, Timeframe::M15, &boundary(), 0).is_err());
}

#[test]
fn a_candle_running_backwards_stops_the_scan() {
    let candles = vec![moving(thursday(30)), moving(thursday(15))];

    assert!(find_holes(&candles, Timeframe::M15, &boundary(), 0).is_err());
}
