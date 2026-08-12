//! Does each detector find its own shape?

use nsc_core::pattern::{CandleShape, DojiKind};

use super::helpers::*;

// ── Pin bar ──

#[test]
fn a_long_tail_below_a_small_body_is_a_bullish_pin_bar() {
    // Body 10 at the top, 80 of wick below, 10 above.
    let seen = found(&[candle(0, 80, 100, 0, 90)], CandleShape::PinBar).expect("a pin bar");

    assert!(seen.bias().is_bullish(), "the tail points down");
    assert_eq!(seen.proportions().tail_to_body(), Some(8.0));
    assert_eq!(seen.spans(), 1);
}

#[test]
fn a_long_tail_above_a_small_body_is_a_bearish_pin_bar() {
    let seen = found(&[candle(0, 20, 100, 0, 10)], CandleShape::PinBar).expect("a pin bar");

    assert!(seen.bias().is_bearish());
}

#[test]
fn a_fat_body_is_not_a_pin_bar() {
    // Body 50 of 100 — well past the third a pin bar is allowed.
    assert!(found(&[candle(0, 20, 100, 0, 70)], CandleShape::PinBar).is_none());
}

// A quarter nose, a quarter body and a half tail passes every other test —
// the tail is twice the body and the body is under a third. But the body is
// sitting in the middle of the candle, which makes it a spinning top leaning
// one way rather than a rejection.
#[test]
fn a_body_stranded_in_the_middle_is_not_a_pin_bar() {
    assert!(found(&[candle(0, 50, 100, 0, 75)], CandleShape::PinBar).is_none());
}

// Long wicks both sides is indecision, and it means close to the opposite of
// a pin bar. The nose test is what separates them.
#[test]
fn long_wicks_on_both_sides_are_not_a_pin_bar() {
    assert!(found(&[candle(0, 45, 100, 0, 55)], CandleShape::PinBar).is_none());
}

// ── Doji ──

#[test]
fn a_body_of_almost_nothing_is_a_doji() {
    let seen = found(
        &[candle(0, 50, 100, 0, 52)],
        CandleShape::Doji(DojiKind::LongLegged),
    )
    .expect("a long-legged doji");

    assert_eq!(seen.bias(), nsc_core::pattern::Bias::Neutral);
}

#[test]
fn a_doji_with_no_upper_wick_is_a_dragonfly() {
    assert!(
        found(
            &[candle(0, 98, 100, 0, 99)],
            CandleShape::Doji(DojiKind::Dragonfly)
        )
        .is_some()
    );
}

#[test]
fn a_doji_with_no_lower_wick_is_a_gravestone() {
    assert!(
        found(
            &[candle(0, 2, 100, 0, 1)],
            CandleShape::Doji(DojiKind::Gravestone)
        )
        .is_some()
    );
}

// Two true things about one candle. Which of them matters needs the level and
// the trend, and that is the rules layer's question.
#[test]
fn one_candle_can_be_both_a_pin_bar_and_a_doji() {
    let shapes: Vec<_> = seen(&[candle(0, 98, 100, 0, 99)])
        .iter()
        .map(|s| s.shape())
        .collect();

    assert!(shapes.contains(&CandleShape::PinBar), "got {shapes:?}");
    assert!(
        shapes.contains(&CandleShape::Doji(DojiKind::Dragonfly)),
        "got {shapes:?}"
    );
}

// ── Engulfing ──

#[test]
fn a_body_that_swallows_the_one_before_it_is_engulfing() {
    let down = candle(0, 60, 70, 40, 50);
    let up = candle(1, 45, 100, 30, 70);

    let seen = found(&[down, up], CandleShape::Engulfing).expect("engulfing");

    assert!(seen.bias().is_bullish());
    assert_eq!(seen.spans(), 2);
}

// Wicks are ignored on purpose. The body is where the market spent the
// session; a wick is where it was rejected.
#[test]
fn covering_only_the_wicks_is_not_engulfing() {
    let down = candle(0, 70, 100, 0, 30);
    let up = candle(1, 40, 90, 10, 60);

    assert!(found(&[down, up], CandleShape::Engulfing).is_none());
}

#[test]
fn two_candles_the_same_colour_are_not_engulfing() {
    let up = candle(0, 40, 70, 30, 60);
    let bigger_up = candle(1, 30, 100, 20, 80);

    assert!(found(&[up, bigger_up], CandleShape::Engulfing).is_none());
}

// ── Belt-hold ──

#[test]
fn a_long_candle_opening_at_its_low_is_a_bullish_belt_hold() {
    let seen = found(&[candle(0, 0, 100, 0, 95)], CandleShape::BeltHold).expect("a belt-hold");

    assert!(seen.bias().is_bullish());
}

#[test]
fn a_candle_with_a_wick_under_its_open_is_not_a_belt_hold() {
    assert!(found(&[candle(0, 20, 100, 0, 95)], CandleShape::BeltHold).is_none());
}

// ── Tweezers ──

#[test]
fn two_candles_topping_out_together_are_a_tweezer_top() {
    let up = candle(0, 20, 100, 10, 90);
    let down = candle(1, 90, 98, 20, 30);

    let seen = found(&[up, down], CandleShape::Tweezers).expect("tweezers");

    assert!(seen.bias().is_bearish(), "a top is a rejection of higher");
}

#[test]
fn two_candles_bottoming_out_together_are_a_tweezer_bottom() {
    let down = candle(0, 80, 90, 0, 20);
    let up = candle(1, 20, 70, 2, 60);

    let seen = found(&[down, up], CandleShape::Tweezers).expect("tweezers");

    assert!(seen.bias().is_bullish());
}

#[test]
fn highs_further_apart_than_the_tolerance_are_not_tweezers() {
    // The tolerance is 0.05 of a normal candle, so 5 here. These two tops are
    // 20 apart, which is a different price.
    let up = candle(0, 20, 100, 10, 90);
    let down = candle(1, 75, 80, 20, 30);

    assert!(found(&[up, down], CandleShape::Tweezers).is_none());
}
