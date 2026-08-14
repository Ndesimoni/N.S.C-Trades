//! Catching the use of data you did not have yet. **The most important folder
//! here.**
//!
//! The types already refuse a lookahead one at a time. A `Swing` cannot be
//! confirmed before the candle it sits on. A `Candle` that is still forming
//! cannot become a `BarClosed`.
//!
//! What no single type can see is the *run*. A swing confirmed on Friday is a
//! perfectly valid swing — it is only wrong if something reads it on Tuesday.
//! Nothing about the swing itself says so. Only the moment it was read does,
//! and only the run knows that.
//!
//! That is what `Guard` is: a watcher standing at one moment, that everything
//! being read has to pass through.
//!
//! ## Why this gets its own folder instead of being a habit at code review
//!
//! **This mistake does not produce an error. It produces a better result.**
//!
//! The backtest finishes. The equity curve looks excellent. The only symptom
//! is that live trading never resembles it — and by then months have gone by
//! and the cause is several rewrites back.
//!
//! Anything that survives a run with the guard on is at least *achievable*.
//! That is a much stronger statement than "the numbers look good".
//!
//! ## It is always on
//!
//! There is no switch. A check you can turn off is a check that is off on the
//! run whose number you end up believing.

mod watcher;

#[cfg(test)]
mod tests;

pub use watcher::Guard;
