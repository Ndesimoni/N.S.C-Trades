//! What can go wrong sending.

use nsc_core::error::{Answer, Knows};
use thiserror::Error;

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
