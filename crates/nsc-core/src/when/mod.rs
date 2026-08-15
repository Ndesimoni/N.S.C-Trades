//! When the bot is allowed to speak.
//!
//! **The trading week is not the calendar week.** It opens Sunday 17:00 New
//! York, so the session everyone calls Monday runs from Sunday evening to
//! Monday evening.
//!
//! Read a moment off the UTC calendar instead and Monday's silence lands three
//! hours into Tuesday's session and misses Sunday evening altogether. Nothing
//! errors — it is just wrong, every week.
//!
//! ## Nothing here reads the clock
//!
//! Every function is handed `now`. That is what keeps this crate usable by the
//! backtester: the same rules run over 2019 by passing 2019 in, with no "if we
//! are backtesting" anywhere.
//!
//! ## Three states, not two
//!
//! "Do not trade" and "do not speak" are different things, and collapsing them
//! would either silence a day he wants to watch or suggest trades in the hours
//! he never takes them.

mod allow;
mod beat;
mod rules;
mod session;

#[cfg(test)]
mod tests;

pub use allow::{Allowed, allowed};
pub use beat::{beat_due, beat_words};
pub use rules::{Rules, WhenError, load};
pub use session::{into_day, opened, trading_day};
