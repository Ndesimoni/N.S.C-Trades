//! Where prices come from.
//!
//! `source` is what the bot asks. `sources` is who answers.
//!
//! `news` is the one that is not the broker at all — IBKR has
//! neither an economic calendar nor a research consensus.

pub mod news;
pub mod source;
pub mod sources;
