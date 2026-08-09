//! Things that can go wrong, as types.
//!
//! The clean crates return errors instead of crashing. The backtester runs
//! this code across years of candles, so a crash on one bad candle would kill
//! an entire test run. Bad input should be reported and skipped, not fatal.
