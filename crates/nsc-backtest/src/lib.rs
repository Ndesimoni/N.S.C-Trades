//! # nsc-backtest — replaying history and testing settings
//!
//! Runs the **same** `nsc-ta` and `nsc-strategy` code the live bot runs, fed
//! old candles instead of a broker feed. That is the whole design: not a
//! second copy of the strategy written for testing, but the actual strategy
//! given different data.
//!
//! ## One candle at a time, never all at once
//!
//! Candles are replayed in order, and the engine only ever sees what had
//! printed at that moment.
//!
//! This is slower than processing the whole history at once — and it is the
//! only way to be sure the result was actually achievable. Every "process it
//! all at once" backtest of a swing-based strategy leaks future knowledge
//! somewhere. The leak is always small, always flattering, and very hard to
//! find afterwards.
//!
//! This is where Rust earns its place. You will not run one backtest. You
//! will run thousands of setting combinations, and the difference between
//! seconds and an afternoon decides whether you explore your options or guess
//! at them.
//!
//! ## What a good result looks like
//!
//! Not the highest number. A **patch** — a group of nearby settings that all
//! work. One setting that beats its neighbours by a mile has found a quirk of
//! this particular history and will not survive next month.

pub mod error;
pub mod guards;
pub mod harness;
pub mod metrics;
pub mod replay;
pub mod report;
pub mod sweep;
