//! What to say when the opening hours are over.
//!
//! **The first thing each session is a report of what was FOUND**, never of
//! anything arriving.
//!
//! Price can walk into a zone while the bot is quiet — over a weekend, or
//! through the opening hours — and still be sitting there when it starts
//! looking. The watcher fires on a *change*, so nothing would ever be said
//! about it, and saying "arrived" would put Sunday's move on Monday's clock.
//!
//! ## Why it waits
//!
//! It used to go the moment the socket opened. The first hours of a day are
//! where a move gets faked and taken back, so that report was of a position
//! price had not committed to — and he would act on it, or learn to ignore it.
//!
//! Now it waits for the settle window to pass and then says where things
//! actually stand. That is the same moment a trade becomes allowed, which is
//! not a coincidence: it is the point the day is worth reading.
//!
//!   awake.rs   the report itself, and what it remembers
//!   tests.rs   what it remembers, which is the whole of the behaviour

mod awake;

#[cfg(test)]
mod tests;

pub use awake::Awake;
