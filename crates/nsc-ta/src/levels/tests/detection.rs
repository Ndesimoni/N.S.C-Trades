//! Does the finder draw the levels you would draw?

use nsc_core::timeframe::Timeframe;

use super::helpers::*;
use crate::levels::find_levels;
use crate::swings::find_swings;

/// Runs the whole job: candles in, levels out.
fn levels_on(
    peaks: &[i64],
    min_touches: usize,
    max_age_bars: usize,
) -> Vec<nsc_core::level::Level> {
    let candles = chart_with_peaks(peaks);
    let swings = find_swings(&candles, swing_settings(), 14).expect("valid");

    find_levels(
        &candles,
        &swings,
        Timeframe::M15,
        &level_settings(min_touches, max_age_bars),
        14,
    )
    .expect("valid")
}

#[test]
fn two_peaks_at_the_same_price_make_a_level() {
    let levels = levels_on(&[200, 200], 2, 500);

    assert_eq!(levels.len(), 1, "got {levels:?}");
    assert_eq!(levels[0].touches(), 2);
    assert_eq!(levels[0].centre(), price(200));
    assert!(levels[0].contains(price(200)));
    assert_eq!(levels[0].timeframe(), Timeframe::M15);
}

#[test]
fn peaks_that_are_nearly_the_same_price_make_one_level() {
    // Three peaks within a couple of points of each other. By eye that is one
    // line drawn through all three, not three lines.
    let levels = levels_on(&[200, 202, 204], 2, 500);

    assert_eq!(levels.len(), 1, "got {levels:?}");
    assert_eq!(levels[0].touches(), 3);
}

#[test]
fn a_price_touched_once_is_not_a_level() {
    // Two peaks, nowhere near each other. Each has been visited once, and one
    // visit proves nothing.
    let levels = levels_on(&[200, 400], 2, 500);

    assert!(levels.is_empty(), "got {levels:?}");
}

#[test]
fn separate_prices_make_separate_levels_lowest_first() {
    let levels = levels_on(&[400, 400, 200, 200], 2, 500);

    assert_eq!(levels.len(), 2, "got {levels:?}");
    assert_eq!(levels[0].centre(), price(200));
    assert_eq!(levels[1].centre(), price(400));
}

#[test]
fn the_touch_dates_are_the_candles_the_touches_sit_on() {
    let levels = levels_on(&[200, 200], 2, 500);

    assert_eq!(levels[0].first_touch(), at(peak_index(0)));
    assert_eq!(levels[0].last_touch(), at(peak_index(1)));
}

// A price that turned the market long ago and has not been near it since is
// history, not a level you would draw today.
#[test]
fn touches_older_than_the_lookback_window_stop_counting() {
    let recent = levels_on(&[200, 200, 200], 2, 500);
    assert_eq!(recent[0].touches(), 3);

    // The same chart, looking back only ten candles. Two of the three touches
    // fall off the left of that window, and one touch is not a level.
    let forgetful = levels_on(&[200, 200, 200], 2, 10);
    assert!(forgetful.is_empty(), "got {forgetful:?}");
}

#[test]
fn asking_for_more_touches_finds_fewer_levels() {
    assert_eq!(levels_on(&[200, 200], 2, 500).len(), 1);
    assert!(levels_on(&[200, 200], 3, 500).is_empty());
}
