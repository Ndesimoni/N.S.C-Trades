//! Names for the candlestick patterns this project reads.
//!
//! Six shapes, and they are the six the trader actually uses — not a textbook
//! index. A pattern in the code that nobody would act on is noise in every
//! backtest it appears in.
//!
//! ## A shape, and nothing about what it means
//!
//! Knowing that a candle engulfed the one before it does not tell you to buy.
//! Whether it matters needs the level it happened at, the trend it happened
//! in, and the timeframe it happened on. That is `nsc-strategy`.
//!
//! So a sighting here says what the shape was and how pronounced it was. It
//! never looks left, and it never scores anything. Textbook descriptions
//! usually bolt the context on — "a hammer after a downtrend" — and that half
//! belongs to the rules.
//!
//! ## What is where
//!
//! - [`shape`] — `CandleShape` and `Bias`: which pattern, and which way it
//!   points if it points at all
//! - [`sighting`] — `PatternSighting`: one shape found on one candle, with the
//!   measurements that made it one
//!
//! Chart patterns — head and shoulders, triangles, flags — are deliberately
//! not here. They are many-swing, far more subjective and much weaker, and
//! trend plus levels plus Fibonacci plus candlesticks gives most of the edge
//! for a fraction of the work.

mod shape;
mod sighting;

#[cfg(test)]
mod tests;

pub use shape::{Bias, CandleShape, DojiKind};
pub use sighting::PatternSighting;
