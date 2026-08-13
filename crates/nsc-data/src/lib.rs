//! # nsc-data — brokers, database, and checking the data is sound
//!
//! The line between the outside world and the two clean crates. Everything
//! here is async and can fail. Nothing here makes a trading decision.
//!
//! ## The trait that matters
//!
//! `MarketDataSource` hides your broker. Switching from OANDA to MetaTrader
//! to a data vendor touches this crate and nothing else.
//!
//! Worth having from day one, because the broker is the decision most likely
//! to change and the hardest to undo if it leaks into every crate.
//!
//! ## Checking the data is not optional
//!
//! A missing hour of candles does not announce itself. It quietly shifts a
//! swing high, which shifts a level, which changes every signal after it —
//! and the backtest still finishes and still prints a believable number.
//!
//! `gaps.rs` exists so that broken data is loud instead of plausible.

pub mod backfill;
pub mod cache;
pub mod error;
pub mod events;
pub mod gaps;
pub mod levels;
pub mod source;
pub mod sources;
pub mod store;
