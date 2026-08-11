//! Timeframes, and the maths that goes with them.
//!
//! Handles: how long each candle lasts, snapping any moment back to the start
//! of its candle, and where a trading day and week begin.
//!
//! The daily candle is the tricky one. In forex, the day does not end at
//! midnight UTC — it ends at the time set in `config/app.toml`, usually 5pm
//! New York. That time decides where every daily level sits, so it is applied
//! here, in one place, instead of being worked out again by whoever needs it.
//!
//! ## Everything is anchored to the daily close
//!
//! Not just the daily candle — every timeframe. A 4-hour candle starts at the
//! daily close, then every 4 hours after it.
//!
//! The reason is that candles have to nest. Six 4-hour candles must make
//! exactly one daily candle, or the aggregator has no way to say "this daily
//! candle is now finished". If 4-hour candles were anchored to midnight UTC
//! instead, the daily candle would start in the middle of one, and the two
//! would drift apart twice a year when New York changes its clocks.
//!
//! Practical consequence: check the 4-hour candles this produces against your
//! own chart once. Different platforms anchor them differently, and if yours
//! disagrees you want to know now, not after you have trusted a level.
//!
//! ## Why this module never reads a config file
//!
//! `nsc-core` is not allowed to read files or check the clock. So the daily
//! close is passed in, as a [`DayBoundary`] that somebody else built from
//! `app.toml`.
//!
//! That is also what makes it testable: hand it a made-up boundary and check
//! the answers, with no config anywhere near it.
//!
//! ## What is where
//!
//! - [`kind`] — which timeframes exist, and how long each one lasts
//! - [`boundary`] — where the trading day and week begin
//! - [`snap`] — which candle a given moment belongs to

mod boundary;
mod kind;
mod snap;

#[cfg(test)]
mod tests;

pub use boundary::DayBoundary;
pub use kind::Timeframe;
