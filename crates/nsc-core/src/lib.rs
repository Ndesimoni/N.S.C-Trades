//! What the bot knows, with nothing it can reach.
//!
//! A candle. A level. What went wrong, and whether it is worth another go.
//!
//! **There is no feed in here, no Telegram, no Chrome and no clock** — and not
//! by discipline. `Cargo.toml` has no `reqwest` and no `tokio`, so nothing
//! here *can* call the network. The compiler refuses.
//!
//! That is what will let the backtester and the live bot run the same analysis
//! and get the same answer. The moment the analysis can fetch something, a
//! backtest stops describing the bot — and it does not look broken, it looks
//! *better*.
//!
//! Everything that talks to the world lives in `nsc-work-man`.

pub mod candle;
pub mod error;
pub mod levels;
pub mod swing;
pub mod when;
