//! What can go wrong asking for candles.

use nsc_core::error::{Answer, Knows};
use thiserror::Error;

/// What can go wrong asking for candles.
///
/// Named rather than lumped together, because **a bad key and a busy server
/// need opposite responses**. Retry the key forever and it looks exactly like
/// a dead connection.
#[derive(Debug, Error)]
pub enum FeedError {
    /// The key is not in `.env`.
    #[error("TWELVE_DATA_API_KEY is not set. Is there a .env file in the project root?")]
    NoKey,

    /// Could not reach them at all.
    #[error("could not reach Twelve Data: {0}")]
    Unreachable(String),

    /// They answered, and said no.
    ///
    /// **The code decides everything.** 401 is a wrong key and will be wrong
    /// forever; 429 is "slow down" and will be fine in a minute.
    #[error("Twelve Data refused: {code} {message}")]
    Refused { code: u16, message: String },

    /// They answered with something that is not candles.
    #[error("Twelve Data did not send candles:\n{0}")]
    NotCandles(String),
}

impl Knows for FeedError {
    fn answer(&self) -> Answer {
        match self {
            // No key is no key. Waiting will not put one there.
            FeedError::NoKey => Answer::GiveUp,

            // The line, or their end. Both clear on their own.
            FeedError::Unreachable(_) => Answer::soon(),

            FeedError::Refused { code, .. } => match code {
                // Too many requests. They have TOLD us to wait, so wait
                // properly rather than hammering.
                429 => Answer::in_a_while(),

                // Their end fell over.
                500..=599 => Answer::soon(),

                // A wrong key, a pair not on the plan, a malformed request.
                // None of those get better by asking again.
                _ => Answer::GiveUp,
            },

            // Could be a blip, could be their shape changing. Worth one more
            // go — whoever is retrying will stop counting eventually.
            FeedError::NotCandles(_) => Answer::soon(),
        }
    }
}
