//! Layer 2 — is price somewhere that matters?
//!
//! Checks how close price is to support/resistance, Fibonacci zones and
//! trendlines, and requires a minimum number of them to agree.
//!
//! "Close" is measured against normal candle size. This is the layer where
//! something your eye judges instantly has to become a number — and where a
//! fixed pip tolerance would quietly stop working the moment you added a
//! second pair.
//!
//! Worn-out levels get refused here when configured. A zone tested five times
//! is not five times stronger. It is more likely to break.
