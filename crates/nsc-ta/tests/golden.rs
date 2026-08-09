//! Golden-file tests for the chart-reading engine.
//!
//! Runs each saved candle file through swing detection, level building,
//! trendline fitting, Fibonacci anchoring and trend reading, then compares
//! against the saved answer.
//!
//! ## The test that matters most
//!
//! **One candle at a time must give the same answer as all at once.**
//!
//! Feed the candles in one by one, the way the live bot does. The result must
//! be byte-for-byte identical to processing the whole series in one go.
//!
//! Any difference means data from the future leaked in. This one check is the
//! cheapest possible detector for it, and it catches the whole category of bug
//! before the backtester is even involved.
//!
//! ## Also checked here
//!
//! - no swing is reported before the candle that confirmed it
//! - unfinished candles are ignored completely
//! - the same setup on EURUSD and GBPJPY scores the same — which is what
//!   proves the settings really are measured in ATR and not pips in disguise
