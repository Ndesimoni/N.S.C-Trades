//! Is a bigger candle ever handed out before it has finished?

use nsc_core::candle::Candle;
use nsc_core::timeframe::Timeframe;

use super::helpers::*;
use crate::aggregate::{Aggregator, aggregate};
use crate::error::TaError;

// ── Never too early ──

// THE RULE THIS MODULE EXISTS FOR. Four candles fill an hour, but nothing has
// yet proved the hour is over — the feed could be late, the market could be
// shut. Only the fifth candle makes the first hour knowable.
#[test]
fn an_hour_is_not_finished_until_the_next_one_starts() {
    let exactly_full =
        aggregate(&drift(4), Timeframe::M15, Timeframe::H1, &boundary()).expect("valid");

    assert!(
        exactly_full.is_empty(),
        "four candles fill an hour but do not finish it: {exactly_full:?}"
    );
}

#[test]
fn the_last_bucket_is_always_left_out() {
    // Nine candles: two full hours, then one candle into a third.
    let hourly = aggregate(&drift(9), Timeframe::M15, Timeframe::H1, &boundary()).expect("valid");

    assert_eq!(hourly.len(), 2, "the third hour is still forming");
}

#[test]
fn every_candle_handed_back_says_it_is_complete() {
    let hourly = aggregate(&drift(20), Timeframe::M15, Timeframe::H1, &boundary()).expect("valid");

    assert!(!hourly.is_empty());
    assert!(hourly.iter().all(|candle| candle.is_complete()));
}

// The forming candle exists for drawing a live chart. It says so plainly, and
// every analysis in this project refuses an incomplete candle.
#[test]
fn the_forming_candle_is_marked_incomplete() {
    let mut builder =
        Aggregator::new(Timeframe::M15, Timeframe::H1, boundary()).expect("a bigger timeframe");
    for candle in drift(3) {
        builder.update(&candle).expect("valid");
    }

    let forming = builder.forming().expect("valid").expect("one is forming");

    assert!(!forming.is_complete());
    assert_eq!(forming.open(), price(100));
}

#[test]
fn nothing_is_forming_before_the_first_candle() {
    let builder =
        Aggregator::new(Timeframe::M15, Timeframe::H1, boundary()).expect("a bigger timeframe");

    assert!(builder.forming().expect("valid").is_none());
}

// ── Guards ──

#[test]
fn an_unfinished_smaller_candle_is_refused() {
    let still_forming = Candle::new(
        start(),
        price(100),
        price(105),
        price(95),
        price(100),
        None,
        false,
    )
    .expect("valid candle");

    let mut builder =
        Aggregator::new(Timeframe::M15, Timeframe::H1, boundary()).expect("a bigger timeframe");

    assert!(matches!(
        builder.update(&still_forming),
        Err(TaError::IncompleteCandle { .. })
    ));
}

#[test]
fn an_empty_history_builds_nothing() {
    let hourly = aggregate(&[], Timeframe::M15, Timeframe::H1, &boundary()).expect("valid");

    assert!(hourly.is_empty());
}

// ── Found by reading the code back, not by a failing test ──

// Out of order, the bucket about to be sealed would be sealed by a candle
// from BEFORE it, and the history handed back would run backwards. Nothing
// downstream checks that.
#[test]
fn a_candle_arriving_out_of_order_is_refused() {
    let mut builder =
        Aggregator::new(Timeframe::M15, Timeframe::H1, boundary()).expect("a bigger timeframe");

    builder
        .update(&candle(4, 100, 105, 95, 101))
        .expect("valid");

    assert!(matches!(
        builder.update(&candle(1, 100, 105, 95, 101)),
        Err(TaError::Core(
            nsc_core::error::CoreError::CandlesOutOfOrder { .. }
        ))
    ));
}

#[test]
fn the_same_candle_twice_is_refused() {
    let mut builder =
        Aggregator::new(Timeframe::M15, Timeframe::H1, boundary()).expect("a bigger timeframe");

    let twice = candle(0, 100, 105, 95, 101);
    builder.update(&twice).expect("valid");

    assert!(builder.update(&twice).is_err());
}

// Building 15-minute candles out of 4-hour ones is not a hard job, it is a
// meaningless one — and it would quietly produce a chart that looks fine.
#[test]
fn building_smaller_candles_out_of_bigger_ones_is_refused() {
    let backwards = Aggregator::new(Timeframe::H4, Timeframe::M15, boundary());

    assert!(matches!(backwards, Err(TaError::CannotAggregate { .. })));
}

#[test]
fn building_the_same_timeframe_is_refused() {
    let pointless = Aggregator::new(Timeframe::H1, Timeframe::H1, boundary());

    assert!(matches!(pointless, Err(TaError::CannotAggregate { .. })));
}

// A weekend, a holiday or a dead feed leaves a hole. The bucket before it is
// still finished, and the one after starts clean — nothing is invented in
// between.
#[test]
fn a_gap_in_the_data_still_seals_the_candle_before_it() {
    let mut candles = drift(4);
    // Jump forward two full hours, leaving one empty.
    candles.push(candle(12, 200, 205, 195, 201));

    let hourly = aggregate(&candles, Timeframe::M15, Timeframe::H1, &boundary()).expect("valid");

    assert_eq!(hourly.len(), 1, "the hour before the gap: {hourly:?}");
    assert_eq!(hourly[0].close(), price(104));
}
