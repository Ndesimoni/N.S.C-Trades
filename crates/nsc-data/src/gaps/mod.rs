//! Checking a history is sound before anything is built on it.
//!
//! Bad data does not fail loudly. A missing hour shifts a swing high, which
//! shifts a level, which changes every signal after it — and the backtest
//! still finishes and prints a perfectly believable number.
//!
//! There are two different faults here, and they are not the same thing.
//!
//! ## A candle that is missing from the file
//!
//! The broker lost data, or the export was cut short. Apart from the weekend
//! break this is always a fault, and `holes.rs` finds it.
//!
//! A 4-hour candle built from twelve 15-minute candles instead of sixteen has
//! the wrong high and the wrong low, and it still looks like a normal candle.
//!
//! ## A candle that is there but never moved
//!
//! Open, high, low and close all at the same price. This is **not** missing
//! data — the market was open and nothing traded, and the broker printed the
//! candle correctly. `flat.rs` finds these.
//!
//! They matter because the analysis reads them as real turns. This project has
//! already been bitten once: two flat candles invented a swing at the left edge
//! of every history, and the tests were green throughout.
//!
//! ## What this folder does not do
//!
//! It reports. It does not delete, repair or fill anything in.
//!
//! A repaired candle is a made-up candle, and the made-up ones are
//! indistinguishable from real ones a week later. Deciding what to do about a
//! hole is a decision, and decisions get made by the person, not the scan.

mod flat;
mod holes;

#[cfg(test)]
mod tests;

pub use flat::{FlatRun, find_flat_runs};
pub use holes::{Hole, Reason, find_holes};
