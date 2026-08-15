//! One pip, worked out from how the pair is quoted.

use super::super::{Pair, Thickness};
use super::support::d;

fn pair(symbol: &str, digits: u32) -> Pair {
    Pair {
        symbol: symbol.into(),
        digits,
        nightly_break_minutes: 0,
        levels: Vec::new(),
    }
}

fn thickness(approach_pips: &str) -> Thickness {
    Thickness {
        weekly: d("0.35"),
        daily: d("0.46"),
        h4: d("0.55"),
        approach_pips: d(approach_pips),
    }
}

// A pip is ten ticks, so it falls out of `digits` and never needs its own
// setting. That matters because `digits` is the one thing the bot can work out
// for itself when he sends a brand new pair from his phone.
#[test]
fn a_pip_comes_from_how_the_pair_is_quoted() {
    assert_eq!(pair("XAU/USD", 2).pip(), d("0.1"));
    assert_eq!(pair("EUR/USD", 5).pip(), d("0.0001"));
    assert_eq!(pair("USD/JPY", 3).pip(), d("0.01"));
}

// Nothing real is quoted to nought decimals, but a bad file must not panic in
// a library crate — the watcher would die on one bad pair.
#[test]
fn a_pair_quoted_to_whole_numbers_does_not_blow_up() {
    assert_eq!(pair("ODD", 0).pip(), d("1"));
}

#[test]
fn the_reach_is_that_pip_times_the_setting() {
    assert_eq!(pair("XAU/USD", 2).reach(thickness("1.0")), d("0.1"));
    assert_eq!(pair("XAU/USD", 2).reach(thickness("5.0")), d("0.5"));
    assert_eq!(pair("EUR/USD", 5).reach(thickness("1.0")), d("0.0001"));
}

// The whole reason it is in pips rather than a share of the band. One number
// means the same SIZE OF TOUCH everywhere, while a share would mean ten cents
// on a thin band and ten dollars on a thick one.
#[test]
fn one_setting_means_a_touch_on_every_pair() {
    let setting = thickness("1.0");

    assert_eq!(pair("XAU/USD", 2).reach(setting), d("0.1"));
    assert_eq!(pair("GBP/USD", 5).reach(setting), d("0.0001"));
}
