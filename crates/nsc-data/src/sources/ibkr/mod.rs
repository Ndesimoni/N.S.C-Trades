//! Interactive Brokers.
//!
//! Connecting to TWS or IB Gateway, asking what the account holds, fetching
//! candles, and holding the live price line open.
//!
//! **This feed needs a program running.** Unlike a cloud API, nothing here
//! works unless TWS or IB Gateway is logged in and reachable. That is the
//! whole cost of using it, and it is paid at deploy time rather than in code.

mod candles;
mod connect;
mod contract;
mod error;
mod serves;
mod ticks;

#[cfg(test)]
mod tests;

pub use connect::IbkrConnection;
pub use error::IbkrError;
pub use serves::Serves;
