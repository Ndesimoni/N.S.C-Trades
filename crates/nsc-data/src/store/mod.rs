//! The record — what the bot decided, and what the market did about it.
//!
//! ```text
//!     error.rs    what can go wrong, and whether to try again
//!     pool.rs     opening it, and running the migrations
//!     candles.rs  the history everything else is measured against
//! ```
//!
//! ## SQL lives here and nowhere else
//!
//! Every query hands back an `nsc-core` type, never a raw row. A table can
//! change shape and the change stops inside this folder.
//!
//! **`nsc-core` and `nsc-strategy` never touch this.** Neither has `sqlx` in
//! its manifest, and it is the manifest that enforces it rather than a rule
//! somebody remembers. A rule that needs a row gets handed the row — which is
//! what lets the backtester and the live bot run the same analysis.
//!
//! ## It is a record, not a cache
//!
//! What price is doing right now belongs in memory. What was **decided**, and
//! what happened next, belongs here and cannot be recreated afterwards.
//!
//! Design: `docs/worksheets/database.md`.

mod candles;
mod error;
mod news;
mod pool;

#[cfg(test)]
mod tests;

pub use candles::{count, newest, oldest, read, write};
pub use error::StoreError;
pub use news::{between as news_between, count as news_count, write as news_write};
pub use pool::{Store, open};
