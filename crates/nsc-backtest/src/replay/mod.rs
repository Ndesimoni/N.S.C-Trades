//! Replaying a history the way the live bot would have received it.
//!
//! The backtester's job is not to read a file. It is to hand the analysis the
//! same sequence of events the bot would have seen, in the same order, so that
//! a result from history describes what the bot actually does.
//!
//! ## What it does that a loop does not
//!
//! A 15-minute candle at 16:45 also finishes the 4-hour candle that began at
//! 13:00. The bot learns both at that moment. So the replay builds the bigger
//! timeframes as it goes and emits a [`BarClosed`] for each one that finishes,
//! rather than only walking the file.
//!
//! [`BarClosed`]: nsc_data::events::BarClosed
//!
//! ## Biggest timeframe first
//!
//! When several finish together, the bigger ones come out first. Smaller
//! timeframes read the bigger ones for context, so the context has to be fresh
//! before anything reads it.
//!
//! Get that backwards and the same candles give different answers — and they
//! would differ between the backtester and the bot, which is the one thing
//! this design exists to prevent.
//!
//! ## What is where
//!
//! - [`walker`] — feeding candles in, one at a time

mod walker;

#[cfg(test)]
mod tests;

pub use walker::Replay;
