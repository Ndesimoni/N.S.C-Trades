//! One candle has finished. This is the only thing the analysis ever sees.

use std::sync::Arc;

use chrono::{DateTime, Utc};
use nsc_core::candle::Candle;
use nsc_core::symbol::Symbol;
use nsc_core::timeframe::Timeframe;

use crate::error::DataError;

/// A candle finished forming.
///
/// The backtester replays these out of a file or the database as fast as it
/// can. The live bot builds them from the broker feed. Everything downstream
/// takes the same type and cannot tell which one it is talking to.
///
/// ## The promise it makes
///
/// **The candle inside is complete.** There is no way to build one of these
/// around a candle that is still forming, because [`BarClosed::new`] refuses
/// it. So no code downstream has to remember to check.
///
/// That matters more than it sounds. An unfinished candle's high and low have
/// not happened yet, so anything built on one is using prices the market never
/// printed — and it does not error, it just makes the results better.
///
/// ## Why the symbol is shared rather than copied
///
/// A backtest over six years of 15-minute candles fires about sixty thousand
/// of these per instrument, per timeframe. A `Symbol` holds three strings, so
/// copying one into every event would mean millions of allocations to carry a
/// fact that never changes.
#[derive(Debug, Clone)]
pub struct BarClosed {
    symbol: Arc<Symbol>,
    timeframe: Timeframe,
    candle: Candle,
}

impl BarClosed {
    /// Refuses a candle that has not finished forming.
    ///
    /// This is the whole point of the type. If it were possible to build one
    /// of these around a half-formed candle, every piece of analysis
    /// downstream would need its own check, and one of them would eventually
    /// be forgotten.
    pub fn new(
        symbol: Arc<Symbol>,
        timeframe: Timeframe,
        candle: Candle,
    ) -> Result<Self, DataError> {
        if !candle.is_complete() {
            return Err(DataError::Core(
                nsc_core::error::CoreError::ImpossibleCandle {
                    open_time: candle.open_time(),
                    detail: "a bar cannot close while it is still forming".into(),
                },
            ));
        }

        Ok(Self {
            symbol,
            timeframe,
            candle,
        })
    }

    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }

    pub fn timeframe(&self) -> Timeframe {
        self.timeframe
    }

    pub fn candle(&self) -> Candle {
        self.candle
    }

    /// The moment the analysis is standing at.
    ///
    /// **Pass this to every `is_known_at` check.** It is what decides whether
    /// a swing or a level had confirmed yet, and getting it wrong is the one
    /// mistake that makes a backtest look better rather than broken.
    ///
    /// It is the candle's **open** time, not its close, and that is
    /// deliberate. A swing confirmed by this candle is stamped with this
    /// candle's open time, so asking `is_known_at(at())` says yes for
    /// everything knowable now and no for everything that needed the next
    /// candle. Using the close time instead would let in swings that are one
    /// candle early.
    pub fn at(&self) -> DateTime<Utc> {
        self.candle.open_time()
    }
}
