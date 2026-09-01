//! Everything that talks to the world.
//!
//! The feed, Telegram, Chrome, and the programs that use them.
//!
//! **What the bot KNOWS lives in `nsc-core`** — a candle, a level, what went
//! wrong. That crate has no `reqwest` and no `tokio`, so nothing in it can
//! reach anything. This one is where reaching happens.
//!
//! **Where prices come from lives in `nsc-data`.** Candles and the live price
//! line both come from IBKR, through `nsc_data::sources::ibkr` — there is no
//! `feed/` here any more, and nothing in this crate holds a broker's address.

pub mod card;
pub mod inbox;
pub mod places;
pub mod retry;
pub mod review;
pub mod secrets;
pub mod telegram;
pub mod watch;
pub mod web;
