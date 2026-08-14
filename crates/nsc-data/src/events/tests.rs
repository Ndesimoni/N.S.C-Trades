use std::sync::Arc;

use chrono::{DateTime, TimeDelta, Utc};
use nsc_core::candle::Candle;
use nsc_core::price::{Pips, Price};
use nsc_core::symbol::{AssetClass, Symbol};
use nsc_core::timeframe::Timeframe;
use rust_decimal::Decimal;

use super::BarClosed;
use crate::error::DataError;

fn gold() -> Arc<Symbol> {
    Arc::new(
        Symbol::new(
            "XAUUSD",
            AssetClass::Metal,
            Decimal::new(1, 1), // 0.1
            2,
            Pips::new(Decimal::from(5)),
            None,
            Some(nsc_core::symbol::Currency::new("USD").expect("valid")),
        )
        .expect("valid symbol"),
    )
}

fn at(minutes: i64) -> DateTime<Utc> {
    "2026-08-14T00:00:00Z"
        .parse::<DateTime<Utc>>()
        .expect("valid timestamp")
        + TimeDelta::try_minutes(minutes).expect("in range")
}

fn candle(minutes: i64, complete: bool) -> Candle {
    let p = |n: i64| Price::new(Decimal::from(n));

    Candle::new(
        at(minutes),
        p(4300),
        p(4350),
        p(4280),
        p(4340),
        None,
        complete,
    )
    .expect("valid candle")
}

#[test]
fn a_finished_candle_becomes_an_event() {
    let event = BarClosed::new(gold(), Timeframe::H4, candle(0, true)).expect("valid");

    assert_eq!(event.symbol().name(), "XAUUSD");
    assert_eq!(event.timeframe(), Timeframe::H4);
    assert_eq!(event.candle().close(), Price::new(Decimal::from(4340)));
}

// The whole point of the type. If a half-formed candle could get in here,
// every piece of analysis downstream would need its own check and one of them
// would eventually be forgotten.
#[test]
fn a_candle_that_is_still_forming_is_refused() {
    let out = BarClosed::new(gold(), Timeframe::H4, candle(0, false));

    assert!(matches!(out, Err(DataError::Core(_))), "got {out:?}");
}

// ── The one answer to "what time is it" ──

#[test]
fn the_moment_is_the_candles_open_time() {
    let event = BarClosed::new(gold(), Timeframe::H4, candle(240, true)).expect("valid");

    assert_eq!(event.at(), at(240));
}

// A swing confirmed by this candle is stamped with this candle's open time, so
// it is knowable now. One confirmed by the next candle is not. Using the close
// time instead would let in swings that are one candle early.
#[test]
fn a_swing_confirmed_by_this_candle_is_knowable_and_the_next_one_is_not() {
    use nsc_core::swing::{Swing, SwingKind};

    let event = BarClosed::new(gold(), Timeframe::H4, candle(240, true)).expect("valid");

    let knowable = Swing::new(
        SwingKind::High,
        at(0),
        at(240),
        Price::new(Decimal::from(4350)),
    )
    .expect("valid swing");

    let not_yet = Swing::new(
        SwingKind::High,
        at(0),
        at(480),
        Price::new(Decimal::from(4350)),
    )
    .expect("valid swing");

    assert!(knowable.is_known_at(event.at()));
    assert!(!not_yet.is_known_at(event.at()));
}

// Cloning an event must not cost three string allocations. A backtest fires
// about sixty thousand of these per instrument per timeframe.
#[test]
fn cloning_an_event_shares_the_symbol_rather_than_copying_it() {
    let symbol = gold();
    let event = BarClosed::new(Arc::clone(&symbol), Timeframe::H4, candle(0, true)).expect("valid");

    let copy = event.clone();

    assert!(std::ptr::eq(event.symbol(), copy.symbol()));
    assert_eq!(
        Arc::strong_count(&symbol),
        3,
        "one held here, two in events"
    );
}
