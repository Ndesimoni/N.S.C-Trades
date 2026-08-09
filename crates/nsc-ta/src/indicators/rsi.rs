//! RSI, used mainly to spot divergence.
//!
//! Overbought and oversold readings are not used as signals. The useful part
//! is divergence: price making a higher high while RSI does not, at a level.
//! That is one of the scored extras in `config/strategy.toml`.
//!
//! Divergence is measured between **confirmed** swing points, never between
//! arbitrary candles. Divergence drawn to a swing that is not confirmed yet is
//! using data you do not have.
