//! `BarClosed` — where the backtester and the live bot meet.
//!
//! ```text
//!   backtester ─┐
//!               ├─→ "candle closed: pair, timeframe, prices" ─→ everything else
//!   live bot   ─┘
//! ```
//!
//! The backtester replays these from a file or the database as fast as it can.
//! The live bot builds them from the broker feed. Everything downstream cannot
//! tell which one it is talking to, and does not need to.
//!
//! This is the mechanism behind the project's main rule. If any code ever asks
//! "am I backtesting?", the backtest has stopped describing the live bot and
//! this join has been broken.
//!
//! ## Two things the event guarantees
//!
//! **The candle is complete.** [`BarClosed::new`] refuses a half-formed one,
//! so nothing downstream has to remember to check. An unfinished candle's high
//! and low have not happened yet, and using one does not error — it just makes
//! the results better than anything tradeable.
//!
//! **There is one answer to "what time is it".** [`BarClosed::at`] is the
//! moment to pass to every `is_known_at` check. Without a single source for
//! that, two pieces of code eventually disagree by one candle, and one of them
//! is reading the future.
//!
//! ## What is where
//!
//! - [`bar_closed`] — the event itself

mod bar_closed;

#[cfg(test)]
mod tests;

pub use bar_closed::BarClosed;
