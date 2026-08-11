//! Does the finder refuse what it should refuse?
//!
//! These are the tests that matter most. A level drawn from prices the market
//! had not printed yet does not cause an error — it makes the backtest look
//! better than anything you could have traded.

use nsc_core::candle::Candle;
use nsc_core::swing::SwingKind::High;
use nsc_core::timeframe::Timeframe;

use super::helpers::*;
use crate::error::TaError;
use crate::levels::find_levels;
use crate::swings::find_swings;

#[test]
fn a_level_is_never_known_on_the_candle_of_its_last_touch() {
    let candles = chart_with_peaks(&[200, 200]);
    let swings = find_swings(&candles, swing_settings(), 14).expect("valid");

    let levels = find_levels(
        &candles,
        &swings,
        Timeframe::M15,
        &level_settings(2, 500),
        14,
    )
    .expect("valid");

    for level in &levels {
        assert!(
            level.confirmed_at() > level.last_touch(),
            "a touch is a swing, and a swing takes candles to confirm: {level:?}"
        );
        assert!(!level.is_known_at(level.last_touch()));
    }
}

// A swing that has not confirmed yet is one you cannot see. Handing one in
// must not quietly add a touch to a level.
#[test]
fn a_swing_that_has_not_confirmed_yet_is_ignored() {
    let candles = chart_with_peaks(&[200, 200]);
    let mut swings = find_swings(&candles, swing_settings(), 14).expect("valid");

    // Sits at a candle far beyond the end of this chart, so it confirms long
    // after the last candle closed.
    swings.push(swing(High, 500, 201));

    let levels = find_levels(
        &candles,
        &swings,
        Timeframe::M15,
        &level_settings(2, 500),
        14,
    )
    .expect("valid");

    assert_eq!(levels.len(), 1, "got {levels:?}");
    assert_eq!(
        levels[0].touches(),
        2,
        "the third touch had not happened yet"
    );
}

// An unfinished candle's high and low have not finished happening. A level
// built from one is drawn from prices that may never print.
#[test]
fn an_unfinished_last_candle_is_refused() {
    let mut candles = chart_with_peaks(&[200, 200]);
    let swings = find_swings(&candles, swing_settings(), 14).expect("valid");

    let still_forming = Candle::new(
        at(500),
        price(100),
        price(105),
        price(95),
        price(100),
        None,
        false,
    )
    .expect("valid candle");
    candles.push(still_forming);

    let refused = find_levels(
        &candles,
        &swings,
        Timeframe::M15,
        &level_settings(2, 500),
        14,
    );

    assert!(matches!(refused, Err(TaError::IncompleteCandle { .. })));
}

// Without ATR there is no idea how big a normal candle is on this
// instrument, so there is no honest way to decide how thick a band should be.
#[test]
fn nothing_is_found_before_atr_has_warmed_up() {
    let candles: Vec<Candle> = (0..5).map(|i| candle(i, 105, 95)).collect();

    let refused = find_levels(&candles, &[], Timeframe::M15, &level_settings(2, 500), 14);

    assert!(matches!(refused, Err(TaError::NotEnoughCandles { .. })));
}

#[test]
fn an_empty_chart_finds_no_levels() {
    let levels = find_levels(&[], &[], Timeframe::M15, &level_settings(2, 500), 14).expect("valid");

    assert!(levels.is_empty());
}

#[test]
fn a_nonsense_setting_stops_the_run() {
    let candles = chart_with_peaks(&[200, 200]);

    let refused = find_levels(&candles, &[], Timeframe::M15, &level_settings(1, 500), 14);

    assert!(matches!(refused, Err(TaError::BadSetting { .. })));
}
