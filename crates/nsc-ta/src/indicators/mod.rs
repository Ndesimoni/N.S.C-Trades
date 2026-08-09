//! The few indicators this system uses. Deliberately boring.
//!
//! You trade off price, so indicators are support, not signal. Their main job
//! here is ATR, which is the yardstick almost every threshold is measured
//! against.
//!
//! All of them update one candle at a time where possible. The backtester runs
//! these millions of times across settings sweeps, and recalculating a whole
//! window every candle is the difference between a sweep that takes seconds
//! and one that eats an afternoon.

pub mod atr;
pub mod moving_average;
pub mod rsi;
