//! The one question every failure has to answer: **try again, or give up?**
//!
//! This is the whole reason the library has named troubles instead of one
//! catch-all error. A bot that cannot tell a bad API key from a dropped line
//! does one of two things, and both are bad:
//!
//! - retries the bad key forever, and it looks exactly like a dead connection
//! - or dies on a hiccup that would have cleared in three seconds
//!
//! It is written down in `CLAUDE.md` and it was ignored until the price
//! watcher made it matter — that is the first thing here that must survive a
//! failure rather than exit on one.

mod answer;
mod kinds;

#[cfg(test)]
mod tests;

pub use answer::{Answer, Knows};
pub use kinds::{CandleError, CardError, FeedError, LevelError, SendError};
