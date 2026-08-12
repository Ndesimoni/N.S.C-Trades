//! Do the bigger candles come out right?

use nsc_core::timeframe::Timeframe;

use super::helpers::*;
use crate::aggregate::aggregate;

// ── Building them ──

#[test]
fn four_fifteen_minute_candles_make_an_hour() {
    // Five candles: four fill the first hour, the fifth starts the next one
    // and is what makes the first knowable.
    let hourly = aggregate(&drift(5), Timeframe::M15, Timeframe::H1, &boundary()).expect("valid");

    assert_eq!(hourly.len(), 1, "got {hourly:?}");
    assert_eq!(hourly[0].open_time(), start());
    assert!(hourly[0].is_complete());
}

#[test]
fn the_bigger_candle_takes_the_first_open_and_the_last_close() {
    let hourly = aggregate(&drift(5), Timeframe::M15, Timeframe::H1, &boundary()).expect("valid");

    assert_eq!(hourly[0].open(), price(100), "the first candle's open");
    assert_eq!(hourly[0].close(), price(104), "the fourth candle's close");
}

#[test]
fn the_bigger_candle_takes_the_highest_high_and_the_lowest_low() {
    let mut candles = drift(5);
    // A spike in the middle of the hour.
    candles[2] = candle(2, 102, 300, 10, 103);

    let hourly = aggregate(&candles, Timeframe::M15, Timeframe::H1, &boundary()).expect("valid");

    assert_eq!(hourly[0].high(), price(300));
    assert_eq!(hourly[0].low(), price(10));
}

#[test]
fn sixteen_fifteen_minute_candles_make_a_four_hour() {
    let four_hour =
        aggregate(&drift(17), Timeframe::M15, Timeframe::H4, &boundary()).expect("valid");

    assert_eq!(four_hour.len(), 1, "got {four_hour:?}");
    assert_eq!(four_hour[0].open_time(), start());
}

// Six 4-hour candles fill exactly one day, which is what anchoring everything
// to the daily close is for.
#[test]
fn four_hour_candles_nest_inside_the_day() {
    let candles = drift(97);

    let four_hour = aggregate(&candles, Timeframe::M15, Timeframe::H4, &boundary()).expect("valid");
    let daily = aggregate(&candles, Timeframe::M15, Timeframe::D1, &boundary()).expect("valid");

    assert_eq!(four_hour.len(), 6, "six of them in a day");
    assert_eq!(daily.len(), 1);
    assert_eq!(four_hour[0].open_time(), daily[0].open_time());
    assert_eq!(
        daily[0].high(),
        four_hour.iter().map(|c| c.high()).max().expect("some")
    );
}
