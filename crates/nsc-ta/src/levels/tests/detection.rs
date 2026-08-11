//! Does the finder draw the levels you would draw?

use nsc_core::level::Level;
use nsc_core::timeframe::Timeframe;

use super::helpers::*;
use crate::levels::find_levels;
use crate::swings::find_swings;

/// Runs the whole job: candles in, levels out.
fn levels_on(turns: &[i64], min_touches: usize, max_age_bars: usize) -> Vec<Level> {
    let candles = zigzag(turns);
    let swings = find_swings(&candles, swing_settings()).expect("valid");

    find_levels(
        &candles,
        &swings,
        Timeframe::M15,
        &level_settings(min_touches, max_age_bars),
        ATR_PERIOD,
    )
    .expect("valid")
}

#[test]
fn a_price_turned_at_twice_becomes_a_level() {
    let levels = levels_on(&[100, 200, 100, 200, 100], 2, 500);

    let level = level_at(&levels, 200).expect("a level at 200");

    assert_eq!(level.touches(), 2);
    assert_eq!(level.centre(), price(200));
    assert_eq!(level.timeframe(), Timeframe::M15);
}

#[test]
fn turns_at_nearly_the_same_price_make_one_level() {
    // Three peaks within four points of each other. By eye that is one line
    // through all three, not three lines.
    let levels = levels_on(&[100, 200, 100, 202, 100, 204, 100], 2, 500);

    let level = level_at(&levels, 202).expect("a level at the peaks");

    assert_eq!(level.touches(), 3);
}

#[test]
fn a_price_turned_at_once_is_not_a_level() {
    // Two peaks, nowhere near each other. Each has been visited once, and one
    // visit proves nothing.
    let levels = levels_on(&[100, 200, 100, 300, 100], 2, 500);

    assert!(level_at(&levels, 200).is_none(), "got {levels:?}");
    assert!(level_at(&levels, 300).is_none(), "got {levels:?}");
}

// The tops and the bottoms are both levels, and they come back in price order.
#[test]
fn separate_prices_make_separate_levels_lowest_first() {
    let levels = levels_on(&[100, 200, 100, 200, 100, 200, 100], 2, 500);

    assert_eq!(levels.len(), 2, "got {levels:?}");
    assert_eq!(levels[0].centre(), price(100));
    assert_eq!(levels[1].centre(), price(200));
}

#[test]
fn the_touch_dates_are_the_candles_the_touches_sit_on() {
    let levels = levels_on(&[100, 200, 100, 200, 100], 2, 500);
    let level = level_at(&levels, 200).expect("a level at 200");

    assert_eq!(level.first_touch(), at(turn_index(1)));
    assert_eq!(level.last_touch(), at(turn_index(3)));
}

// A price that turned the market long ago and has not been near it since is
// history, not a level you would draw today.
#[test]
fn touches_older_than_the_lookback_window_stop_counting() {
    let remembered = levels_on(&[100, 200, 100, 200, 100], 2, 500);
    assert!(level_at(&remembered, 200).is_some());

    // The same chart, looking back only four candles. The first peak falls off
    // the left of that window, and one touch is not a level.
    let forgetful = levels_on(&[100, 200, 100, 200, 100], 2, 4);
    assert!(level_at(&forgetful, 200).is_none(), "got {forgetful:?}");
}

#[test]
fn asking_for_more_touches_finds_fewer_levels() {
    let turns = [100, 200, 100, 200, 100];

    assert!(level_at(&levels_on(&turns, 2, 500), 200).is_some());
    assert!(level_at(&levels_on(&turns, 3, 500), 200).is_none());
}
