//! What an instrument is, and the numbers that go with it.
//!
//! Carries `pip_size` and `digits`, because a stop distance means nothing
//! without them. 20 pips on USDJPY and 20 pips on EURUSD are different
//! numbers of decimal places and different amounts of money.
//!
//! Also holds the two currencies, where there are two. The news filter needs
//! them to ask "does this USD announcement affect this instrument?", and
//! `nsc-risk` needs them to spot that four different pairs are really one bet
//! on the dollar.
//!
//! ## Not everything is a currency pair
//!
//! US30 has no base currency. Gold's is a metal. So both currencies are
//! optional, and anything reading them has to cope with their absence rather
//! than assume a pair.
//!
//! That is also why [`AssetClass`] exists: the daily close, the sessions and
//! the spread behaviour all differ between a currency pair, a metal and a
//! stock index, and the code needs to be able to tell them apart.
//!
//! ## What is where
//!
//! - [`class`] — forex, metal, index or energy
//! - [`currency`] — three-letter codes, checked
//! - [`instrument`] — the instrument itself

mod class;
mod currency;
mod instrument;

#[cfg(test)]
mod tests;

pub use class::AssetClass;
pub use currency::Currency;
pub use instrument::Symbol;
