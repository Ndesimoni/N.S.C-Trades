//! Candles that are there but never moved.

use super::super::find_flat_runs;
use super::support::*;

#[test]
fn a_history_that_moves_has_no_flat_runs() {
    assert!(find_flat_runs(&clean(40), 3).is_empty());
}

#[test]
fn flat_candles_in_a_row_become_one_run() {
    let mut candles = clean(10);
    flatten(&mut candles, 3, 8, 4300);

    let runs = find_flat_runs(&candles, 3);

    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].candles(), 5);
    assert_eq!(runs[0].from(), thursday(45));
    assert_eq!(runs[0].to(), thursday(105));
    assert_eq!(runs[0].price(), p(4300));
}

// One flat quarter of an hour on a quiet Sunday evening is not news. Reporting
// it would bury the twenty-in-a-row that is.
#[test]
fn a_short_run_below_the_threshold_is_not_reported() {
    let mut candles = clean(10);
    flatten(&mut candles, 4, 5, 4300);

    assert!(find_flat_runs(&candles, 3).is_empty());
    assert_eq!(
        find_flat_runs(&candles, 1).len(),
        1,
        "asked for all of them"
    );
}

// The worst case is a feed that froze and never came back. If a run is only
// closed when a moving candle turns up, that one is never reported at all.
#[test]
fn a_run_still_going_when_the_candles_end_is_still_reported() {
    let mut candles = clean(10);
    flatten(&mut candles, 6, 10, 4300);

    let runs = find_flat_runs(&candles, 3);

    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].candles(), 4);
    assert_eq!(runs[0].to(), thursday(135), "the last candle in the file");
}

#[test]
fn one_moving_candle_between_two_shelves_keeps_them_apart() {
    let mut candles = clean(20);
    flatten(&mut candles, 2, 6, 4300);
    flatten(&mut candles, 10, 15, 4400);

    let runs = find_flat_runs(&candles, 3);

    assert_eq!(runs.len(), 2);
    assert_eq!(runs[0].price(), p(4300));
    assert_eq!(runs[1].price(), p(4400));
}

// Both candles have no range of their own, but price moved between them. One
// run reported at one price would be a number nobody could have measured.
#[test]
fn two_shelves_touching_at_different_prices_stay_two_shelves() {
    let mut candles = clean(20);
    flatten(&mut candles, 2, 6, 4300);
    flatten(&mut candles, 6, 10, 4400);

    let runs = find_flat_runs(&candles, 3);

    assert_eq!(runs.len(), 2, "one shelf per price");
    assert_eq!(runs[0].price(), p(4300));
    assert_eq!(runs[1].price(), p(4400));
    assert_eq!(runs[1].from(), thursday(90));
}

// The pair the analysis actually trips over. Two flat candles at the left edge
// of a history invented a swing on every instrument, and the tests were green.
#[test]
fn the_two_flat_candles_at_the_left_edge_are_found() {
    let mut candles = clean(10);
    flatten(&mut candles, 0, 2, 4300);

    let runs = find_flat_runs(&candles, 2);

    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].from(), thursday(0));
    assert_eq!(runs[0].candles(), 2);
}
