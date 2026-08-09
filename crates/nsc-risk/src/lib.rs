//! # nsc-risk — position size, exposure, and brakes
//!
//! Version 1 sends signals only, so nothing here places a trade. It still
//! matters: this crate stops the bot sending you the same idea five times
//! and stops it shouting at you through a losing streak.
//!
//! Settings live in `config/risk.toml`.

pub mod brakes;
pub mod correlation;
pub mod error;
pub mod exposure;
pub mod sizing;
