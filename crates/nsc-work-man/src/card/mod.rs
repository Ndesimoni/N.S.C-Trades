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
mod listing;
mod live;
mod marking;
mod setup;
mod sizes;
mod soon;
mod spelling;
mod waiting;
mod wrong;
mod zone;

#[cfg(test)]
mod tests;

pub use alive::{Alive, heartbeat};
pub use error::CardError;
pub use fill::{render, render_marked};
pub use listing::calendar;
pub use live::armed;
pub use marking::{Mark, Part};
pub use setup::setup;
pub use sizes::{CONTEXT, RUN};
pub use soon::coming;
pub use spelling::as_written;
pub use wrong::{Wrong, caption, trouble};
pub use zone::{alert, closed};
