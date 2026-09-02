//! The lookup that decides whether a pair keeps being watched.

use std::collections::HashMap;

use nsc_core::levels::{Pair, Watch};

use super::super::Watching;
use super::doing::watched_as;

fn watching_gold() -> HashMap<String, Watching> {
    let pair = Pair {
        symbol: "XAU/USD".into(),
        digits: 2,
        nightly_break_minutes: 60,
        approach_share: None,
        levels: Vec::new(),
    };

    let watch = Watch::over(Vec::new(), rust_decimal::Decimal::new(5, 2));

    HashMap::from([("XAU/USD".to_string(), Watching { pair, watch })])
}

/// **The bug, stated as a test.**
///
/// The watch list is keyed `XAU/USD`; the file it came from is
/// `XAUUSD.toml`. Looking the stem up directly can never hit, so the
/// branch meant to keep a pair whose file went unreadable dropped it
/// instead — silently, and for good, because the next reload only happens
/// when a file changes and nothing has to change again.
#[test]
fn a_file_stem_finds_the_pair_it_belongs_to() {
    let old = watching_gold();

    assert!(
        !old.contains_key("XAUUSD"),
        "the stem is not the key — this is what the old code did"
    );

    assert_eq!(watched_as(&old, "XAUUSD"), Some("XAU/USD".to_string()));
}

#[test]
fn a_file_for_a_pair_that_is_not_watched_finds_nothing() {
    let old = watching_gold();

    assert_eq!(watched_as(&old, "EURUSD"), None);
    assert_eq!(watched_as(&old, ""), None);
}

/// A stem that already reads like a symbol still works — nothing here
/// assumes six letters.
#[test]
fn it_does_not_guess_where_the_slash_goes() {
    let mut old = watching_gold();
    let pair = Pair {
        symbol: "BRENT/USD".into(),
        digits: 2,
        nightly_break_minutes: 60,
        approach_share: None,
        levels: Vec::new(),
    };
    let watch = Watch::over(Vec::new(), rust_decimal::Decimal::new(5, 2));
    old.insert("BRENT/USD".to_string(), Watching { pair, watch });

    assert_eq!(watched_as(&old, "BRENTUSD"), Some("BRENT/USD".to_string()));
}
