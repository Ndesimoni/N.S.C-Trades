//! Spotting candlestick patterns.
//!
//! One or two candles, objective, easy to define — the reliable half of
//! pattern recognition, and what your entries actually fire on.
//!
//! Every detector follows two rules:
//!   1. Size is judged against a normal candle, never in fixed pips.
//!   2. Where it happens matters. A pin bar in the middle of nowhere is noise;
//!      the same candle at a tested level is a trigger. So detectors report
//!      the pattern and how good it is, and let the rules decide whether the
//!      location justifies acting on it.

pub mod doji;
pub mod engulfing;
pub mod inside_bar;
pub mod pin_bar;
pub mod star;
