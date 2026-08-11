//! The typed version of `config/ta.toml`, with sanity checks on load.
//!
//! Parsing happens elsewhere; this module owns the checking. Catching a
//! nonsensical combination here — say a "close to the level" tolerance wider
//! than the level grouping distance, which would make every price count as
//! being at every level — is far cheaper than finding it in a backtest that
//! has already run for an hour.
//!
//! ## Why this crate does not read the file itself
//!
//! `nsc-ta` is not allowed to touch the outside world. So it describes what
//! the settings look like, and whoever starts the program reads the TOML,
//! fills these in, and hands them over.
//!
//! That is also what makes the analysis testable. Hand it made-up settings
//! and check the answers, with no config file anywhere near it.
//!
//! ## Checked once, at startup
//!
//! Call [`TaSettings::validate`] after loading. Checking on every candle
//! would be millions of pointless checks — and finding out at candle 400,000
//! that the lookback is zero is far too late.
//!
//! ## Only what has code behind it
//!
//! `ta.toml` has more sections than this. They get added as the modules that
//! use them are built. Settings written for code that does not exist go stale
//! before anyone reads them.
//!
//! ## What is where
//!
//! - [`settings`] — everything, gathered in one place
//! - [`swings`] — from `[swings]`
//! - [`levels`] — from `[levels]`
//! - [`indicators`] — from `[indicators]`

mod indicators;
mod levels;
mod settings;
mod swings;

#[cfg(test)]
mod tests;

pub use indicators::IndicatorSettings;
pub use levels::LevelSettings;
pub use settings::TaSettings;
pub use swings::SwingSettings;
