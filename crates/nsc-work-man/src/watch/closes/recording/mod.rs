//! **Writing down what rung 3 decided** — the signal, or the refusal.
//!
//! ```text
//!   version.rs   a hash of the settings that produced it
//!   features.rs  everything the bot saw, as it saw it
//!   writing.rs   turning a decision into a row
//!   asking.rs    the two buttons under a setup
//! ```
//!
//! ## Why the refusals matter as much as the signals
//!
//! `CLAUDE.md`: *"Rejected setups get saved, not thrown away. Save which layer
//! rejected them. Those rows answer 'why did nothing fire this week?' and they
//! are the 'don't take this' examples the Phase 4 model needs."*
//!
//! A quiet week and a broken bot look identical from the outside. So do "no
//! shapes printed" and "forty printed and none was near a level" — and those
//! are completely different problems.
//!
//! ## Nothing here can end the run
//!
//! A record that will not write is a gap in the history. The bot's job is
//! watching his levels, and it must not stop doing that because Postgres is
//! down.

mod asking;
mod features;
mod version;
mod writing;

pub use asking::ask;
pub(super) use version::rules_version;
pub(super) use writing::{Made, Missed, keep_refusal, keep_signal};
