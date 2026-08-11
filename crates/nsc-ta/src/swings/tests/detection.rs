//! Does the finder find the right swings?

use nsc_core::swing::SwingKind;

use super::helpers::*;
use crate::swings::find_swings;

// ── Finding a peak ──

#[test]
fn a_clear_peak_is_found() {
    let candles = around(candle(20, 200, 95), 5);

    let swings = find_swings(&candles, settings(3, 0.5), 14).expect("valid");
    let highs: Vec<_> = swings.iter().filter(|s| s.is_high()).collect();

    assert_eq!(highs.len(), 1, "expected one peak, got {swings:?}");
    assert_eq!(highs[0].bar_time(), at(20));
    assert_eq!(highs[0].price(), price(200));
}

#[test]
fn a_peak_is_confirmed_by_the_candle_lookback_later() {
    let candles = around(candle(20, 200, 95), 5);

    let swings = find_swings(&candles, settings(3, 0.5), 14).expect("valid");
    let peak = swings.iter().find(|s| s.is_high()).expect("a peak");

    // Lookback is 3, so candle 23 is what made it knowable.
    assert_eq!(peak.confirmed_at(), at(23));

    // And the type refuses to let anyone use it before then.
    assert!(!peak.is_known_at(at(22)));
    assert!(peak.is_known_at(at(23)));
}

// ── The noise filter ──

#[test]
fn a_bump_smaller_than_the_filter_is_ignored() {
    // ATR settles at 10, so a filter of 0.5 needs the swing to stand out by
    // 5. This one stands out by 2.
    let candles = around(candle(20, 107, 95), 5);

    let swings = find_swings(&candles, settings(3, 0.5), 14).expect("valid");

    assert!(
        !swings.iter().any(|s| s.bar_time() == at(20)),
        "a 2-point bump on a 10-point candle is chop, not structure"
    );
}

#[test]
fn turning_the_filter_off_finds_the_same_bump() {
    let candles = around(candle(20, 107, 95), 5);

    let swings = find_swings(&candles, settings(3, 0.0), 14).expect("valid");

    assert!(swings.iter().any(|s| s.bar_time() == at(20) && s.is_high()));
}

// ── Ties ──

#[test]
fn a_flat_top_produces_no_swing() {
    // Two candles share the highest high. Neither strictly beats the other,
    // so neither is a peak. Missing a level is safer than inventing one.
    let mut candles = flat(20);
    candles.push(candle(20, 200, 95));
    candles.push(candle(21, 200, 95));
    candles.extend((22..27).map(|i| candle(i, 105, 95)));

    let swings = find_swings(&candles, settings(3, 0.5), 14).expect("valid");

    assert!(!swings.iter().any(|s| s.is_high()), "got {swings:?}");
}

// ── A candle can be both ──

#[test]
fn an_outside_bar_is_both_a_high_and_a_low() {
    let candles = around(candle(20, 200, 10), 5);

    let swings = find_swings(&candles, settings(3, 0.5), 14).expect("valid");
    let here: Vec<_> = swings.iter().filter(|s| s.bar_time() == at(20)).collect();

    assert_eq!(here.len(), 2, "got {here:?}");
    assert!(here.iter().any(|s| s.kind() == SwingKind::High));
    assert!(here.iter().any(|s| s.kind() == SwingKind::Low));
}

// ── The end of the history ──

#[test]
fn the_last_few_candles_cannot_be_judged_yet() {
    // A peak, but only two candles follow it and the lookback is three.
    let candles = around(candle(20, 200, 95), 2);

    let swings = find_swings(&candles, settings(3, 0.5), 14).expect("valid");

    // Still unknown. That is correct, not a gap — tomorrow it is a swing.
    assert!(swings.is_empty(), "got {swings:?}");
}

// ── Nothing before ATR exists ──

#[test]
fn nothing_is_found_before_atr_has_warmed_up() {
    // A huge peak at candle 3, long before ATR has 14 candles to work with.
    let mut candles = flat(3);
    candles.push(candle(3, 500, 95));
    candles.extend((4..10).map(|i| candle(i, 105, 95)));

    let swings = find_swings(&candles, settings(3, 0.5), 14).expect("valid");

    // Without ATR there is no idea what a normal candle looks like, so there
    // is no way to say whether 500 is a swing or just how this instrument
    // behaves. Finding nothing beats guessing.
    assert!(swings.is_empty(), "got {swings:?}");
}
