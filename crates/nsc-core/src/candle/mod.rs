//! One candle, and groups of them.
//!
//! A candle is four prices and a time. The hard part is not the data, it is
//! the two rules attached to it.
//!
//! **`open_time` is when the candle STARTED**, always in UTC. Not when it
//! ended. Store the close time instead and everything shifts by one bar.
//! It still looks right, which is what makes it so hard to find.
//!
//! **`complete` says whether the candle has finished.** An unfinished candle
//! is the one on the right of a live chart that keeps moving. Its high and
//! low have not happened yet. Read one and you are using prices the market
//! has not printed — the candle you acted on is not the candle that ends up
//! in the history.
//!
//! ## What is where
//!
//! - [`bar`] — one candle
//! - [`series`] — a run of candles for one instrument

mod bar;
mod series;

#[cfg(test)]
mod tests;

pub use bar::Candle;
pub use series::CandleSeries;
