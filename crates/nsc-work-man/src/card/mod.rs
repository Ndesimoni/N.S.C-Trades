//! Drawing a card by filling in a template and letting Chrome screenshot it.
//!
//! **The design lives in `assets/card/*.html`, not here.** Open one of those
//! files, change it, and the next message picks it up — no rebuild, no Rust.
//!
//! The cost: whatever machine runs this needs Chrome installed. Fine on a Mac.
//! A real dependency on a server, and worth remembering before it goes
//! anywhere but a laptop.

mod alive;
mod chrome;
mod error;
mod facts;
mod fill;
mod live;
mod wrong;
mod zone;

#[cfg(test)]
mod tests;

pub use alive::{Alive, heartbeat};
pub use error::CardError;
pub use fill::render;
pub use live::armed;
pub use wrong::{Wrong, caption, trouble};
pub use zone::{alert, closed};
