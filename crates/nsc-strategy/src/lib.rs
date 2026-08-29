//! The rules, applied in one place.
//!
//! **One rule, and it is a sentence:** a shape he trades, sitting at a level
//! he drew.
//!
//! ```text
//!     shape.rs    which shapes count, and where each one is measured from
//!     place.rs    THE TEST — is it at the level
//!     rules.rs    the settings, out of config/strategy.toml
//!     finding.rs  the one way in
//!     reasons.rs  the one sentence that explains it
//! ```
//!
//! ## Why the level is the whole point
//!
//! `nsc-bull` and `nsc-bear` were measured across five pairs and five
//! timeframes: followed for ten candles they reached +1 normal candle before
//! -1 in **29 of 75, where a coin flip is 50%**.
//!
//! **None of those had a level under them.** So this crate is the test of the
//! sentence `pattern/README.txt` already ends on — a pattern is a description,
//! and what makes one worth anything is the level it printed at.
//!
//! If these come back at 38% too, the level does not save it. That is a
//! finding, not a failure.
//!
//! ## It cannot reach anything
//!
//! No feed, no clock, no database — `Cargo.toml` has none of them, so nothing
//! here *can* fetch. Everything is handed in, which is what lets the
//! backtester and the live bot run these exact rules and agree.
//!
//! **There is no "if we are backtesting" anywhere in here, and there never
//! may be.** The moment there is, the backtest is testing something else — and
//! the mismatch makes results look better rather than broken.
//!
//! ## It reports. It does not enter.
//!
//! Version 1 sends signals and places no trades. Where the stop goes has not
//! been settled, and a signal with no stop is a reading rather than a trade —
//! so it says what it saw and stops there.

pub mod finding;
pub mod place;
pub mod reasons;
pub mod rules;
pub mod shape;

#[cfg(test)]
mod tests;

pub use finding::{Signal, look};
pub use place::{Placing, where_it_sits};
pub use rules::{Rules, StrategyError, load};
pub use shape::{Traded, traded};
