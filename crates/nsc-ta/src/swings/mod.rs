//! Finding swing highs and lows.
//!
//! **Swings sit under everything else** — levels, trendlines, Fibonacci
//! anchors and trend direction are all counted off them.
//!
//! ```text
//!     rules.rs     the four numbers, all shares of a move
//!     extreme.rs   a price and its candle, and the arithmetic of a run
//!     facing.rs    which way round a leg is, written once
//!     memory.rs    the recent runs, and what counts as a move
//!     leg.rs       THE RULE — the two ways a peak proves itself
//!     seed.rs      the start, before any swing has confirmed
//!     step.rs      what a candle did to the leg it arrived on
//!     finder.rs    the state machine, one candle at a time
//! ```
//!
//! **No candle counting.** A peak is not a peak because of how many candles
//! sit either side of it — that passes a lazy rounded top with twenty quiet
//! candles round it, and fails a sharp turn with four. What proves a peak is
//! what price did afterwards.

mod extreme;
mod facing;
mod finder;
mod leg;
mod memory;
mod rules;
mod seed;
mod step;

#[cfg(test)]
mod tests;

pub use finder::{Finder, SwingsError};
pub use rules::{Rules, RulesError, load};
