//! What KIND of thing a candle did at a zone.
//!
//! `AtZone` says where it ENDED — above, below, inside. This is about the
//! difference between a wick that grazed the edge and a candle that drove a
//! third of the way in. Both "closed above"; they are not the same event.

use rust_decimal::Decimal;

use super::super::{Action, AtZone, Band, Timeframe, action, happening, how_deep};
use super::support::d;
use crate::candle::Bar;

/// The gold band he drew at 4094 — 4055.43 to 4132.57, about 77 thick.
fn gold() -> Band {
    Band::around(Timeframe::Weekly, d("4094"), d("220.42"), d("0.35"))
}

fn candle(open: &str, high: &str, low: &str, close: &str) -> Bar {
    serde_json::from_str(&format!(
        r#"{{"datetime":"2026-08-19 13:00:00","open":"{open}","high":"{high}",
            "low":"{low}","close":"{close}"}}"#
    ))
    .expect("a valid candle")
}

fn kiss() -> Decimal {
    d("0.25")
}

// A wick that grazed the top edge and a candle that drove a third of the way in
// both "closed above". THEY ARE NOT THE SAME EVENT, and the one he acts on is
// the second.
#[test]
fn a_graze_and_a_real_push_are_told_apart() {
    let band = gold();

    // Dipped about a point into a band 77 thick — under a fiftieth.
    let graze = candle("4160", "4165", "4131.5", "4155");
    assert_eq!(action(&band, &graze, kiss()), Action::Kissed);

    // Drove a third of the way in and was sold back out.
    let push = candle("4160", "4165", "4106", "4150");
    assert_eq!(action(&band, &push, kiss()), Action::Rejected);
}

// THE ONE THAT WOULD BE BACKWARDS. A candle that opened above the zone and
// closed below it went THROUGH — the level did not hold. Judged on depth alone
// it drove far in and closed outside, which is the shape of a rejection, and
// the card would say the level held when it broke.
#[test]
fn cutting_through_is_never_called_a_rejection() {
    let band = gold();

    let down = candle("4160", "4165", "4000", "4010");
    assert_eq!(action(&band, &down, kiss()), Action::CutThrough);

    let up = candle("4000", "4200", "3990", "4180");
    assert_eq!(action(&band, &up, kiss()), Action::CutThrough);

    // And both are deep, which is what would have made them rejections.
    assert!(how_deep(&band, &down) > kiss());
}

#[test]
fn a_candle_that_ended_inside_has_settled_nothing() {
    assert_eq!(
        action(&gold(), &candle("4110", "4125", "4100", "4120"), kiss()),
        Action::Settled
    );
}

#[test]
fn a_candle_that_never_reached_it_did_nothing() {
    let was = action(&gold(), &candle("4200", "4220", "4150", "4210"), kiss());

    assert_eq!(was, Action::Missed);
    assert!(!was.worth_saying());
}

// How deep counts is a setting, so the same candle can be a graze or a push.
#[test]
fn how_deep_counts_as_a_push_is_a_setting() {
    let band = gold();
    let bar = candle("4160", "4165", "4120", "4155"); // about 16% in

    assert_eq!(action(&band, &bar, d("0.10")), Action::Rejected);
    assert_eq!(action(&band, &bar, d("0.50")), Action::Kissed);
}

// The side matters as much as the action. "Kissed it" alone says nothing about
// whether the level held as support or as resistance.
#[test]
fn the_words_carry_which_side_it_held() {
    assert_eq!(
        happening(Action::Kissed, AtZone::ClosedAbove),
        "kissed it and held above"
    );
    assert_eq!(
        happening(Action::Kissed, AtZone::ClosedBelow),
        "kissed it and held below"
    );
}
