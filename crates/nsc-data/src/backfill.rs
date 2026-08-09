//! Downloading price history. Run this before anything else works.
//!
//! Pulls history from whichever broker is configured, in chunks that respect
//! the provider's limits, saving as it goes so an interrupted run picks up
//! where it stopped instead of starting over.
//!
//! How much history: at least two to three years for higher-timeframe
//! strategies. Fewer than a few hundred signals in a backtest is not a sample,
//! it is a story — and the urge to tune against a story is where overfitting
//! begins.
//!
//! Once the small candles are in, the bigger timeframes get built and saved,
//! so settings sweeps do not pay that cost over and over.
