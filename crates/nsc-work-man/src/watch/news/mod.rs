//! Telling him what is about to print.
//!
//! ```text
//!     holding.rs   what it has downloaded, and what it has already said
//!     saying.rs    drawing the card and sending it
//! ```
//!
//! **It runs on its own, beside the price watcher.** It needs no prices, no
//! bands and no IBKR — only the clock and the internet. So it is spawned once
//! at startup like the inbox, rather than living inside the price loop, which
//! blocks for hours at a time waiting on the socket.
//!
//! ## It fails quiet, never loud
//!
//! The calendar going down must not stop the bot. Every failure here is
//! reported to the terminal and the watcher carries on with whatever it
//! downloaded last — the same rule the AI layer has, and for the same reason:
//! a bot that went silent because a free web page hiccupped is worse than one
//! that did not know about the news.

mod holding;
mod saying;

pub use holding::run;
