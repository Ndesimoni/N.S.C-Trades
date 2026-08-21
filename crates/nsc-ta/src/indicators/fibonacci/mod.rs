//! Fibonacci retracements over a move.
//!
//! **The ratios are the easy part. Which move you measure is the whole game** —
//! the same ratios drawn from a different pair of points give completely
//! different prices.
//!
//! So the move itself is what gets stored, and the levels are worked out from
//! it. When a Fibonacci reading looks wrong, the move it picked is nearly
//! always the disagreement, and an argument about a move is one you can settle
//! by looking at a chart.
//!
//! ```text
//!     leg.rs      Leg — the move, and the two questions you ask of it
//!     rules.rs    which shares matter, and what job each one has
//!     reading.rs  where price sits in the move, and the lines it draws
//! ```
//!
//! **It does not know where the move came from.** Swings will anchor it one
//! day; his own drawn levels can anchor it today. Neither belongs in here.

mod leg;
mod reading;
mod rules;

#[cfg(test)]
mod tests;

pub use leg::Leg;
pub use reading::{Where, levels, read, targets};
pub use rules::{Rules, RulesError, load};
