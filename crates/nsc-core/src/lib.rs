//! # nsc-core — the shared vocabulary
//!
//! Types only. No database, no internet, no async, no clock.
//!
//! Every other crate speaks in these types. That stops the code turning into
//! a pile of bare numbers whose meaning depends on which function you happen
//! to be reading. A price is not a distance is not an ATR multiple, and the
//! compiler should refuse to let you mix them up.
//!
//! If you want to add a dependency here, whatever you are building probably
//! belongs in a different crate.

pub mod candle;
pub mod error;
pub mod fib;
pub mod level;
pub mod pattern;
pub mod price;
pub mod session;
pub mod signal;
pub mod structure;
pub mod swing;
pub mod symbol;
pub mod timeframe;
pub mod trendline;
