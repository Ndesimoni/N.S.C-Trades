//! `TaSnapshot` — everything the bot knows at one candle close.
//!
//! The single handover point between reading the chart and applying rules.
//! `nsc-strategy` reads only this; it never goes back to the raw candles.
//! That boundary is what lets the rules be tested against a handful of made-up
//! values instead of years of price history.
//!
//! Contains, for every timeframe in play: confirmed swings, active levels,
//! valid trendlines, the current Fibonacci set, trend direction, detected
//! patterns, indicator values, and market context.
//!
//! It is also the training data. A snapshot gets saved onto the signal row and
//! becomes one example for the Phase 4 model. Saving it instead of working it
//! out again later is deliberate — recalculating against updated chart-reading
//! code would train the model on inputs the live bot never produced.
