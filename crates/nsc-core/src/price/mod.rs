//! Prices, pips, and distances measured in ATR.
//!
//! These are separate types rather than plain numbers, on purpose. The bug
//! this prevents is adding a price to a distance, or comparing a pip value
//! against an ATR multiple. Both look fine to the compiler if everything is
//! just `f64`, and both produce silently wrong stops.
//!
//! Converting between pips and ATR multiples happens here. Almost every
//! threshold in this system is in ATR rather than pips, because a pip value
//! that works on one pair stops working on the next one.
//!
//! ## What is where
//!
//! - [`point`] — `Price`, a point on the chart
//! - [`distance`] — the three ways of measuring a gap between two prices
//! - [`ops`] — what you are allowed to add and subtract, and what you are not

mod distance;
mod ops;
mod point;

#[cfg(test)]
mod tests;

pub use distance::{AtrMultiple, Pips, PriceDistance};
pub use point::Price;
