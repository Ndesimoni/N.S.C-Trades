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

use nsc_core::levels::Timeframe;

/// Only he may write levels.
///
/// **Channel posts carry no sender at all** — Telegram strips it, because a
/// post is from the channel rather than from a person. So the private chat is
/// the only place the bot can tell who is talking.
const OWNER: i64 = 6089491075;

const PAIRS: &str = "config/pairs";
const TIMEFRAMES: [(&str, Timeframe); 3] = [
    ("Weekly", Timeframe::Weekly),
    ("Daily", Timeframe::Daily),
    ("4-hour", Timeframe::H4),
];

const NEW_PAIR: &str = "+ new pair";

/// What he can do to one pair, from its own page.
const ADD: &str = "+ Add levels";
const DROP: &str = "− Take one off";
const STOP: &str = "✗ Stop watching";

/// Backing out.
///
/// **Every keyboard carries it.** Without one the only ways out of a flow are
/// finishing it or sending a command that happens to replace the buttons —
/// and the buttons stay on his screen in the meantime, over his own keyboard,
/// looking like the bot is waiting for something.
const CLOSE: &str = "✗ Close";
const UNDO: &str = "↩ Undo";

/// Stopping a pair takes two taps, not one.
///
/// It throws away every level he has drawn for that pair — months of chart
/// work — and it is done by tapping a button on a phone while doing something
/// else.
const YES: &str = "✓ Yes, stop it";
const NO: &str = "✗ Keep it";

mod asked;
mod conversation;
mod dropping;
mod one;
mod pairs;
mod picture;
mod talking;

pub use talking::plainly;

mod hearing;

pub use hearing::run;
