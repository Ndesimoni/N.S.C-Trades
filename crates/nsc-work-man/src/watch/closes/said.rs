//! What one report was about.

/// Which of the two things was said about a candle.
///
/// A candle gets spoken about **twice** — once part-way through, once when it
/// finishes — and they must be remembered apart, or the look silences the
/// close that follows it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum Kind {
    /// Part-way through, while it was still running.
    SoFar,
    /// Finished.
    Closed,
}

/// What one report was about: a pair's zone, on one interval, in one way.
///
/// **The zone is in the key, not just the candle.** It used to be remembered
/// per candle, so a second zone coming live in the middle of an hour never got
/// that hour's close at all — price reaches 4,120, the candle is reported,
/// price then runs to 4,135, and the same candle is remembered as done. He
/// waited a full hour for news the bot already had.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct Said {
    pub(super) symbol: String,
    pub(super) interval: &'static str,
    pub(super) kind: Kind,

    /// The zone it was about, as written. `Band` is not hashable and the price
    /// is what identifies a level anyway — the same number is the same line.
    pub(super) band: String,
}
