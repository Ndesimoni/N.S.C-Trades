use rust_decimal::Decimal;

use super::*;
use crate::error::CoreError;

fn price(mantissa: i64, scale: u32) -> Price {
    Price::new(Decimal::new(mantissa, scale))
}

#[test]
fn subtracting_two_prices_gives_a_distance() {
    let gap = price(10850, 4) - price(10800, 4);
    assert_eq!(gap.value(), Decimal::new(50, 4));
}

#[test]
fn pips_round_trip() {
    let pip_size = Decimal::new(1, 4); // 0.0001, EURUSD
    let gap = price(10850, 4) - price(10800, 4);

    let pips = gap.to_pips(pip_size).expect("pip size is not zero");
    assert_eq!(pips.value(), Decimal::new(50, 0));

    assert_eq!(pips.to_distance(pip_size), gap);
}

#[test]
fn a_distance_in_normal_candles() {
    let gap = PriceDistance::new(Decimal::new(30, 4)); // 0.0030
    let atr = PriceDistance::new(Decimal::new(10, 4)); // 0.0010

    assert_eq!(
        gap.to_atr_multiple(atr).expect("atr above zero").value(),
        3.0
    );
}

#[test]
fn flat_market_does_not_panic() {
    let gap = PriceDistance::new(Decimal::new(30, 4));
    let flat = PriceDistance::new(Decimal::ZERO);

    assert_eq!(gap.to_atr_multiple(flat), Err(CoreError::ZeroAtr));
}

// ── Round numbers ──

fn step(mantissa: i64, scale: u32) -> RoundStep {
    RoundStep::new(Decimal::new(mantissa, scale)).expect("a step above zero")
}

#[test]
fn sterling_steps_by_a_hundred_pips() {
    let step = step(100, 4); // 0.0100 — 0.8000, 0.8100, 0.8200
    let here = price(8137, 4); // 0.8137

    assert_eq!(step.below(here), price(8100, 4));
    assert_eq!(step.above(here), price(8200, 4));
    assert_eq!(step.nearest(here), price(8100, 4));
}

#[test]
fn the_yen_steps_by_whole_numbers() {
    let step = step(1, 0); // 1.00 — 78.00, 79.00
    let here = price(7862, 2); // 78.62

    assert_eq!(step.nearest(here), price(79, 0));
}

#[test]
fn a_big_instrument_steps_by_a_thousand() {
    let step = step(1000, 0);
    let here = price(90480, 0);

    assert_eq!(step.below(here), price(90000, 0));
    assert_eq!(step.nearest(here), price(90000, 0));
}

#[test]
fn distance_from_a_round_number_keeps_its_side() {
    let step = step(100, 4); // 0.0100

    // Just above 0.8000, coming up to it from underneath is a different trade
    // from falling towards it, so the sign is kept.
    let above = price(8007, 4).distance_from_round(step);
    assert_eq!(above.value(), Decimal::new(7, 4));

    let below = price(7993, 4).distance_from_round(step);
    assert_eq!(below.value(), Decimal::new(-7, 4));

    let exactly = price(8000, 4).distance_from_round(step);
    assert!(exactly.is_zero());
    assert!(step.is_round(price(8000, 4)));
}

// Oil traded below zero in April 2020. Rejecting that would delete a real week
// of history, so the maths has to keep working underneath it.
#[test]
fn round_numbers_work_below_zero() {
    let step = step(10, 0);
    let here = price(-3720, 2); // -37.20

    assert_eq!(step.below(here), price(-40, 0));
    assert_eq!(step.above(here), price(-30, 0));
    assert_eq!(step.nearest(here), price(-40, 0));
}

#[test]
fn a_step_of_zero_is_refused() {
    let refused = RoundStep::new(Decimal::ZERO);

    assert!(matches!(refused, Err(CoreError::InvalidRoundStep { .. })));
}

// ── How round is it ──

/// Halves, hundreds, then the big figure.
fn sterling_ladder() -> RoundLadder {
    RoundLadder::new(vec![step(50, 4), step(100, 4), step(1000, 4)]).expect("in order")
}

#[test]
fn the_more_zeros_a_price_ends_in_the_stronger_it_is() {
    let ladder = sterling_ladder();

    assert_eq!(ladder.rank(price(8050, 4)), 1); // 0.8050 — a half
    assert_eq!(ladder.rank(price(8800, 4)), 2); // 0.8800 — a hundred
    assert_eq!(ladder.rank(price(8000, 4)), 3); // 0.8000 — the big figure
    assert_eq!(ladder.rank(price(8037, 4)), 0); // not round at all
}

#[test]
fn the_nearest_round_number_comes_back_with_its_strength() {
    let ladder = sterling_ladder();

    let (number, rank) = ladder.nearest(price(8014, 4)).expect("a ladder with steps");

    assert_eq!(number, price(8000, 4));
    assert_eq!(rank, 3, "0.8000 is on every rung");
}

// The concept, in the trader's own examples. It comes down to trailing zeros:
// the further a number rounds off, the more people are watching it.
#[test]
fn the_traders_own_examples_come_out_in_the_right_order() {
    let ladder =
        RoundLadder::new(vec![step(10, 0), step(100, 0), step(1000, 0)]).expect("in order");

    let rank = |n| ladder.rank(price(n, 0));

    // 88000 rounds off to the thousand. 80800 stops at the hundred.
    assert!(rank(88000) > rank(80800));

    // 88800 stops at the hundred. 88880 only reaches the ten.
    assert!(rank(88800) > rank(88880));

    assert_eq!(
        (rank(88000), rank(80800), rank(88800), rank(88880)),
        (3, 2, 2, 1)
    );
}

// Out of order, how round a price counts as would depend on the order someone
// typed the settings in.
#[test]
fn a_ladder_that_is_not_in_order_is_refused() {
    let refused = RoundLadder::new(vec![step(1000, 4), step(50, 4)]);

    assert!(matches!(refused, Err(CoreError::InvalidRoundLadder { .. })));
}

// The rungs have to sit on top of each other, or counting them says nothing.
// With steps of 3 and 10, the price 30 is on the small rung only and 10 is on
// the big rung only — both score one, which claims they are equally round when
// they cannot be compared at all.
#[test]
fn a_ladder_whose_rungs_do_not_stack_is_refused() {
    let refused = RoundLadder::new(vec![step(3, 0), step(10, 0)]);

    assert!(matches!(refused, Err(CoreError::InvalidRoundLadder { .. })));
}

#[test]
fn a_ladder_with_no_steps_is_refused() {
    let refused = RoundLadder::new(Vec::new());

    assert!(matches!(refused, Err(CoreError::InvalidRoundLadder { .. })));
}

#[test]
fn a_stop_buffer_becomes_a_real_distance() {
    let atr = PriceDistance::new(Decimal::new(10, 4)); // 0.0010
    let buffer = AtrMultiple::new(0.3);

    let distance = buffer.to_distance(atr).expect("0.3 is representable");
    assert_eq!(distance.value(), Decimal::new(3, 4)); // 0.3 x 0.0010 = 0.0003

    let stop = price(10800, 4) - distance;
    assert_eq!(stop.value(), Decimal::new(107970, 5)); // 1.0800 - 0.0003
}
