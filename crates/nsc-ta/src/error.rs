//! Things that can go wrong while reading the chart.
//!
//! Typed errors, not a catch-all. The backtester runs this code over years of
//! candles inside a settings sweep, and it needs to tell apart "not enough
//! candles yet to work out ATR" — skip this one and carry on — from "these
//! settings make no sense" — stop everything.
//!
//! A vague error forces the caller to treat both the same way, which means
//! either killing sweeps over harmless gaps or silently swallowing real
//! mistakes.
//!
//! Nothing here crashes. A bad candle in year three must not destroy the two
//! hours of work before it.

use chrono::{DateTime, Utc};
use thiserror::Error;

/// What can go wrong reading a chart.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum TaError {
    /// Asked for something that needs more history than we have. Wanting a
    /// 14-period ATR from 10 candles, for example.
    ///
    /// Normal at the start of a series. Skip the candle and carry on — a few
    /// candles later there will be enough.
    #[error("need at least {needed} candles but only have {have}")]
    NotEnoughCandles { needed: usize, have: usize },

    /// A setting in `config/ta.toml` cannot be used.
    ///
    /// A settings mistake, not a data problem. Stop rather than skip —
    /// retrying will never fix it.
    #[error("{setting} is {value}, which cannot be used: {why}")]
    BadSetting {
        setting: String,
        value: String,
        why: String,
    },

    /// An unfinished candle reached the analysis.
    ///
    /// This is a bug in whatever fed the candle in, not bad market data. An
    /// unfinished candle's high and low have not happened yet, so anything
    /// built from it is using prices the market never printed.
    ///
    /// Stop. A result built on this is worse than no result, because it
    /// looks fine.
    #[error("the candle at {open_time} has not finished forming yet")]
    IncompleteCandle { open_time: DateTime<Utc> },

    /// Something the shared types refused.
    ///
    /// Mostly this is `SwingKnownTooEarly` — a swing that claimed to be
    /// knowable before the candle it sits on. If that ever appears, the
    /// detector has a lookahead bug.
    ///
    /// Passed straight through rather than reworded. Rewording an error loses
    /// the detail that tells you which candle to go and look at.
    #[error(transparent)]
    Core(#[from] nsc_core::error::CoreError),
}
