//! Everything that talks to the world.
//!
//! The feed, Telegram, Chrome, and the programs that use them.
//!
//! **What the bot KNOWS lives in `nsc-core`** — a candle, a level, what went
//! wrong. That crate has no `reqwest` and no `tokio`, so nothing in it can
//! reach anything. This one is where reaching happens.

pub mod card;
pub mod feed;
pub mod retry;
pub mod review;
pub mod telegram;
pub mod watch;
