//! Tests for the candlestick detectors.
//!
//! - [`helpers`] — candles 100 tall, so every share is a percentage
//! - [`shapes`] — does each detector find its own shape
//! - [`guards`] — does it refuse what it should refuse

mod guards;
mod helpers;
mod shapes;
