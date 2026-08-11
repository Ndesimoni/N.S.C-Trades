//! Average True Range — the most important number in this codebase.
//!
//! ATR is how big a normal candle is right now. It predicts nothing. It
//! matters because it is the **yardstick**.
//!
//! How close counts as "at the level", how much room the stop gets, how big a
//! candle is too big to chase — all measured in ATR. That is what lets one
//! settings file work on EURUSD and gold at the same time.
//!
//! Read `indicators/README.txt` for how true range and the smoothing work,
//! and why the smoothing choice decides whether our numbers match your chart.
//!
//! ## What is where
//!
//! - [`running`] — ATR kept up to date as candles arrive, one at a time
//! - [`series`] — ATR for a whole history at once
//!
//! The second runs the candles through *the same* struct as the first. Not
//! similar code — the same code. That is what makes "one at a time" and "all
//! at once" give identical answers, instead of something we have to keep
//! testing and hoping about.

mod running;
mod series;

#[cfg(test)]
mod tests;

pub use running::Atr;
pub use series::atr_series;
