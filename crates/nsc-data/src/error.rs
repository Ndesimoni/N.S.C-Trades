//! Things that go wrong getting price data in.
//!
//! The different kinds exist to answer one question where they are caught:
//! **retry, or give up?**
//!
//!   - timeout or connection dropped → retry, backing off. The broker is
//!     having a moment.
//!   - rate limited                  → retry, but slower, and respect what
//!     the provider told you.
//!   - bad API key                   → **give up.** Retrying a dead key
//!     forever looks exactly like a dead feed and wastes hours of hunting.
//!   - unexpected response format    → **give up and shout.** The provider
//!     changed something, and guessing at the new format corrupts your
//!     history.
//!   - missing candles               → **give up.** Analysis on holed data is
//!     worse than no analysis, because it still produces confident numbers.
//!
//! Lumping these together is how a bot ends up retrying an expired key every
//! thirty seconds for a week.
//!
//! Only the file-reading kinds exist so far. The broker ones arrive with the
//! brokers — an error for code that does not exist is a promise nobody can
//! keep.

use std::path::PathBuf;

use thiserror::Error;

/// What can go wrong getting candles in.
#[derive(Debug, Error)]
pub enum DataError {
    /// The file could not be opened or read at all.
    #[error("cannot read {path}: {detail}")]
    CannotRead { path: PathBuf, detail: String },

    /// The columns are not the ones expected.
    ///
    /// **Give up rather than guess.** A file whose columns are in a different
    /// order still parses if you guess, and every price in it is then wrong in
    /// a way nothing downstream can notice.
    #[error("cannot make sense of the columns in {path}: {detail}")]
    BadHeader { path: PathBuf, detail: String },

    /// A row could not be read.
    ///
    /// **Give up rather than skip.** A live feed sending one bad candle should
    /// be shrugged off, but a file is the same every time it is read — so a
    /// bad row is a broken file, and quietly dropping it changes every level
    /// built from it with nothing to show that it happened.
    #[error("{path} line {line}: {detail}")]
    BadRow {
        path: PathBuf,
        line: usize,
        detail: String,
    },

    /// Asked to find holes in candles that are not a fixed number of minutes
    /// apart.
    ///
    /// Daily and weekly candles are not. Clocks change and weekends are three
    /// days long, so "one step along" is not a subtraction. Guessing would
    /// report a hole at every weekend and miss the real ones.
    #[error(
        "{timeframe} candles are not a fixed number of minutes apart, so holes in them cannot be found by subtracting"
    )]
    NotAFixedStep { timeframe: String },

    /// Two candles at the same time, or one before the one in front of it.
    ///
    /// Not a hole — a broken file. Everything that reads candles assumes they
    /// are in order, so this has to stop rather than be scanned past.
    #[error("the candles are not in time order: {at} comes after {previous}")]
    OutOfOrder {
        previous: chrono::DateTime<chrono::Utc>,
        at: chrono::DateTime<chrono::Utc>,
    },

    /// A timestamp could not be understood.
    #[error("{path} line {line}: '{text}' is not a time this program can read")]
    BadTimestamp {
        path: PathBuf,
        line: usize,
        text: String,
    },

    /// A levels file could not be parsed.
    ///
    /// **Give up rather than guess.** A level with a mistyped price is a line
    /// drawn at the wrong place, and every decision made against it is wrong
    /// with nothing to show that it happened.
    #[error("cannot read the levels in {path}: {detail}")]
    BadLevelsFile { path: PathBuf, detail: String },

    /// Something the shared types refused — an impossible candle, or candles
    /// out of order.
    ///
    /// Passed through rather than reworded, because the detail says which
    /// candle to go and look at.
    #[error(transparent)]
    Core(#[from] nsc_core::error::CoreError),
}
