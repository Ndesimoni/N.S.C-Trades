//! Spotting candlestick patterns.
//!
//! Six shapes, and they are the six the trader actually uses: pin bar,
//! engulfing, doji, belt-hold, tweezers — with hammer and inverted hammer
//! being the same shape as a pin bar, pointing one way or the other.
//!
//! ## A shape, and nothing about what it means
//!
//! Knowing a candle engulfed the one before it does not tell you to buy. That
//! needs the level it happened at, the trend it happened in and the timeframe
//! it happened on, and all three are `nsc-strategy`.
//!
//! **These detectors never look left.** Textbook bolts the context onto the
//! pattern — "a hammer after a downtrend" — and that half is the rules'. The
//! same candle in open space is still a hammer; it is simply not a trade.
//!
//! ## Two yardsticks, used for different questions
//!
//! **Shape** is measured as shares of the candle's own height. A body that is
//! a fifth of its candle is a fifth on EURUSD and a fifth on gold, so no ATR
//! and no pip size come into it.
//!
//! **Size** — whether a candle is big at all — is measured in ATR. Only the
//! belt-hold and the tweezer tolerance need it, and both need it for good
//! reason: one is about a long candle, the other about two prices being near
//! enough to call the same.
//!
//! ## The numbers are textbook, not the trader's
//!
//! Standard measurements, taken as defaults so the detectors could be built.
//! They live in `[candles]` in `config/ta.toml` and are marked there as
//! borrowed. Replacing them is editing a file, not changing code.
//!
//! ## What is where
//!
//! - [`pin_bar`], [`doji`], [`engulfing`], [`belt_hold`], [`tweezers`] — one
//!   shape each
//! - [`finder`] — asks all five about the newest candle
//! - [`series`] — a whole history at once, for the backtester
//!
//! `inside_bar` and `star` are stubs from the original scaffolding. Neither is
//! on the trader's list, and they are left untouched pending a decision rather
//! than deleted or quietly built.

mod belt_hold;
mod doji;
mod engulfing;
mod pin_bar;
mod tweezers;

mod finder;
mod series;

pub mod inside_bar;
pub mod star;

#[cfg(test)]
mod tests;

pub use finder::look_at;
pub use series::find_patterns;
