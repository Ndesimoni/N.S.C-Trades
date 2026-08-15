//! What can go wrong reading or writing his levels.

use thiserror::Error;

use crate::error::{Answer, Knows};

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
