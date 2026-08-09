//! `BarClosed` — where the backtester and the live bot meet.
//!
//! ```text
//!   backtester ─┐
//!               ├─→ "candle closed: pair, timeframe, prices" ─→ everything else
//!   live bot   ─┘
//! ```
//!
//! The backtester replays these from the database as fast as it can. The live
//! bot builds them from the broker feed. Everything downstream cannot tell
//! which one it is talking to, and does not need to.
//!
//! This is the mechanism behind the project's main rule. If any code ever asks
//! "am I backtesting?", the backtest has stopped describing the live bot and
//! this join has been broken.
