//! Real gold candle runs, in the shapes that made a pattern.
//!
//! ```text
//!     making.rs    turning written-down prices back into candles
//!     pairs.rs     runs of two
//!     triples.rs   runs of three
//! ```

mod making;
mod pairs;
mod triples;

pub(super) use making::{normal_2024, normal_2026};
pub(super) use pairs::*;
pub(super) use triples::*;
