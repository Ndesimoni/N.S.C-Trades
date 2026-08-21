//! What can go wrong building a swing.

use thiserror::Error;

use crate::error::{Answer, Knows};

/// What can go wrong building a swing.
#[derive(Debug, Error)]
pub enum SwingError {
    /// **A swing that claims to be known before, or at, the candle it sits
    /// on.** That can never be true — you need candles AFTER a peak to know
    /// it was a peak.
    ///
    /// If this ever fires, whatever built the swing has a lookahead bug. And
    /// lookahead bugs do not announce themselves any other way: they make the
    /// results look BETTER, not broken.
    #[error("a swing on {bar_time} cannot have been known at {confirmed_at}")]
    KnownTooSoon {
        bar_time: String,
        confirmed_at: String,
    },
}

impl Knows for SwingError {
    fn answer(&self) -> Answer {
        // A lookahead bug is not a hiccup. Asking again gets the same answer.
        Answer::GiveUp
    }
}
