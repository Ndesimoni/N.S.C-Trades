//! Things that go wrong during a backtest.
//!
//! The one that matters is "used data from the future", raised by `guards.rs`.
//! It kills the run completely and on purpose: a contaminated run must produce
//! **no number at all**, rather than a number with a warning attached.
//!
//! The reasoning is about behaviour, not correctness. A warned-about number
//! still gets read, compared, and eventually acted on — especially when it is
//! a good number. Refusing to produce one is the only reliable way to stop a
//! poisoned backtest influencing a decision six weeks later.

use chrono::{DateTime, Utc};
use thiserror::Error;

/// What can go wrong replaying a history.
#[derive(Debug, Error)]
pub enum BacktestError {
    /// The analysis touched something it could not have known yet.
    ///
    /// **This kills the run.** No number comes out, on purpose.
    ///
    /// A number with a warning attached still gets read, compared and
    /// eventually acted on — especially a good one. Refusing to produce one is
    /// the only reliable way to stop a poisoned backtest influencing a
    /// decision six weeks later.
    #[error(
        "lookahead at {now}: {what} was not knowable until {knowable_at} — the run is not trustworthy and has been stopped"
    )]
    LookaheadDetected {
        /// What was touched, in words. "the swing high at 4350 on 2026-06-14".
        what: String,
        /// The moment the analysis was standing at.
        now: DateTime<Utc>,
        /// The first moment it could honestly have been used.
        knowable_at: DateTime<Utc>,
    },

    /// The chart reading refused something — an unfinished candle, a
    /// timeframe that cannot be built from the file, a bad setting.
    ///
    /// Passed straight through rather than reworded. The detail says which
    /// candle to go and look at, and rewording loses it.
    #[error(transparent)]
    Chart(#[from] nsc_ta::error::TaError),

    /// Getting the candles in went wrong.
    #[error(transparent)]
    Data(#[from] nsc_data::error::DataError),

    /// The shared types refused something.
    #[error(transparent)]
    Core(#[from] nsc_core::error::CoreError),
}
