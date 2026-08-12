//! Fibonacci retracements over a move.
//!
//! The ratios are the easy part. **Which move you measure is the whole game** —
//! the same ratios drawn from a different pair of swings give completely
//! different prices.
//!
//! So the move itself is stored alongside the levels, and it goes into the
//! signal's reasoning. When a Fibonacci signal looks wrong, the move it picked
//! is nearly always the disagreement, and an argument about a move is one you
//! can settle by looking at a chart.
//!
//! ## What is where
//!
//! - [`retracement`] — `FibRetracement`: one move, and the two questions you
//!   ask of it
//!
//! Which ratios matter, and what each one is for, are set in
//! `config/ta.toml`. They are not in here, because a level with no job
//! attached is a line the bot draws and nothing reads.

mod retracement;

#[cfg(test)]
mod tests;

pub use retracement::FibRetracement;
