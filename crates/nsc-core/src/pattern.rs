//! Names for candlestick and chart patterns.
//!
//! Two separate lists, kept apart on purpose:
//!
//!   `CandlePattern` — engulfing, pin bar, inside bar, doji, star. One or two
//!   candles, objective, easy to define, and reliable enough to enter on.
//!
//!   `ChartPattern` — head and shoulders, triangles, double tops, flags. Many
//!   swings, much more subjective, and much weaker statistically. Deliberately
//!   pushed to Phase 1.5, because trend direction plus levels plus Fibonacci
//!   plus candlesticks gives you most of the edge for a fraction of the work.
//!
//! Every detection carries a confidence and the candles it covers, so a signal
//! can explain itself.
