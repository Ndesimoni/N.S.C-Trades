//! Real gold candle runs, in the shapes that made a pattern.
//!
//! ```text
//!     making.rs    turning written-down prices back into candles
//!     pairs.rs     runs of two
//!     triples.rs   runs of three
//!     pushes.rs    runs that make -- or just miss -- HIS own pattern
//! ```

mod making;
mod pairs;
mod pushes;
mod triples;

pub(super) use making::{normal_2024, normal_2026};
pub(super) use pairs::*;
pub(super) use pushes::*;
pub(super) use triples::*;
