//! Reading a chart.
//!
//! **It describes. It never decides.** "This is a doji" lives here; "this is a
//! buy" lives in `nsc-strategy`. Keep that line and a rule can change without
//! touching a pattern, and a pattern can be added without touching a rule.
//!
//! Three things, in the order they are built:
//!
//! ```text
//!   candle/     what ONE candle is
//!   pattern/    what a RUN of them does — reads the above
//!   indicator/  numbers off the price series
//! ```
//!
//! **Everything is measured in ATR, never in points.** A three-point body is
//! nothing on gold and a week on the euro. Measure in points and it works on
//! the pair you tested and quietly stops working on every other one.
//!
//! There is no feed in here, no clock and no global state — and not by
//! discipline. `Cargo.toml` has no `reqwest` and no `tokio`, so nothing here
//! *can* reach out.
pub mod candle;
pub mod pattern;
