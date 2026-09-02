//! Noticing that he has sent a new level, without being restarted.
//!
//! **The levels used to be read once, at startup.** He would send one from his
//! phone, the inbox would save it correctly, the file would be right — and the
//! watcher would never look again. Nothing said so. The level simply did
//! nothing until the next restart, which might be days.

mod doing;
mod noticing;

#[cfg(test)]
mod tests;

pub use doing::{again, say_it_is_armed};
pub use noticing::Files;
