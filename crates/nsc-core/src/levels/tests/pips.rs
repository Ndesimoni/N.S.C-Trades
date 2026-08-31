//! One pip, worked out from how the pair is quoted.

use super::super::{Band, Pair, Thickness, Timeframe};
use super::support::d;

fn pair(symbol: &str, digits: u32) -> Pair {
    Pair {
        symbol: symbol.into(),
        digits,
        nightly_break_minutes: 0,
        approach_share: None,
        levels: Vec::new(),
    }
}

/// The same pair, but with its own idea of how close counts.
fn pair_wanting(symbol: &str, digits: u32, share: &str) -> Pair {
    Pair {
        approach_share: Some(d(share)),
        ..pair(symbol, digits)
    }
}

fn thickness(approach_share: &str) -> Thickness {
    Thickness {
        weekly: d("0.35"),
        daily: d("0.46"),
        h4: d("0.55"),
        approach_share: d(approach_share),
        kiss_depth: d("0.25"),
        only_breaks: true,
        close_cards: Default::default(),
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

/// **A share, not a price**, since 31 August 2026.
///
/// The pair no longer decides a distance at all — it hands back a share, and
/// each band works out its own reach from its own thickness. Before this the
/// pair returned one price for every one of its levels.
#[test]
fn the_pair_hands_back_a_share_not_a_distance() {
    assert_eq!(pair("XAU/USD", 2).reach_share(thickness("0.05")), d("0.05"));
    assert_eq!(pair("EUR/USD", 5).reach_share(thickness("0.05")), d("0.05"));
}

/// **The same share is a different distance on every band, and that is the
/// point.**
///
/// In pips it was the other way round: four pips was 22% of an AUD/USD daily
/// band and 0.03% of a gold weekly one. The Aussie fired constantly and gold
/// had no approach warning at all.
#[test]
fn one_share_becomes_each_bands_own_distance() {
    let share = pair("AUD/USD", 5).reach_share(thickness("0.05"));

    // An AUD/USD daily band of 17.9 pips, and a gold weekly one of about $153.
    let aussie = Band::around(Timeframe::Daily, d("0.71500"), d("0.00389"), d("0.46"));
    let gold = Band::around(Timeframe::Weekly, d("4094"), d("438"), d("0.35"));

    let near_aussie = aussie.thickness() * share;
    let near_gold = gold.thickness() * share;

    // A twentieth of each, so each is the same fraction of its own band.
    assert_eq!(near_aussie / aussie.thickness(), share);
    assert_eq!(near_gold / gold.thickness(), share);

    // And they are wildly different prices, as they must be.
    assert!(near_gold > near_aussie * d("1000"));
}

// ── But a pair may want its own ──

#[test]
fn a_pair_can_want_more_room_than_the_shared_setting() {
    let shared = thickness("0.05");

    assert_eq!(pair("XAU/USD", 2).reach_share(shared), d("0.05"));
    assert_eq!(
        pair_wanting("XAU/USD", 2, "0.20").reach_share(shared),
        d("0.20")
    );
}

#[test]
fn a_pair_can_want_less_too() {
    assert_eq!(
        pair_wanting("EUR/USD", 5, "0.01").reach_share(thickness("0.05")),
        d("0.01")
    );
}

// A pair file written before the override existed has none, and must still get
// the shared number rather than nothing.
#[test]
fn a_pair_without_one_falls_back_to_the_shared_setting() {
    assert_eq!(pair("GBP/USD", 5).approach_share, None);
    assert_eq!(pair("GBP/USD", 5).reach_share(thickness("0.05")), d("0.05"));
}

// Typing it into the file is the ONLY way he will ever set this, so the trip
// through TOML is the part worth pinning.
#[test]
fn a_pair_file_can_carry_its_own_number() {
    let text = r#"
symbol = "XAU/USD"
digits = 2
approach_share = 0.20

[[level]]
timeframe = "weekly"
price = "4094"
"#;

    let pair: Pair = toml::from_str(text).expect("a valid pair file");

    assert_eq!(pair.approach_share, Some(d("0.20")));
    assert_eq!(pair.reach_share(thickness("0.05")), d("0.20"));
}

/// **An old file saying `approach_pips = 40` must not be read as a share.**
///
/// Forty times the band would parse perfectly and fire on everything — the
/// loudest possible bug arriving in total silence. There is deliberately no
/// serde alias, so the old name is ignored and the shared share is used.
#[test]
fn an_old_pip_setting_is_ignored_rather_than_believed() {
    let text = r#"
symbol = "XAU/USD"
digits = 2
approach_pips = 40

[[level]]
timeframe = "weekly"
price = "4094"
"#;

    let pair: Pair = toml::from_str(text).expect("it still loads");

    assert_eq!(pair.approach_share, None, "the old name is not adopted");
    assert_eq!(pair.reach_share(thickness("0.05")), d("0.05"));
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

    assert_eq!(pair.approach_share, None);
    assert_eq!(pair.reach_share(thickness("0.05")), d("0.05"));
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
