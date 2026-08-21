//! Starting the bot up, and keeping it up.
//!
//! ```text
//!     forever.rs   the loop — open the line, watch it, open it again
//!     armed.rs     reading the levels again when he sends one
//!     picture.rs   the live picture, for anything that ASKS rather
//!                  than watches — /status and the heartbeat
//! ```

mod armed;
mod forever;
mod picture;
mod retiring;

pub use armed::say_it_is_armed;
pub use forever::run;
pub(crate) use picture::snapshot;
