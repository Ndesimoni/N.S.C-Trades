//! Talking to Postgres. SQL lives here and nowhere else.
//!
//! These return proper types from `nsc-core`, never raw database rows, so a
//! change to the tables stays inside this folder.

pub mod backtests;
pub mod candles;
pub mod labels;
pub mod outcomes;
pub mod signals;
