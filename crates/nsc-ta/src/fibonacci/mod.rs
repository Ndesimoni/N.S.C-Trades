//! Drawing Fibonacci levels automatically.
//!
//! The ratios are trivial. **Picking which move to measure is the actual
//! work**, and it is where this module earns its keep — the same ratios drawn
//! from a different pair of swings give completely different prices.
//!
//! ## Which move
//!
//! The last completed leg: the two most recent confirmed swings. That is the
//! move price is retracing right now, and it is the same run the swing finder
//! measured to confirm those swings in the first place.
//!
//! Using anything else would let two parts of the chart-reading code disagree
//! about what the current move is, and a disagreement like that is invisible
//! until a signal looks wrong and nobody can say why.
//!
//! **Still open:** which timeframe, and what to do when a bigger move is still
//! running inside a smaller one. See `docs/worksheets/fibonacci.md`.
//!
//! ## The four levels each do a different job
//!
//! Not one zone with lines in it:
//!
//! | Level | What it is for |
//! |---|---|
//! | 0.382 | a reading — a pullback this shallow means the trend is strong |
//! | 0.5 to 0.618 | the golden zone, where to look to get in |
//! | 0.786 | where stops get looked at, not always |
//!
//! This module reports where those prices are and how deep price has come
//! back. It decides nothing: the location layer picks an entry, the
//! invalidation layer places a stop, and both live in `nsc-strategy`.
//!
//! ## What is where
//!
//! - [`draw`] — picking the move and building the retracement from it
//! - [`reading`] — the four prices, and how deep price is now

mod draw;
mod reading;

#[cfg(test)]
mod tests;

pub use draw::last_move;
pub use reading::FibReading;
