//! Building blocks the guard tests share.

use std::sync::Arc;

use chrono::{DateTime, TimeDelta, Utc, Weekday};
use chrono_tz::America::New_York;
use nsc_core::candle::Candle;
use nsc_core::level::{Band, Level};
use nsc_core::price::{Pips, Price};
use nsc_core::swing::{Swing, SwingKind};
use nsc_core::symbol::{AssetClass, Symbol};
use nsc_core::timeframe::{DayBoundary, Timeframe};
use rust_decimal::Decimal;

use super::super::Guard;
use crate::error::BacktestError;

pub(super) fn boundary() -> DayBoundary {
    DayBoundary::new(17, 0, New_York, Weekday::Sun).expect("17:00 is a real time")
}

/// 21:00 UTC is 17:00 New York in summer — the start of a trading day, so the
/// bigger candles line up from minute zero.
pub(super) fn at(minutes: i64) -> DateTime<Utc> {
    "2026-08-13T21:00:00Z"
        .parse::<DateTime<Utc>>()
        .expect("valid timestamp")
        + TimeDelta::try_minutes(minutes).expect("in range")
}

pub(super) fn p(n: i64) -> Price {
    Price::new(Decimal::from(n))
}

pub(super) fn guard(minutes: i64) -> Guard {
    Guard::standing_at(at(minutes), boundary())
}

pub(super) fn candle(minutes: i64, complete: bool) -> Candle {
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

/// A swing that sat at `bar_time` and was proved by the candle opening
/// `confirmed_at`.
pub(super) fn swing(bar_time: i64, confirmed_at: i64) -> Swing {
    Swing::new(SwingKind::High, at(bar_time), at(confirmed_at), p(4350)).expect("valid swing")
}

pub(super) fn level(timeframe: Timeframe, last_touch: i64, confirmed_at: i64) -> Level {
    let band = Band::new(p(4340), p(4360)).expect("valid band");

    Level::new(band, timeframe, 3, at(0), at(last_touch), at(confirmed_at)).expect("valid level")
}

pub(super) fn caught(out: Result<impl std::fmt::Debug, BacktestError>) -> BacktestError {
    match out {
        Err(e @ BacktestError::LookaheadDetected { .. }) => e,
        other => panic!("expected the run to be killed, got {other:?}"),
    }
}

pub(super) fn gold() -> Arc<Symbol> {
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
