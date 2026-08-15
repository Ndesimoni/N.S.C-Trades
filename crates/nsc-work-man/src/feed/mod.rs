//! Asking Twelve Data for candles.
//!
//! One request, one answer. The live price stream is a separate thing.
//!
//! **The candle is never computed.** It comes from the feed finished, exactly
//! as it appears on his chart. Building one out of smaller candles or out of
//! ticks would produce something close to the broker's, never the same, and
//! then nobody could say which was right.

mod ask;
mod error;

#[cfg(test)]
mod tests;

pub use ask::for_pair;
pub use error::FeedError;
