//! # nsc-telegram — sending signals and collecting your verdict
//!
//! Sends signals, and captures what you thought of them.
//!
//! The second half is the one that matters long term. Those 👍/👎 buttons look
//! like a nice touch and are actually the data pipeline for Phase 4. Every
//! signal is tracked to its result automatically, and every button press adds
//! your opinion on top. Result plus opinion is exactly the pair a model needs
//! to learn which setups you would have taken.
//!
//! Build this from day one. Adding it later means throwing away every signal
//! sent before it existed — months of the data that is hardest to recreate.

pub mod bot;
pub mod callbacks;
pub mod commands;
pub mod error;
pub mod format;
pub mod keyboard;
