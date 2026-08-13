//! The levels the trader drew himself.
//!
//! Read from `config/levels/<PAIR>.toml`, one file per instrument.
//!
//! ## Why these are not found by the code
//!
//! `nsc-ta::levels` can find levels, and it was tried against his own chart on
//! six years of gold. The best any setting managed was four of his eight, and
//! three of them could never be found at all.
//!
//! The reason turned out not to be a wrong number. His levels are where a big
//! move **ended** — a crash that stopped dead, a rally that ran out. The finder
//! looks for prices where several swings **cluster**. Those are different
//! definitions, and no band width bridges them.
//!
//! So the bot trades his levels. The finder keeps running, but only so it can
//! be scored against them.
//!
//! ## What is in the file
//!
//! A band, a timeframe, and the day it was drawn. No touch count — he drew it
//! because a move ended there, and that has no count. Asking a hand-drawn
//! level for its touches gives `None` rather than a made-up number.
//!
//! ## Every level runs forward only
//!
//! `from` is the day it was drawn, and the level does not exist before it.
//!
//! A level drawn today knows what price did last year. Letting it act on last
//! year's candles would make a backtest look better than anything that could
//! have been traded — the one mistake this whole design exists to prevent.
//!
//! ## What is where
//!
//! - [`file`] — the shape of the file on disk
//! - [`read`] — reading it, and turning it into levels

mod file;
mod read;

#[cfg(test)]
mod tests;

pub use file::{DrawnLevel, LevelsFile};
pub use read::{Thickness, read_all_levels, read_levels};
