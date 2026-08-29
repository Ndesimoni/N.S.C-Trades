//! Rung 2 — what a candle did at a zone, once it has finished.
//!
//! **Only pairs with price at a zone are ever fetched.** A quiet week costs
//! nothing. That is the same principle that killed the design where a candle
//! was fetched on every close on every pair whether anything had happened or
//! not.
//!
//! ## It never works out when a candle closes
//!
//! It asks, every ten minutes, for the newest candle and lets the feed's own
//! stamp say whether that is one it has already reported.
//!
//! Working out the boundaries here would mean knowing where the feed puts its
//! 4-hour candles, which nobody has measured. Guessing wrong reports a candle
//! that has not happened, and that is the mistake that makes results look
//! better rather than broken.
//!
//!   due.rs      when a pair's next candle is worth asking about
//!   said.rs     what one report was about — pair, interval, kind, zone
//!   look.rs     the ten-minute check, and what it decides to ask about
//!   report.rs   saying what a candle did, one zone at a time
//!   fetch.rs    asking the feed, and never letting that end the run
//!   setups.rs   RUNG 3 — a shape he trades, at a level he drew

mod due;
mod fetch;
mod look;
mod report;
mod said;
mod setups;

#[cfg(test)]
mod tests;

pub use look::Closes;
pub use setups::settings;
