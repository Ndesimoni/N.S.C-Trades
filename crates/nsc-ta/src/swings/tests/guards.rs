//! Does the finder refuse what it should refuse?
//!
//! These matter more than the detection tests. A swing used before it could
//! have been known does not cause an error — it makes the backtest better than
//! anything you could have traded.

use nsc_core::candle::Candle;

use super::helpers::*;
use crate::error::TaError;
use crate::swings::{SwingFinder, find_swings};

#[test]
fn every_swing_is_confirmed_after_the_candle_it_sits_on() {
    let candles = path(&[100, 200, 300, 250, 200, 150, 100, 160, 220, 300, 400]);
    let swings = find_swings(&candles, settings()).expect("valid");

    assert!(!swings.is_empty(), "nothing to check");

    for swing in &swings {
        assert!(
            swing.confirmed_at() > swing.bar_time(),
            "a swing cannot be known on its own candle: {swing:?}"
        );
        assert!(!swing.is_known_at(swing.bar_time()));
    }
}

// The swings come back as they were learned, which is the order the live bot
// would have had them in.
#[test]
fn swings_arrive_in_the_order_they_were_confirmed() {
    let candles = path(&[100, 200, 300, 250, 200, 150, 100, 160, 220, 300, 400]);
    let swings = find_swings(&candles, settings()).expect("valid");

    for pair in swings.windows(2) {
        assert!(
            pair[0].confirmed_at() <= pair[1].confirmed_at(),
            "{swings:?}"
        );
    }
}

// An unfinished candle's high and low have not happened yet.
#[test]
fn an_unfinished_candle_is_refused() {
    let still_forming = Candle::new(
        at(0),
        price(100),
        price(105),
        price(95),
        price(100),
        None,
        false,
    )
    .expect("valid candle");

    let mut finder = SwingFinder::new(settings()).expect("valid settings");

    assert!(matches!(
        finder.update(&still_forming),
        Err(TaError::IncompleteCandle { .. })
    ));
}

#[test]
fn nonsense_settings_stop_the_run() {
    let mut broken = settings();
    broken.confirm_retracement = 0.0;

    assert!(matches!(
        SwingFinder::new(broken),
        Err(TaError::BadSetting { .. })
    ));
}

#[test]
fn an_empty_history_finds_nothing() {
    let swings = find_swings(&[], settings()).expect("valid");

    assert!(swings.is_empty());
}

// The backtester runs the whole history at once and the live bot takes one
// candle at a time. They must agree, and they do because they are the same
// code.
#[test]
fn one_at_a_time_matches_all_at_once() {
    let candles = path(&[
        100, 200, 300, 250, 200, 150, 100, 160, 220, 300, 400, 300, 200,
    ]);

    let all_at_once = find_swings(&candles, settings()).expect("valid");

    let mut finder = SwingFinder::new(settings()).expect("valid settings");
    let mut one_at_a_time = Vec::new();
    for candle in &candles {
        one_at_a_time.extend(finder.update(candle).expect("valid"));
    }

    assert_eq!(all_at_once, one_at_a_time);
}
