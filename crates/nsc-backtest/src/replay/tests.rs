use std::sync::Arc;

use chrono::{DateTime, TimeDelta, Utc, Weekday};
use chrono_tz::America::New_York;
use nsc_core::candle::Candle;
use nsc_core::price::{Pips, Price};
use nsc_core::symbol::{AssetClass, Symbol};
use nsc_core::timeframe::{DayBoundary, Timeframe};
use rust_decimal::Decimal;

use super::Replay;

fn gold() -> Arc<Symbol> {
    Arc::new(
        Symbol::new(
            "XAUUSD",
            AssetClass::Metal,
            Decimal::new(1, 1),
            2,
            Pips::new(Decimal::from(5)),
            None,
            None,
        )
        .expect("valid symbol"),
    )
}

fn boundary() -> DayBoundary {
    DayBoundary::new(17, 0, New_York, Weekday::Sun).expect("17:00 is a real time")
}

/// Candles every 15 minutes from the start of a trading day.
fn candles(count: i64) -> Vec<Candle> {
    // 21:00 UTC is 17:00 New York in summer — the start of a trading day, so
    // the bigger candles line up from the first one.
    let start = "2026-07-13T21:00:00Z"
        .parse::<DateTime<Utc>>()
        .expect("valid timestamp");

    (0..count)
        .map(|i| {
            let p = |n: i64| Price::new(Decimal::from(n));
            Candle::new(
                start + TimeDelta::try_minutes(i * 15).expect("in range"),
                p(4300),
                p(4310),
                p(4290),
                p(4305),
                None,
                true,
            )
            .expect("valid candle")
        })
        .collect()
}

fn replay(derived: &[Timeframe]) -> Replay {
    Replay::new(gold(), Timeframe::M15, derived, boundary()).expect("valid replay")
}

// ── Every candle produces at least itself ──

#[test]
fn each_candle_closes_its_own_bar() {
    let mut walker = replay(&[]);

    for candle in candles(6) {
        let bars = walker.feed(&candle).expect("valid");

        assert_eq!(bars.len(), 1);
        assert_eq!(bars[0].timeframe(), Timeframe::M15);
        assert_eq!(bars[0].candle().open_time(), candle.open_time());
    }
}

// ── The bit a plain loop would miss ──

// The fourth 15-minute candle also finishes the hour that began with the
// first. The bot learns both at that moment, so the replay has to as well.
#[test]
fn a_candle_that_also_finishes_an_hour_closes_two_bars() {
    let mut walker = replay(&[Timeframe::H1]);
    let candles = candles(9);

    // The first four build the hour but do not finish it.
    for candle in &candles[..4] {
        assert_eq!(walker.feed(candle).expect("valid").len(), 1);
    }

    // The fifth starts the next hour, which closes the first one.
    let bars = walker.feed(&candles[4]).expect("valid");

    assert_eq!(bars.len(), 2, "the hour and the 15-minute");
    assert_eq!(bars[0].timeframe(), Timeframe::H1);
    assert_eq!(bars[1].timeframe(), Timeframe::M15);
}

// ── The order that cannot be wrong ──

// Smaller timeframes read the bigger ones for context, so the bigger ones have
// to have moved first. The wrong order gives different answers on the same
// candles — and would differ between the backtester and the bot.
#[test]
fn bars_come_out_biggest_timeframe_first() {
    let mut walker = replay(&[Timeframe::M30, Timeframe::H1, Timeframe::H4]);

    let mut seen = Vec::new();
    for candle in candles(40) {
        let bars = walker.feed(&candle).expect("valid");
        if bars.len() > 2 {
            seen = bars.iter().map(|b| b.timeframe()).collect();
            break;
        }
    }

    assert!(seen.len() > 2, "nothing closed several at once: {seen:?}");

    for pair in seen.windows(2) {
        assert!(pair[0] > pair[1], "out of order: {seen:?}");
    }
}

// ── What it refuses ──

#[test]
fn a_timeframe_smaller_than_the_file_is_refused() {
    // You cannot cut a 15-minute candle into 5-minute ones. Quietly ignoring
    // this would leave you wondering why no bars ever arrived.
    let out = Replay::new(gold(), Timeframe::H1, &[Timeframe::M15], boundary());

    assert!(out.is_err());
}

#[test]
fn asking_for_the_base_timeframe_twice_does_not_double_it() {
    let mut walker = replay(&[Timeframe::M15, Timeframe::M15]);
    let bars = walker.feed(&candles(1)[0]).expect("valid");

    assert_eq!(bars.len(), 1, "the base is emitted once");
}

#[test]
fn a_candle_that_is_still_forming_is_refused() {
    let mut walker = replay(&[]);

    let p = |n: i64| Price::new(Decimal::from(n));
    let forming = Candle::new(
        "2026-07-13T21:00:00Z"
            .parse::<DateTime<Utc>>()
            .expect("valid"),
        p(4300),
        p(4310),
        p(4290),
        p(4305),
        None,
        false,
    )
    .expect("valid candle");

    assert!(walker.feed(&forming).is_err());
}

// ── The test that matters most ──

// A replay hands the analysis one candle at a time. The chart tool builds the
// whole history at once. If those two ever produced different candles, every
// backtest would stop describing what the bot does — and it would not look
// broken, it would just look different.
//
// They cannot differ here, because both go through the same aggregator. This
// test is what keeps that true.
#[test]
fn replaying_gives_the_same_candles_as_building_them_all_at_once() {
    let candles = candles(400);

    for timeframe in [Timeframe::M30, Timeframe::H1, Timeframe::H4] {
        let all_at_once =
            nsc_ta::aggregate::aggregate(&candles, Timeframe::M15, timeframe, &boundary())
                .expect("valid");

        let mut walker = replay(&[timeframe]);
        let mut one_at_a_time = Vec::new();

        for candle in &candles {
            for bar in walker.feed(candle).expect("valid") {
                if bar.timeframe() == timeframe {
                    one_at_a_time.push(bar.candle());
                }
            }
        }

        assert_eq!(
            all_at_once, one_at_a_time,
            "{timeframe} differs between replay and bulk"
        );
    }
}
