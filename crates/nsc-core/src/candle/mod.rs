//! One candle, exactly as the feed sends it — and the one question that
//! matters about it: **has it finished?**
//!
//! A candle still forming has a high that is not its high and a close that is
//! not its close. Reading it is reading prices the market has not printed. The
//! answer is worked out from the clock, never from where the candle sits in
//! the list.

mod bar;
mod error;

#[cfg(test)]
mod tests;

pub use bar::{Bar, Series, normal_candle};
pub use error::CandleError;
