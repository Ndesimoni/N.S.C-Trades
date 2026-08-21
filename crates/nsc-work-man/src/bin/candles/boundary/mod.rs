//! Finding where a feed starts its day, without asking it.
//!
//! ```text
//!     lined.rs      Lined — one big candle, and the small one it started on
//!     matching.rs   lining them up, and whether they agree on an answer
//! ```

mod lined;
mod matching;

#[cfg(test)]
mod tests;

pub use lined::Lined;
pub use matching::{agreed_on, hour_of, line_up, voted, weekday_of};
