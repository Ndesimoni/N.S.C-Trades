//! **Why a candle was not a signal**, and which layer said so.
//!
//! `look` answers `Result` rather than `Option` since 2 September 2026.
//! Nothing is the honest answer for the market and a useless one for the
//! record: *"nothing printed"* and *"forty shapes printed and none was near a
//! level"* are completely different problems, and they were the same silence.

use nsc_ta::pattern::Pattern;

use super::support::{band, d, his_gold, rules};
use crate::shape::Traded;
use crate::{Refused, look};

/// The same pattern settings the other tests use.
fn patterns() -> nsc_ta::pattern::Rules {
    nsc_ta::pattern::load(std::path::Path::new("../../config/patterns.toml"))
        .expect("config/patterns.toml should load")
}

/// **A quiet candle is refused at the shape layer and is NOT worth keeping.**
///
/// That is nearly every candle. A row for each would make the rejections table
/// larger than `candles` while saying less — and "there was no shape on it"
/// can be worked out from the candle any time.
#[test]
fn a_candle_with_no_shape_is_not_worth_writing_down() {
    let quiet = Refused::NoShape;

    assert_eq!(quiet.layer(), "shape");
    assert!(!quiet.worth_keeping(), "not a row");
}

/// **A shape he does not trade IS worth keeping.** It cannot be worked out
/// later, because which four he trades is a setting and settings change.
#[test]
fn a_shape_he_does_not_trade_is_worth_writing_down() {
    let tweezer = Refused::NotHis {
        pattern: Pattern::Tweezer { top: true },
    };

    assert_eq!(tweezer.layer(), "shape");
    assert!(tweezer.worth_keeping());
    assert!(
        tweezer.why().contains("not one of the four"),
        "it says why: {}",
        tweezer.why()
    );
}

/// **The most interesting row in the table.** A shape he trades, printing with
/// no level under it. His own push measured without a level came back at 38%
/// over 75 tries, where a coin flip is 50% — so this is the refusal that says
/// whether the level is doing any work.
#[test]
fn a_traded_shape_with_no_level_says_so() {
    let nowhere = Refused::NoLevel {
        shape: Traded::Engulfing { up: true },
        touching: d("1.2345"),
    };

    assert_eq!(nowhere.layer(), "place");
    assert!(nowhere.worth_keeping());

    let why = nowhere.why();
    assert!(why.contains("bullish engulfing"), "names the shape: {why}");
    assert!(why.contains("1.2345"), "and where it printed: {why}");
}

/// **The real thing, through `look`.** His own gold engulfing, judged against
/// a band that is nowhere near it.
#[test]
fn look_refuses_at_the_place_layer_when_no_level_is_near() {
    let (bars, normal) = his_gold();
    let history: Vec<&_> = bars.iter().collect();

    let elsewhere = band();
    let far = nsc_core::levels::Band {
        timeframe: elsewhere.timeframe,
        price: elsewhere.price + d("500"),
        top: elsewhere.top + d("500"),
        bottom: elsewhere.bottom + d("500"),
    };

    let why =
        look(&history, &[far], normal, &patterns(), &rules()).expect_err("no level is near it");

    assert_eq!(
        why.layer(),
        "place",
        "the shape was fine; the place was not"
    );
    assert!(why.worth_keeping(), "this is the row worth having");
}

/// And with no bands at all, which is the same refusal.
#[test]
fn look_refuses_at_the_place_layer_with_no_levels_at_all() {
    let (bars, normal) = his_gold();
    let history: Vec<&_> = bars.iter().collect();

    let why = look(&history, &[], normal, &patterns(), &rules())
        .expect_err("a shape with nothing under it");

    assert_eq!(why.layer(), "place");
}
