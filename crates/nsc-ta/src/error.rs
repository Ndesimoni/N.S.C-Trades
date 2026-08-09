//! Things that can go wrong while reading the chart.
//!
//! Typed errors, not a catch-all. The backtester runs this code over years of
//! candles inside a settings sweep, and it needs to tell apart "not enough
//! candles yet to work out ATR" — skip this one and carry on — from "these
//! settings make no sense" — stop everything.
//!
//! A vague error forces the caller to treat both the same way, which means
//! either killing sweeps over harmless gaps or silently swallowing real
//! mistakes.
//!
//! Nothing here crashes. A bad candle in year three must not destroy the two
//! hours of work before it.
