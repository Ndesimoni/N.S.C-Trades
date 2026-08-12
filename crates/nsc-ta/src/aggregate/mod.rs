//! Building 1-hour, 4-hour and daily candles out of smaller ones.
//!
//! ## Why build them rather than ask the broker
//!
//! Control over when the day ends. The daily close — 5pm New York by
//! convention, set in `config/app.toml` — decides where every daily level
//! sits. Brokers disagree with each other about it, and a level that does not
//! match the one on your own chart destroys your trust in the bot faster than
//! a losing trade does.
//!
//! Building them also means one answer for every instrument and every feed,
//! instead of whatever each one happened to send.
//!
//! ## The rule this module exists to keep
//!
//! **A part-formed bigger candle is never handed out as finished.**
//!
//! A 4-hour candle made from three 15-minute candles is not a 4-hour candle.
//! Its high and low have not finished happening. Signal on it and you are
//! using prices the market has not printed — and the backtest gets better,
//! not broken, which is what makes it dangerous.
//!
//! So a bigger candle is only marked complete once a smaller candle from the
//! **next** bucket has arrived. Not when the clock says it should have
//! finished: the market can be shut, a feed can be late, and a candle that is
//! merely expected is not a candle that happened.
//!
//! The last bucket in any run of candles therefore comes back unfinished.
//! That is correct, not a gap — more candles may still be coming, and nothing
//! here can know that they are not.
//!
//! ## What is where
//!
//! - [`bucket`] — the smaller candles gathered so far for one bigger candle
//! - [`builder`] — one candle at a time, the way the live bot works
//! - [`series`] — a whole history at once, for the backtester

mod bucket;
mod builder;
mod series;

#[cfg(test)]
mod tests;

pub use builder::Aggregator;
pub use series::aggregate;
