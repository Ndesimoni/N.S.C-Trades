//! Finding swing highs and lows — the foundation of everything.
//!
//! Spend more time here than anywhere else. Levels, trendlines, Fibonacci
//! anchors, trend direction and chart patterns are all built on this one
//! output. Get it right and most of the engine follows. Get it wrong and every
//! feature downstream is quietly rubbish, in a way that is very hard to trace
//! back.
//!
//! ## What makes a peak a peak
//!
//! Not the number of candles either side of it. That is the usual way of
//! finding swings and it asks the wrong question — a lazy rounded top with
//! twenty quiet candles around it passes, and a sharp turn with four candles
//! around it fails.
//!
//! The question is what price did afterwards. A peak counts once the market
//! has **given back half of the run that made it**. Half of that particular
//! move, so a 300-point rally needs about 150 back and a 60-point rally needs
//! about 30.
//!
//! A shallower give-back counts too, but only once price has taken the peak
//! out — because the strongest trends barely pause, and a rule that only
//! confirmed on depth would go blind exactly where structure matters most.
//!
//! And a run has to be worth calling a run: at least a share of the biggest of
//! the last few. Otherwise half of a tiny move is a tinier pullback, and a
//! quiet afternoon fills with swings that are really just noise.
//!
//! ## Confirmation — read this before changing anything
//!
//! Every swing is tagged with `confirmed_at`, and callers must respect it.
//!
//! Here that is not a rule to remember, it is how the code works. Nothing can
//! be called a swing until the candles that prove it have closed, so an
//! unconfirmed swing never exists to be misused.
//!
//! ## What is where
//!
//! - [`run`] — a price with its candle, and what share of a move is what
//! - [`memory`] — the recent runs, and whether the next one is big enough
//! - [`leg`] — one move in one direction, and the two ways it ends
//! - [`seed`] — the start of a history, before any direction is known
//! - [`finder`] — one candle at a time, the way the live bot works
//! - [`series`] — a whole history at once, for the backtester
//!
//! Read `README.txt` for the decisions inside the finder and what they cost.

mod direction;
mod leg;
mod memory;
mod run;
mod seed;
mod step;

mod finder;
mod series;

#[cfg(test)]
mod tests;

pub use finder::SwingFinder;
pub use series::find_swings;
