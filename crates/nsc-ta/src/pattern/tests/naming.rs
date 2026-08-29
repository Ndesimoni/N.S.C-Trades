//! That every pattern has its own name, and says how many candles it needs.
//!
//! **Neither of these was checked until `nsc-bull` and `nsc-bear` were added.**
//! Adding a pattern means adding a name and an arity, and both are the sort of
//! thing that is got wrong by copying the line above and editing half of it.
//!
//! `candles()` matters more than it looks. It ends in a wildcard arm — anything
//! that is not a star or a march answers two — so a three-candle pattern added
//! later would quietly answer two and be handed one candle too few. The
//! compiler cannot catch that. This can.

use std::collections::HashSet;

use crate::pattern::Pattern;

/// Every pattern there is, written out.
///
/// **Written out on purpose, not generated.** The point is that adding a
/// variant to the enum does not silently add it here too — someone has to come
/// and say what it is called and how long it is.
fn all() -> Vec<Pattern> {
    vec![
        Pattern::Engulfing { up: true },
        Pattern::Engulfing { up: false },
        Pattern::Harami { up: true },
        Pattern::Harami { up: false },
        Pattern::Tweezer { top: true },
        Pattern::Tweezer { top: false },
        Pattern::PiercingLine,
        Pattern::DarkCloudCover,
        Pattern::Star {
            up: true,
            abandoned: true,
        },
        Pattern::Star {
            up: false,
            abandoned: true,
        },
        Pattern::Star {
            up: true,
            abandoned: false,
        },
        Pattern::Star {
            up: false,
            abandoned: false,
        },
        Pattern::Marching { up: true },
        Pattern::Marching { up: false },
        Pattern::Push { up: true },
        Pattern::Push { up: false },
    ]
}

/// **Two patterns sharing a name would be read as one.** A tally would add
/// them together and nobody would see it happen.
#[test]
fn every_pattern_has_its_own_name() {
    let mut seen: HashSet<&str> = HashSet::new();

    for pattern in all() {
        let name = pattern.spoken();

        assert!(!name.is_empty(), "{pattern:?} has no name");
        assert!(seen.insert(name), "two patterns are both called {name:?}");
    }
}

/// **The `nsc-` prefix is the house namespace, not a maker's mark.**
///
/// It says the pattern is part of HIS system. It does NOT say he invented it —
/// `nsc-bullish-engulfing` is the engulfing out of any book, adopted and
/// renamed. Only `push` is actually his, and nothing in the name shows that.
///
/// **This test used to assert the opposite** and was right at the time: the
/// prefix started out marking his own pattern alone. He put it on engulfing on
/// 22 August 2026 and the meaning changed underneath it.
#[test]
fn the_prefix_marks_what_is_in_his_system() {
    assert_eq!(Pattern::Push { up: true }.spoken(), "nsc-bull");
    assert_eq!(Pattern::Push { up: false }.spoken(), "nsc-bear");
    assert_eq!(
        Pattern::Engulfing { up: true }.spoken(),
        "nsc-bullish-engulfing"
    );
    assert_eq!(
        Pattern::Engulfing { up: false }.spoken(),
        "nsc-bearish-engulfing"
    );
}

/// **A prefixed name still has to say which pattern it is.**
///
/// `nsc-` on its own carries no meaning, so whatever follows it must survive
/// being stripped and still be a name somebody could act on.
#[test]
fn a_prefixed_name_still_names_something() {
    for pattern in all() {
        let name = pattern.spoken();

        if let Some(rest) = name.strip_prefix("nsc-") {
            assert!(
                !rest.is_empty(),
                "{pattern:?} is called nothing but a prefix"
            );
        }
    }
}

/// **Every pattern needs two candles or three, and nothing else.**
#[test]
fn every_pattern_says_how_many_candles_it_needs() {
    for pattern in all() {
        let needed = pattern.candles();

        assert!(
            needed == 2 || needed == 3,
            "{pattern:?} claims to need {needed} candles"
        );
    }
}

/// **His is a run of two**, and this is what the wildcard arm cannot check for
/// itself.
#[test]
fn his_own_pattern_is_a_run_of_two() {
    assert_eq!(Pattern::Push { up: true }.candles(), 2);
    assert_eq!(Pattern::Push { up: false }.candles(), 2);
}
