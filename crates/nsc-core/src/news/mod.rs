//! What is coming up on the economic calendar.
//!
//! **A level in front of a rate decision is not the same level as one on a
//! quiet Thursday.** This is the part of the bot that knows which it is.
//!
//! ```text
//!     impact.rs   how hard an event is expected to hit
//!     event.rs    one event — what, whose currency, and when
//!     due.rs      THE WINDOW — is it worth saying something yet
//!     away.rs     how long until it prints, in words
//!     span.rs     today, or the whole week -- what /news asks for
//!     rules.rs    the settings, read out of config/news.toml
//! ```
//!
//! ## Nothing here reads the clock, and nothing here fetches
//!
//! Every function is handed `now`, exactly like `when/`. Downloading the
//! calendar happens in `nsc-data::news`, because this crate cannot reach
//! anything and that is the point of it.
//!
//! ## Why a window and not "is it soon"
//!
//! An event earns a message between the widest `warn_at_minutes` before it and
//! `stale_minutes` after. The far edge is the one that matters: without it,
//! a bot restarting at two in the afternoon finds a file full of this
//! morning's releases and sends every one of them at once.

mod away;
mod due;
mod event;
mod impact;
mod rules;
mod span;

#[cfg(test)]
mod tests;

pub use away::away_words;
pub use due::{due, due_at, minutes_until, together};
pub use event::Event;
pub use impact::Impact;
pub use rules::{NewsError, Rules, load};
pub use span::{Span, printed, within};
