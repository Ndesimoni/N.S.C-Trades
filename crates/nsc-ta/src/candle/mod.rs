//! What ONE candle is.
//!
//! Measured first, named second. `shape.rs` does the measuring and has no
//! opinion in it; the naming comes after, from thresholds in `config/`.
//!
//! A run of candles is a different question and lives in `pattern/`.

mod named;
mod naming;
mod rules;
mod shape;

#[cfg(test)]
mod tests;

pub use named::Named;
pub use rules::{Body, Rejection, Rules, RulesError, Wick, load};
pub use shape::Shape;
