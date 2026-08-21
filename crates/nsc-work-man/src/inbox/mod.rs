//! Listen to what he sends the bot, and save it.
//!
//! The other side of Telegram. `telegram/` talks; this listens.
//!
//! **It runs inside the bot**, alongside the watcher. It was a second program
//! for a while, and that meant two terminals and remembering both — and if the
//! inbox was not up, a level he sent went nowhere and nothing said so.
//!
//! Buttons are not set up anywhere — the bot sends them with a message, and
//! tapping one sends that word back as an ordinary message. A button is a
//! shortcut for typing, nothing more.
//!
//! ```text
//!   /level      ->  which pair?      [XAUUSD] [GBPUSD] [+ new pair]
//!   XAUUSD      ->  which timeframe? [Weekly] [Daily] [4-hour]
//!   Weekly      ->  send prices
//!   4520 4000   ->  saved, and it says back what the pair now holds
//! ```
//!
//! **The buttons are the files in `config/pairs/`.** Not a list in this file —
//! that was the mistake the old `settings.rs` made, and two lists disagree in
//! the end.

mod asked;
mod checking;
mod conversation;
mod dropping;
mod hearing;
mod one;
mod pairs;
mod picture;
mod talking;
mod words;

pub use hearing::run;
pub use talking::plainly;
