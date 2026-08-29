//! Downloading the week's economic calendar.
//!
//! **The judgement lives in `nsc-core::news`.** This is only the reaching:
//! ask for the file, check the answer is really the file, turn it into
//! events. What counts as due, and what shares a card, is decided there.
//!
//! ```text
//!     error.rs   what went wrong, and whether another go would help
//!     parse.rs   the feed's own shape, turned into nsc-core events
//!     feed.rs    the asking, and the refusal that arrives looking fine
//! ```
//!
//! ## Why not IBKR
//!
//! IBKR's API carries news **headlines** — six calls, all of them articles
//! from a provider. It has no macro calendar at all: no rate decisions, no
//! payrolls, nothing scheduled with a time on it. That is a different product
//! and they do not sell it through the API.
//!
//! So this is a second source, and it is the one thing in this crate that
//! does not come from the broker.

mod error;
mod feed;
mod parse;

#[cfg(test)]
mod tests;

pub use error::CalendarError;
pub use feed::fetch;
pub use parse::{Parsed, read};
