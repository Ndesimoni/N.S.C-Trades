//! What the guard lets through and what it kills the run for.

use nsc_core::level::{Band, Level};
use nsc_core::timeframe::Timeframe;

use super::support::*;

// ── Swings ──

#[test]
fn a_swing_confirmed_long_ago_gets_through() {
    assert!(guard(240).swing(swing(60, 120), Timeframe::M15).is_ok());
}

// The candle opening at 03:45 closes at 04:00. Standing at 04:00, it has just
// finished — so what it confirmed is knowable, and not a minute earlier. One
// candle either side of this boundary is the difference between a real
// backtest and a flattering one.
#[test]
fn a_swing_confirmed_by_the_candle_that_just_closed_gets_through() {
    assert!(guard(240).swing(swing(60, 225), Timeframe::M15).is_ok());
}

#[test]
fn a_swing_confirmed_by_the_candle_still_forming_kills_the_run() {
    caught(guard(240).swing(swing(60, 240), Timeframe::M15));
}

// ── Levels ──

#[test]
fn a_level_confirmed_later_kills_the_run() {
    caught(guard(240).level(level(Timeframe::M15, 300, 360)));
}

// A level he drew today knows what price did last year. Using it on last
// year's candles would make the backtest look better than anything tradeable.
#[test]
fn a_hand_drawn_level_does_not_exist_before_he_drew_it() {
    let band = Band::new(p(4340), p(4360)).expect("valid band");
    let drawn = Level::drawn_by_hand(band, Timeframe::W1, at(10_080));

    caught(guard(240).level(drawn));
    assert!(guard(30_000).level(drawn).is_ok());
}

// ── Candles ──

#[test]
fn a_candle_that_has_not_closed_yet_kills_the_run() {
    caught(guard(240).candle(&candle(480, true), Timeframe::M15));
}

// Its high and low have not happened yet. Reading them is reading the future,
// even though the timestamp looks like the past.
#[test]
fn a_candle_still_marked_as_forming_kills_the_run() {
    caught(guard(240).candle(&candle(225, false), Timeframe::M15));
}

#[test]
fn the_candle_that_just_closed_gets_through() {
    assert!(
        guard(240)
            .candle(&candle(225, true), Timeframe::M15)
            .is_ok()
    );
}

// ── Lists ──

#[test]
fn one_bad_swing_in_a_good_list_kills_the_run() {
    let swings = [swing(0, 60), swing(60, 120), swing(120, 480)];

    caught(guard(240).swings(&swings, Timeframe::M15));
}

#[test]
fn a_list_of_knowable_swings_comes_back_whole() {
    let swings = [swing(0, 60), swing(60, 120), swing(120, 225)];

    assert_eq!(
        guard(240)
            .swings(&swings, Timeframe::M15)
            .expect("all knowable")
            .len(),
        3
    );
}

// Levels come from every timeframe at once, so each is judged on its own.
#[test]
fn a_mixed_list_of_levels_is_judged_one_by_one() {
    let fine = level(Timeframe::M15, 60, 120);
    let early = level(Timeframe::H4, 60, 120);

    // Both were confirmed at 23:00, so both stamps are in the past. It is
    // 00:59. The 15-minute one closed at 23:15. The 4-hour one does not close
    // until 01:00 — one minute away, and still the future.
    assert!(guard(239).level(fine).is_ok());
    caught(guard(239).levels(&[fine, early]));
}

// ── The message has to be worth reading ──

// A run dies weeks after it was written, in a log somebody else is reading.
// "lookahead detected" sends them hunting. The two times and what was touched
// point straight at the line.
#[test]
fn the_message_says_what_was_touched_and_when_it_became_knowable() {
    let message = caught(guard(240).swing(swing(60, 240), Timeframe::M15)).to_string();

    assert!(message.contains("swing high"), "{message}");
    assert!(message.contains("4350"), "{message}");
    assert!(
        message.contains("01:00:00 UTC"),
        "now is missing: {message}"
    );
    assert!(
        message.contains("01:15:00 UTC"),
        "knowable_at is missing: {message}"
    );
}
