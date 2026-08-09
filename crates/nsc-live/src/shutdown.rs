//! Shutting down cleanly.
//!
//! On a stop signal: stop accepting new candles, let work in progress finish,
//! save anything pending, close connections.
//!
//! The specific thing this protects: a signal that went to Telegram but was
//! not saved yet. On restart the bot would look at that candle again, find no
//! record of having sent it, and send it a second time.
//!
//! Duplicate signals are the fastest way to lose confidence in a bot, and they
//! are completely avoidable here.
