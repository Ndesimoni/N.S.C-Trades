//! Shapes that need more than one candle.

use nsc_core::pattern::CandleShape;

use super::helpers::*;

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

// ── Inside bar ──

#[test]
fn a_candle_wholly_inside_the_one_before_is_an_inside_bar() {
    let wide = candle(0, 20, 100, 0, 80);
    let inside = candle(1, 40, 70, 30, 55);

    let seen = found(&[wide, inside], CandleShape::InsideBar).expect("an inside bar");

    assert_eq!(
        seen.bias(),
        nsc_core::pattern::Bias::Neutral,
        "a coil points nowhere until price leaves it"
    );
    assert_eq!(seen.spans(), 2);
}

#[test]
fn a_candle_poking_outside_is_not_an_inside_bar() {
    let wide = candle(0, 20, 100, 10, 80);
    let poking = candle(1, 40, 70, 5, 55);

    assert!(found(&[wide, poking], CandleShape::InsideBar).is_none());
}

// Nothing narrowed, so nothing coiled.
#[test]
fn two_identical_candles_are_not_an_inside_bar() {
    let one = candle(0, 20, 100, 0, 80);
    let same = candle(1, 20, 100, 0, 80);

    assert!(found(&[one, same], CandleShape::InsideBar).is_none());
}

// ── Star ──

#[test]
fn a_push_a_stall_and_a_push_back_is_a_star() {
    // Up 60, a small stalled candle, then down giving back 50 of it.
    let push = candle(0, 10, 72, 5, 70);
    let stall = candle(1, 72, 82, 68, 74);
    let back = candle(2, 70, 75, 10, 20);

    let seen = found(&[push, stall, back], CandleShape::Star).expect("an evening star");

    assert!(seen.bias().is_bearish(), "a stall at a top");
    assert_eq!(seen.spans(), 3);
}

#[test]
fn a_morning_star_is_the_same_shape_at_a_bottom() {
    let push = candle(0, 70, 75, 5, 10);
    let stall = candle(1, 8, 12, 2, 7);
    let back = candle(2, 10, 80, 8, 70);

    let seen = found(&[push, stall, back], CandleShape::Star).expect("a morning star");

    assert!(seen.bias().is_bullish());
}

// Without the close-back test, any small candle followed by any down candle
// would be an evening star. This is what separates a reversal from a pause.
#[test]
fn a_third_candle_that_barely_gives_anything_back_is_not_a_star() {
    // A real body, but it only takes back 25 of the 60 that was pushed.
    let push = candle(0, 10, 72, 5, 70);
    let stall = candle(1, 72, 82, 68, 74);
    let limp = candle(2, 70, 75, 40, 45);

    assert!(found(&[push, stall, limp], CandleShape::Star).is_none());
}

#[test]
fn a_middle_candle_with_a_real_body_is_not_a_stall() {
    let push = candle(0, 10, 72, 5, 70);
    let another_push = candle(1, 70, 95, 68, 92);
    let back = candle(2, 90, 95, 10, 20);

    assert!(found(&[push, another_push, back], CandleShape::Star).is_none());
}
