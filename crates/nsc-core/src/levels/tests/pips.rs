//! One pip, worked out from how the pair is quoted.

use super::super::{Pair, Thickness};
use super::support::d;

fn pair(symbol: &str, digits: u32) -> Pair {
    Pair {
        symbol: symbol.into(),
        digits,
        nightly_break_minutes: 0,
        approach_pips: None,
        levels: Vec::new(),
    }
}

/// The same pair, but with its own idea of how close counts.
fn pair_wanting(symbol: &str, digits: u32, pips: &str) -> Pair {
    Pair {
        approach_pips: Some(d(pips)),
        ..pair(symbol, digits)
    }
}

fn thickness(approach_pips: &str) -> Thickness {
    Thickness {
        weekly: d("0.35"),
        daily: d("0.46"),
        h4: d("0.55"),
        approach_pips: d(approach_pips),
        kiss_depth: d("0.25"),
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
    assert_eq!(pair("XAU/USD", 2).reach(thickness("4.0")), d("0.4"));
    assert_eq!(pair("XAU/USD", 2).reach(thickness("1.0")), d("0.1"));
    assert_eq!(pair("EUR/USD", 5).reach(thickness("4.0")), d("0.0004"));
}

// The whole reason it is in pips rather than a share of the band. One number
// means the same SIZE OF NUDGE everywhere, while a share would mean ten cents
// on a thin band and ten dollars on a thick one.
#[test]
fn one_setting_covers_every_pair() {
    let setting = thickness("4.0");

    assert_eq!(pair("XAU/USD", 2).reach(setting), d("0.4"));
    assert_eq!(pair("GBP/USD", 5).reach(setting), d("0.0004"));
}

// ── But a pair may want its own ──

// Four pips is two minutes of gold and nearly an hour of euro. So gold gets to
// ask for more without dragging every other pair along with it.
#[test]
fn a_pair_can_want_more_room_than_the_shared_setting() {
    let shared = thickness("4.0");

    assert_eq!(pair("XAU/USD", 2).reach(shared), d("0.4"));
    assert_eq!(pair_wanting("XAU/USD", 2, "40").reach(shared), d("4.0"));
}

#[test]
fn a_pair_can_want_less_too() {
    assert_eq!(
        pair_wanting("EUR/USD", 5, "1").reach(thickness("4.0")),
        d("0.0001")
    );
}

// A pair file written before the override existed has none, and must still get
// the shared number rather than nothing.
#[test]
fn a_pair_without_one_falls_back_to_the_shared_setting() {
    assert_eq!(pair("GBP/USD", 5).approach_pips, None);
    assert_eq!(pair("GBP/USD", 5).reach(thickness("4.0")), d("0.0004"));
}

// Typing it into the file is the ONLY way he will ever set this, so the trip
// through TOML is the part worth pinning. The tests above build a Pair in
// memory and would all pass with the field unreadable from a file.
#[test]
fn a_pair_file_can_carry_its_own_number() {
    let text = r#"
symbol = "XAU/USD"
digits = 2
approach_pips = 40

[[level]]
timeframe = "weekly"
price = "4094"
"#;

    let pair: Pair = toml::from_str(text).expect("a valid pair file");

    assert_eq!(pair.approach_pips, Some(d("40")));
    assert_eq!(pair.reach(thickness("4.0")), d("4.0"), "$4, not 40 cents");
}

// Every pair file he has today was written before this existed. None of them
// may stop loading.
#[test]
fn a_pair_file_written_before_the_setting_existed_still_loads() {
    let text = r#"
symbol = "EUR/USD"
digits = 5

[[level]]
timeframe = "weekly"
price = "1.15000"
"#;

    let pair: Pair = toml::from_str(text).expect("a valid pair file");

    assert_eq!(pair.approach_pips, None);
    assert_eq!(pair.reach(thickness("4.0")), d("0.0004"));
}

// ── Writing a price the way the cards write it ──

// The cards have grouped thousands and held the trailing zeros since the
// beginning. The captions did neither, so gold arrived as 4094.00 underneath a
// picture calling it 4,094.00 — the same number, twice, differently.
#[test]
fn a_price_in_a_caption_reads_like_it_does_on_the_card() {
    use super::super::pretty;

    assert_eq!(pretty(d("4094"), 2), "4,094.00");
    assert_eq!(pretty(d("4132.5736"), 2), "4,132.57");
    assert_eq!(pretty(d("1.15"), 5), "1.15000");
    assert_eq!(pretty(d("999"), 2), "999.00");
    assert_eq!(pretty(d("1234567.5"), 2), "1,234,567.50");
}

// Nothing quoted in whole numbers exists here, but a price with no decimals
// must not come out with a stray dot on the end.
#[test]
fn a_whole_number_keeps_no_stray_point() {
    use super::super::pretty;

    assert_eq!(pretty(d("4094"), 0), "4,094");
    assert_eq!(pretty(d("-4094.5"), 2), "-4,094.50");
}
