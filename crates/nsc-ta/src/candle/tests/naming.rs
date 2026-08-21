//! Which name each shape gets, checked against candles that actually printed.

use std::path::Path;

use rust_decimal::Decimal;

use super::real::every_one;
use crate::candle::{Named, Rules, Shape, load};

fn rules() -> Rules {
    load(Path::new("../../config/candles.toml")).expect("config/candles.toml should read")
}

/// What the taxonomy called each one, and what this crate calls it.
///
/// **Where they differ it is on purpose**, and the reason is always the same:
/// two textbook names, one shape.
const EXPECTED: [(&str, Named); 18] = [
    // Three names for a doji with two long wicks. The four numbers cannot
    // separate a "standard" doji from a rickshaw man from a long-legged one,
    // because nothing about the candle separates them.
    ("standard_doji", Named::LongLeggedDoji),
    ("long_legged", Named::LongLeggedDoji),
    ("rickshaw", Named::LongLeggedDoji),
    // These two really are different shapes: the tail is all on one side.
    ("dragonfly", Named::DragonflyDoji),
    ("gravestone", Named::GravestoneDoji),
    // No wick at either end. Seven of these in 4,165 candles.
    ("bull_marubozu", Named::Marubozu),
    ("bear_marubozu", Named::Marubozu),
    // An opening marubozu IS a belt-hold. Same candle, two names.
    ("opening_maru", Named::BeltHold),
    ("closing_maru", Named::ClosingMarubozu),
    // A long bull candle that opened at its low IS a bullish belt-hold — and
    // the taxonomy found the SAME CANDLE for both names.
    ("long_bull", Named::BeltHold),
    ("bull_belt", Named::BeltHold),
    ("bear_belt", Named::BeltHold),
    // This one has wicks at both ends, so it is only a long candle.
    ("long_bear", Named::LongBody),
    ("spinning_top", Named::SpinningTop),
    ("high_wave", Named::HighWave),
    // Hammer, hanging man, paper umbrella and takuri are one shape. The
    // taxonomy's clearest hammer and clearest takuri are the SAME CANDLE.
    ("hammer", Named::LongLowerWick),
    ("takuri", Named::LongLowerWick),
    ("shooting_star", Named::LongUpperWick),
];

/// **Every shape, on the candle that actually printed it.**
#[test]
fn every_real_candle_gets_the_name_it_should() {
    let rules = rules();

    for one in every_one() {
        let shape = Shape::of(&one.bar, Decimal::ONE).expect("a real candle has a shape");
        let (_, expected) = EXPECTED
            .iter()
            .find(|(called, _)| *called == one.called)
            .unwrap_or_else(|| panic!("{} is not in the expected list", one.called));

        assert_eq!(
            shape.named(&rules),
            *expected,
            "{} ({}) — body {} upper {} lower {}",
            one.called,
            one.stamp,
            shape.body,
            shape.upper,
            shape.lower,
        );
    }
}

/// **A dragonfly is a doji, not a rejection**, and both are true of it.
///
/// Its body is 0.008 and its tail is 0.897 — it passes the long-lower-wick
/// test easily. The body test wins because it is the tighter statement: a
/// candle with no body at all is that, first.
#[test]
fn the_tightest_rule_wins_when_two_are_true() {
    let rules = rules();
    let dragonfly = every_one()
        .into_iter()
        .find(|one| one.called == "dragonfly")
        .expect("the dragonfly is in the list");

    let shape = Shape::of(&dragonfly.bar, Decimal::ONE).expect("it has a shape");

    assert!(
        shape.body <= rules.body.small,
        "it would pass the rejection body test"
    );
    assert!(
        shape.lower >= shape.body * rules.rejection.tail_to_body,
        "and its tail test"
    );
    assert_eq!(shape.named(&rules), Named::DragonflyDoji);
}

/// **The same candle cannot come back with two names**, which is the whole
/// reason this returns one and not a list. The taxonomy found the identical
/// candle for hammer and takuri, and again for long-bull and bullish belt-hold.
#[test]
fn one_candle_gets_exactly_one_name() {
    let rules = rules();
    let all = every_one();

    for pair in [("hammer", "takuri"), ("long_bull", "bull_belt")] {
        let named: Vec<Named> = [pair.0, pair.1]
            .iter()
            .map(|called| {
                let one = all
                    .iter()
                    .find(|one| one.called == *called)
                    .expect("in the list");
                Shape::of(&one.bar, Decimal::ONE)
                    .expect("a shape")
                    .named(&rules)
            })
            .collect();

        assert_eq!(
            named[0], named[1],
            "{} and {} are the same candle",
            pair.0, pair.1
        );
    }
}

/// A shape found once in three years and one found 408 times are different
/// kinds of fact, and the spoken names have to stay distinct to tell them
/// apart at all.
#[test]
fn every_name_is_its_own_word() {
    let mut said: Vec<&str> = EXPECTED.iter().map(|(_, name)| name.spoken()).collect();
    said.sort_unstable();
    said.dedup();

    assert!(said.len() >= 8, "the shapes collapsed too far: {said:?}");
    assert!(said.iter().all(|word| !word.is_empty()));
}

/// **How often a shape turns up is part of what it means.**
///
/// A spinning top appears 408 times in 4,165 gold candles — one in ten. A
/// shape that common cannot carry a decision on its own, and a rule built on
/// one would fire every day and mean nothing.
///
/// A high wave appeared exactly once in three years. That is the other danger:
/// a rule nothing ever triggers looks like a rule that never misfires.
///
/// The counts are carried beside the candles so the next person to loosen a
/// threshold has to look at them.
#[test]
fn the_counts_say_which_shapes_can_carry_a_decision() {
    let all = every_one();

    let found = |called: &str| {
        all.iter()
            .find(|one| one.called == called)
            .map(|one| one.found)
            .expect("in the list")
    };

    assert!(
        found("spinning_top") > found("hammer") * 2,
        "the spinning top is meant to be the commonest thing on the chart",
    );

    assert_eq!(found("high_wave"), 1, "one in three years");
    assert!(
        found("bull_marubozu") + found("bear_marubozu") < 10,
        "a true marubozu is rare"
    );
}
