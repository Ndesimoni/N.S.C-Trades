//! Trend, and the moment it is proved.
//!
//! Higher highs and higher lows means uptrend. Lower highs and lower lows
//! means downtrend. Neither sentence can be said without swing points to
//! count, which is why this sits on top of [`crate::swing`].
//!
//! ## Taking the old high out is not enough
//!
//! The event that matters is not price crossing an old high. It is price
//! crossing it **and going somewhere** — carrying a share of the run that made
//! that high past it.
//!
//! Poke through by a few points and stall, and the high was touched, not
//! taken. That is the most common trap on a chart: it looks like a breakout,
//! it pulls buyers in, and price turns straight back down. A bot without this
//! rule reads it as a higher high, calls the trend intact, and goes hunting
//! for a long at the worst possible moment.
//!
//! ## Both outcomes get reported
//!
//! A market that tried to take a high and could not is telling you something.
//! Those are the "do not take this" examples nothing else in the system
//! collects, and they cannot be gathered afterwards — so a failed attempt is
//! a result here, not a silence.
//!
//! ## What is where
//!
//! - [`trend`] — `Trend`, which way the market is going
//! - [`breaks`] — `StructureBreak`, one old extreme properly taken out
//! - [`attempts`] — `FailedAttempt`, one it crossed and could not hold past
//! - [`event`] — `StructureEvent`, either of those two
//!
//! How far past is a share of the previous run, never a number of pips or of
//! normal candles — see `README.txt` for why that particular yardstick.

mod attempts;
mod breaks;
mod event;
mod trend;

#[cfg(test)]
mod tests;

pub use attempts::FailedAttempt;
pub use breaks::StructureBreak;
pub use event::StructureEvent;
pub use trend::Trend;
