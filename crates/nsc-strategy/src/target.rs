//! Layer 5 — where the trade gets out.
//!
//! Options: a fixed multiple of your risk, the next level in the way, or a
//! Fibonacci extension.
//!
//! This choice moves your win rate more than almost anything else, and it is
//! the easiest thing to test once written down — same entries, same stops,
//! three target methods, three sets of numbers. Run that comparison early. It
//! is the most valuable backtest available in Phase 2.
//!
//! Also works out the partial-exit and break-even plan. Version 1 reports that
//! plan rather than doing it, but it has to be recorded, so that the tracker
//! measures the trade you would actually have managed instead of a naive
//! hold-until-target.
