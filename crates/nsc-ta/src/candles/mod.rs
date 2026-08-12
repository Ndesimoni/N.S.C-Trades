//! Spotting candlestick patterns.
//!
//! Eight shapes: pin bar, engulfing, doji, belt-hold, tweezers, inside bar and
//! star — with hammer and inverted hammer being the same shape as a pin bar,
//! pointing one way or the other.
//!
//! **This list is not finished, and it is not meant to be.** The trader reads
//! more shapes than he can name, and the ones without names are supposed to be
//! found later from labelled trades rather than guessed at now. What that
//! needs from this code is the measurements, which every sighting carries.
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
//! - [`pin_bar`], [`doji`], [`engulfing`], [`belt_hold`], [`tweezers`],
//!   [`inside_bar`], [`star`] — one shape each
//! - [`finder`] — asks all five about the newest candle
//! - [`series`] — a whole history at once, for the backtester
//!

mod belt_hold;
mod doji;
mod engulfing;
mod inside_bar;
mod pin_bar;
mod star;
mod tweezers;

mod finder;
mod series;

#[cfg(test)]
mod tests;

pub use finder::look_at;
pub use series::find_patterns;
