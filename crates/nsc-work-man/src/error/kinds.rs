//! The named troubles themselves.
//!
//! One per thing that can fail, and **each one knows whether it is worth
//! trying again**. That is the only reason they are named rather than lumped
//! into a single catch-all.

use thiserror::Error;

use super::{Answer, Knows};

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

/// What can go wrong sending.
#[derive(Debug, Error)]
pub enum SendError {
    #[error("{0} is not set")]
    NotSet(&'static str),

    #[error("could not read {path}: {detail}")]
    NoPicture { path: String, detail: String },

    #[error("could not reach Telegram: {0}")]
    Unreachable(String),

    /// **Telegram refuses politely** — `ok: false` inside a perfectly ordinary
    /// reply. A reply that parses is not a message that arrived.
    #[error("Telegram refused: {0}")]
    Refused(String),
}

impl Knows for SendError {
    fn answer(&self) -> Answer {
        match self {
            // A missing token or a missing picture stays missing.
            SendError::NotSet(_) | SendError::NoPicture { .. } => Answer::GiveUp,

            SendError::Unreachable(_) => Answer::soon(),

            // Telegram says "Too Many Requests" in words rather than a code we
            // can match on, so the words are what there is to go by. Anything
            // else — a bad token, a chat that does not exist, a caption too
            // long — is settled and will not change.
            SendError::Refused(why) => {
                if why.contains("Too Many Requests") || why.contains("retry after") {
                    Answer::in_a_while()
                } else {
                    Answer::GiveUp
                }
            }
        }
    }
}

/// What can go wrong drawing a card.
#[derive(Debug, Error)]
pub enum CardError {
    #[error("Chrome is not at {0}, and the card is drawn by Chrome")]
    NoChrome(String),

    #[error("could not read the card template at {path}: {detail}")]
    NoTemplate { path: String, detail: String },

    /// A template must say how tall it is, because Chrome screenshots a
    /// **window** rather than a page. Guessing gives a clipped footer that
    /// nobody notices until it is in a signal.
    #[error("{0} has no --card-height line in its CSS")]
    NoHeight(String),

    #[error("there are no candles to draw")]
    NothingToDraw,

    /// **Chrome answers 0 whether it drew the card or its own error page**, so
    /// the only honest check is whether a file appeared.
    #[error("Chrome ran but wrote no picture:\n{0}")]
    DrewNothing(String),

    #[error("could not write {path}: {detail}")]
    CannotWrite { path: String, detail: String },
}

impl Knows for CardError {
    fn answer(&self) -> Answer {
        match self {
            // Chrome does not install itself, a missing template does not
            // appear, and a template with no height still has none next time.
            CardError::NoChrome(_)
            | CardError::NoTemplate { .. }
            | CardError::NoHeight(_)
            | CardError::NothingToDraw => Answer::GiveUp,

            // Chrome falling over, or a disk busy for a moment. Both have been
            // known to clear.
            CardError::DrewNothing(_) | CardError::CannotWrite { .. } => Answer::soon(),
        }
    }
}

/// What can go wrong reading or writing his levels.
#[derive(Debug, Error)]
pub enum LevelError {
    #[error("could not read {path}: {detail}")]
    CannotRead { path: String, detail: String },

    #[error("could not write {path}: {detail}")]
    CannotWrite { path: String, detail: String },

    /// **Give up rather than guess.** A levels file that half-parses would put
    /// bands at prices he never drew, and every signal after that inherits it.
    #[error("{path} cannot be read as {expected}: {detail}")]
    NotReadable {
        path: String,
        expected: &'static str,
        detail: String,
    },
}

impl Knows for LevelError {
    fn answer(&self) -> Answer {
        match self {
            // A file that will not parse will not parse in three seconds
            // either, and half-reading it is worse than stopping.
            LevelError::NotReadable { .. } => Answer::GiveUp,

            // A disk busy for a moment.
            LevelError::CannotRead { .. } | LevelError::CannotWrite { .. } => Answer::soon(),
        }
    }
}
