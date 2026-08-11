//! Finding swing highs and lows — the foundation of everything.
//!
//! Spend more time here than anywhere else. Levels, trendlines, Fibonacci
//! anchors, trend direction and chart patterns are all built on this one
//! output. Get it right and most of the engine follows. Get the sensitivity
//! wrong and every feature downstream is quietly rubbish, in a way that is
//! very hard to trace back.
//!
//! ## How it works
//!
//! A swing high is a candle whose high beats the highs of a few candles on
//! either side. Same idea upside-down for lows. Small moves get filtered out
//! by requiring the swing to stand out by a fraction of a normal candle, so
//! that choppy noise does not register as structure.
//!
//! ## Confirmation — read this before changing anything
//!
//! A swing at candle 100 is not knowable until candle 103 has printed. So
//! every swing is tagged with `confirmed_at`, and callers must respect it.
//!
//! Feeding an unconfirmed swing into level detection is the easiest possible
//! way to produce a beautiful backtest you cannot trade.
//!
//! Here that is not a rule to remember — it is how the code works. The
//! finder cannot decide about a candle until it has seen the candles after
//! it, so an unconfirmed swing never exists to be misused.
//!
//! ## What is where
//!
//! - [`finder`] — one candle at a time, the way the live bot works
//! - [`series`] — a whole history at once, for the backtester
//!
//! Read `README.txt` for the two decisions inside the detector and what they
//! cost.

mod finder;
mod series;

#[cfg(test)]
mod tests;

pub use finder::SwingFinder;
pub use series::find_swings;
