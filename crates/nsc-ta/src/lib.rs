//! # nsc-ta — reading the chart
//!
//! **This crate never touches the outside world.** Candles in, analysis out.
//! No database, no internet, no async, no reading the clock.
//!
//! That is not a style preference. It is what lets the backtester and the
//! live bot run the same analysis code. The moment they run different code,
//! backtest results stop describing the live bot — and you will not notice,
//! because the mismatch makes backtests look better.
//!
//! ## What depends on what
//!
//! ```text
//!            candles
//!               │
//!            swings          ← everything below is built from these
//!         ┌─────┼─────┬──────────┬──────────┐
//!      levels  trendlines  fibonacci   structure
//!         └─────┴─────┴──────────┴──────────┘
//!                       │
//!                   snapshot     ← what the rules engine reads
//! ```
//!
//! Candlestick patterns run alongside this, straight off the raw candles.
//! Chart patterns run off the swing sequence.
//!
//! ## The rule every module here obeys
//!
//! When analysing candle N, you may only read candles up to N. A swing high
//! can only be used after the candle that confirmed it. No exceptions —
//! `nsc-backtest::guards` exists to catch anyone who forgets.

pub mod aggregate;
pub mod candles;
pub mod config;
pub mod context;
pub mod error;
pub mod fibonacci;
pub mod indicators;
pub mod levels;
pub mod patterns;
pub mod snapshot;
pub mod structure;
pub mod swings;
pub mod trendlines;
