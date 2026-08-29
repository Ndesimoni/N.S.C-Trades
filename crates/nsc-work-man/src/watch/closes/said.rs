//! What one report was about.

use nsc_data::source::Interval;

/// Which of the things was said about a candle.
///
/// **There used to be a `SoFar` here too**, for the mid-candle look. That card
/// went on 27 August 2026 and the variant went with it — a value nothing ever
/// constructs is a key nothing ever matches, and it would have sat in this
/// enum looking like a state the bot could still be in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum Kind {
    /// Finished, and it closed outside the band.
    Closed,

    /// **Rung 3 — a shape he trades, at one of his zones.**
    ///
    /// Kept apart from `Closed` deliberately. The same finished candle can be
    /// worth both messages — what it did at the band, and the shape it
    /// completed — and folding them into one key would silence whichever
    /// arrived second.
    Setup,
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

    /// **The timeframe as a type, not as its spelling.** It was a `&'static
    /// str` while the feed's own words were carried around; two spellings of
    /// the same timeframe would have been two different keys, and the same
    /// candle would have reported twice.
    pub(super) interval: Interval,

    pub(super) kind: Kind,

    /// The zone it was about, as written. `Band` is not hashable and the price
    /// is what identifies a level anyway — the same number is the same line.
    pub(super) band: String,
}
