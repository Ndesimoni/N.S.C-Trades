//! Watch price against the levels he drew, and say what happens at them.
//!
//! **Rungs 1 and 2 of the ladder.**
//!
//! ```text
//!     price reaches a zone        an alert card, once per visit
//!     a candle at one finishes    a card per candle that touched it
//! ```
//!
//! Silence is the normal state. Prices arrive about once a second and almost
//! all of them are nowhere near anything.
//!
//! **It watches nothing on a Monday.** Not "stays quiet" — nothing checked,
//! nothing fetched, no queue building up to be dumped on him on Tuesday. When
//! Tuesday opens it says what it FOUND rather than pretending it just
//! happened.
//!
//! This is what `cargo run -p nsc-work-man` does. It lives in the library
//! rather than in a binary so the binary can stay four lines, and so nothing
//! here needs a second copy to be looked at.

mod bands;
mod breathe;
mod closes;
mod kit;
mod line;
mod news;
mod prices;
mod pulse;
mod reload;
mod resumed;
mod run;
mod say;
mod standing;
mod trouble;

pub use bands::for_pair as size_bands;
pub use closes::settings as rung_three;
pub use closes::{CONTEXT, RUN};
pub use news::run as watch_the_news;
pub use run::{run, say_it_is_armed};
pub use standing::{Snapshot, Standing};
pub use trouble::dying;

pub(crate) use kit::{Kit, Watching};

#[cfg(test)]
pub(crate) use trouble::scrub_for_tests;
