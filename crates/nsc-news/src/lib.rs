//! # nsc-news — economic calendar, headlines, and news blackouts
//!
//! You trade 90% off the chart, so news has exactly one job here: **stopping
//! trades, never starting them.** No signal is ever created because of a
//! headline. The calendar only cancels setups that were otherwise fine.
//!
//! That one-way rule is deliberate. Reading the news is where an automated
//! system is weakest and most likely to invent a story. Refusing to trade
//! into a release you already knew about is where it is strongest — that
//! decision needs a clock, not judgement.
//!
//! Phase 5. Leave `news_filter = false` until the chart-reading half is
//! proven, otherwise you cannot tell which part changed your results.

pub mod blackout;
pub mod calendar;
pub mod classify;
pub mod error;
pub mod headlines;
pub mod sources;
