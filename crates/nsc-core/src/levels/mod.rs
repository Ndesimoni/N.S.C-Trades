//! The levels he drew, and the bands they become.
//!
//! **A level is a band, not a line.** Price does not stop at a number, it
//! turns somewhere near one — so "price is at the level" means price is
//! *inside* a band.
//!
//! He draws a line and reads off one price. The thickness comes from
//! `config/levels.toml` and is the same on every pair: 0.35 of a weekly candle
//! for a weekly level, 0.46 of a daily one for a daily.
//!
//! Not a price, a share of a candle. Written as a price it would be right on
//! one pair and absurd on the rest — 78 points is a normal week on gold and
//! about a year on EURUSD.

mod alert;
mod band;
mod close;
mod error;
mod naming;
mod read;
mod watch;
mod write;

#[cfg(test)]
mod tests;

pub use alert::{News, caption, closed_caption, gap, happening, happening_words, note};
pub use band::{Band, Timeframe};
pub use close::{Action, AtZone, action, gapped_in, how_deep, what_it_did};
pub use error::LevelError;
pub use naming::{digits_for, with_slash};
pub use read::{Pair, Thickness, load_pair, load_thickness};
pub use watch::{Nearness, Watch, nearness};
pub use write::{known, save, undo};
