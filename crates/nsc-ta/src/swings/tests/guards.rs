//! Does the finder refuse what it should refuse?

use nsc_core::candle::Candle;

use super::helpers::*;
use crate::error::TaError;
use crate::swings::{SwingFinder, find_swings};

// ── The test that matters most ──

#[test]
fn one_at_a_time_matches_all_at_once() {
    let mut candles = flat(20);
    candles.push(candle(20, 200, 95));
    candles.extend((21..30).map(|i| candle(i, 105, 95)));
    candles.push(candle(30, 60, 10));
    candles.extend((31..40).map(|i| candle(i, 105, 95)));

    let all_at_once = find_swings(&candles, settings(3, 0.5), 14).expect("valid");

    let mut finder = SwingFinder::new(settings(3, 0.5), 14).expect("valid");
    let mut one_at_a_time = Vec::new();
    for candle in &candles {
        one_at_a_time.extend(finder.update(candle).expect("valid"));
    }

    // The live bot gets candles one at a time. The backtester gets the lot.
    // If these ever disagree, backtest results stop describing the bot — and
    // you would not notice, because that kind of mismatch makes backtests
    // look better rather than broken.
    assert_eq!(all_at_once, one_at_a_time);
    assert!(!all_at_once.is_empty(), "the test needs to find something");
}

#[test]
fn an_unfinished_candle_is_refused() {
    let mut finder = SwingFinder::new(settings(3, 0.5), 14).expect("valid");

    let forming = Candle::new(
        at(0),
        price(100),
        price(110),
        price(90),
        price(100),
        None,
        false,
    )
    .expect("valid candle");

    assert!(matches!(
        finder.update(&forming),
        Err(TaError::IncompleteCandle { .. })
    ));
}

#[test]
fn a_lookback_of_zero_is_refused() {
    assert!(matches!(
        SwingFinder::new(settings(0, 0.5), 14),
        Err(TaError::BadSetting { .. })
    ));
}

#[test]
fn every_swing_is_confirmed_after_the_candle_it_sits_on() {
    let mut candles = flat(20);
    candles.push(candle(20, 200, 95));
    candles.extend((21..30).map(|i| candle(i, 105, 95)));
    candles.push(candle(30, 60, 10));
    candles.extend((31..40).map(|i| candle(i, 105, 95)));

    let swings = find_swings(&candles, settings(3, 0.5), 14).expect("valid");
    assert!(!swings.is_empty());

    // Belt and braces. Swing::new already refuses this, but a lookahead bug
    // has no other symptom — it does not crash, it does not warn, it just
    // makes results better. Worth checking twice.
    for swing in &swings {
        assert!(
            swing.confirmed_at() > swing.bar_time(),
            "swing at {:?} claims to be known at {:?}",
            swing.bar_time(),
            swing.confirmed_at()
        );
    }
}
