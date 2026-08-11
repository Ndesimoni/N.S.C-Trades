//! Finding support and resistance, by grouping swing points that sit close
//! together.
//!
//! ## What it does
//!
//! Take the swing highs and lows from the recent past. Wherever several of
//! them sit within one band's thickness of each other, that is a level.
//!
//! The thickness is fixed for the whole timeframe — half a normal candle by
//! default. The band gets **slid** up and down to catch the most touches. It
//! is never **stretched** to reach one more.
//!
//! That is how these get drawn by hand, and the difference matters. A band
//! that stretches keeps growing until it is wide enough to contain half the
//! chart, and then every price is at every level.
//!
//! ## Highs and lows go in the same pot
//!
//! A price that stopped a fall in March and capped a rally in June is one
//! level that has been tested twice, not two levels that happen to share a
//! price. So swing highs and swing lows are grouped together.
//!
//! ## What it does not do
//!
//! It does not say whether a level will hold or break. It reports the band,
//! the timeframe, the touch count and the dates. Whether that is a level
//! worth trading is a judgement for `nsc-strategy`, and it needs the trend,
//! the candle and the higher timeframe as well.
//!
//! ## The lookahead rule
//!
//! A level is only knowable once its last touch has confirmed as a swing —
//! a few candles after the candle that touch sits on. `Level::new` refuses
//! anything else, so a level built too early cannot be created at all.
//!
//! ## What is where
//!
//! - [`grouping`] — sliding one band to catch the most swing points
//! - [`finder`] — the whole job: age, thickness, grouping, levels out
//!
//! Read `README.txt` for the decisions inside the grouping and what they
//! cost.

mod finder;
mod grouping;

#[cfg(test)]
mod tests;

pub use finder::find_levels;
