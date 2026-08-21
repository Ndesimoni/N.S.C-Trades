//! One swing high or low.
//!
//! **Swings sit under everything else.** Levels, trendlines, Fibonacci anchors
//! and trend direction are all counted off them, which is why the type lives
//! here in `nsc-core` rather than beside the code that finds them.
//!
//! ```text
//!     kind.rs    SwingKind — high or low
//!     point.rs   Swing — the swing itself, and its TWO times
//!     error.rs   the one thing that can be wrong: known too soon
//! ```
//!
//! **The two times are the whole point.** Where a swing sits and when it could
//! first be known are different moments, and a swing refuses to exist if the
//! second is not after the first.

mod error;
mod kind;
mod point;

#[cfg(test)]
mod tests;

pub use error::SwingError;
pub use kind::SwingKind;
pub use point::Swing;
