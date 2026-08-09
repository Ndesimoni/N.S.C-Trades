//! Chart patterns — Phase 1.5, deliberately left until later.
//!
//! Head and shoulders, triangles, double tops and flags are the hardest
//! patterns to define in code and the weakest statistically. Trend direction
//! plus levels plus Fibonacci plus candlesticks gives you most of the edge for
//! a fraction of the work, so build that first and prove it.
//!
//! When these do get built, they are matched against the **sequence of swing
//! points**, not against raw candles. Matching raw prices ends in an
//! unmaintainable pile of special cases.
//!
//! Every detector must report a confidence, and may only use swings that were
//! already confirmed at the candle being analysed. A head and shoulders is not
//! a head and shoulders until the right shoulder is confirmed, however obvious
//! it looks looking back.

pub mod double;
pub mod flag;
pub mod head_shoulders;
pub mod triangle;
