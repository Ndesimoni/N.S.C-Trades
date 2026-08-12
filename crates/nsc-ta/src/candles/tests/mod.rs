//! Tests for the candlestick detectors.
//!
//! - [`helpers`] — candles 100 tall, so every share is a percentage
//! - [`one_candle`] — the shapes made from a single candle
//! - [`several_candles`] — the ones that need two or three
//! - [`guards`] — does it refuse what it should refuse

mod guards;
mod helpers;
mod one_candle;
mod several_candles;
