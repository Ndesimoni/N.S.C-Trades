//! Working out the trend, and spotting when it is proved.
//!
//! Higher highs and higher lows means uptrend. Lower highs and lower lows
//! means downtrend. Both sentences are counted off swing points, which is why
//! this sits on top of [`crate::swings`].
//!
//! ## Taking an old high out is not enough
//!
//! Price crossing an old high proves nothing on its own. It has to cross it
//! **and carry a share of the run that made it** past — measured from the
//! high, not from where the pullback began.
//!
//! Poke through by a few points and stall, and the high was touched, not
//! taken. That is the most common trap on a chart: it looks like a breakout,
//! it pulls buyers in, and price turns straight back down. Without this rule a
//! bot reads it as a higher high, calls the trend intact, and goes looking for
//! a long at the worst possible moment.
//!
//! Lower lows work the same way, mirrored. One rule with a direction passed
//! in, because an uptrend and a downtrend judged by slightly different rules
//! is how a bot ends up bullish and bearish about the same chart on the same
//! day.
//!
//! ## Measured as a share of the run, not in normal candles
//!
//! `ta.toml` used to ask for a fixed fraction of a normal candle. The share of
//! the run is the better yardstick: a 200-point rally and a 20-point drift are
//! different events, and what counts as real follow-through differs with them
//! rather than with how big candles happen to be this week.
//!
//! ## What is where
//!
//! - [`reader`] — one candle at a time, the way the live bot works
//! - [`series`] — a whole history at once, for the backtester
//!
//! Read `README.txt` for what is deliberately missing.

mod reader;
mod series;
mod watch;

#[cfg(test)]
mod tests;

pub use reader::StructureReader;
pub use series::read_structure;
