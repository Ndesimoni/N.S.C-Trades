//! What a RUN of candles does.
//!
//! One candle at a time is `candle/`. This is two or three of them together.
//!
//! ```text
//!     named.rs    Pattern — what there is to find
//!     rules.rs    the thresholds, out of config/patterns.toml
//!     body.rs     a candle's body as PRICES, not as a share
//!     two.rs      engulfing, harami, tweezers, piercing, dark cloud
//!     three.rs    the star, and the abandoned baby inside it
//!     finding.rs  the one way in, and the order they are tested
//! ```
//!
//! **It describes. It never decides.** "This is a bullish engulfing" lives
//! here; "this is a buy" lives in `nsc-strategy`.

mod body;
mod finding;
mod named;
mod rules;
mod three;
mod two;

#[cfg(test)]
mod tests;

pub use finding::ending_at;
pub use named::Pattern;
pub use rules::{Engulfing, Harami, Piercing, Rules, RulesError, Star, Tweezers, load};
