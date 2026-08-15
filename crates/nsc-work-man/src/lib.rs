//! The parts of the bot that more than one program needs.
//!
//! There are three programs now — the bot itself, `inbox` which listens for
//! levels, and `levels` which draws them. They all need the same candles, the
//! same cards and the same level file, so that work lives here rather than
//! inside any one of them.
//!
//! The programs in `bin/` are thin on purpose: each one is a job, and the job
//! is done with these.

pub mod candle;
pub mod card;
pub mod error;
pub mod feed;
pub mod levels;
pub mod message;
pub mod review;
pub mod settings;
pub mod telegram;
