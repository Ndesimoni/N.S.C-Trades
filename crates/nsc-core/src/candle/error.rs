//! What can go wrong reading a candle.

use thiserror::Error;

use crate::error::{Answer, Knows};

/// What can go wrong reading a candle.
#[derive(Debug, Error)]
pub enum CandleError {
    /// **A weekly or daily candle is stamped with a bare date**, not a time.
    /// Reading one as an open time would put it a whole day out, so it is
    /// refused rather than guessed at.
    #[error("'{0}' is not a time this program can read")]
    NotATime(String),

    #[error("the interval is not a length of time that can be held")]
    ImpossibleInterval,
}

impl Knows for CandleError {
    fn answer(&self) -> Answer {
        // A stamp that cannot be read now cannot be read in three seconds.
        Answer::GiveUp
    }
}
