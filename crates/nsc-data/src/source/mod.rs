//! What the bot asks for candles and prices, whoever is answering.
//!
//! **Nothing above this line knows which broker you use.** The watcher asks
//! for "the last 60 weekly candles for EUR/USD" and gets them. Which company
//! answered is decided once, at startup.
//!
//! That is not about keeping a spare feed. It is about where broker details
//! are allowed to live. A websocket address sat inside the price watcher for
//! months, and that one line is why changing feed is a job rather than an
//! edit.

mod candles;
mod interval;
mod price;

#[cfg(test)]
mod tests;

pub use candles::MarketDataSource;
pub use interval::Interval;
pub use price::{Heard, Price, Prices};
