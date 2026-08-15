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

    /// Putting a stopped pair back over one he is already watching.
    ///
    /// **Settled.** Retrying cannot help — he has to stop the live one first,
    /// or he would lose levels he is using without being told.
    #[error("{0} is already being watched — stop it first")]
    AlreadyThere(String),
}

impl Knows for LevelError {
    fn answer(&self) -> Answer {
        match self {
            // A file that will not parse will not parse in three seconds
            // either, and half-reading it is worse than stopping.
            LevelError::NotReadable { .. } => Answer::GiveUp,

            // Nor will the pair stop being watched on its own. He has to stop
            // the live one first, and trying again would never help.
            LevelError::AlreadyThere(_) => Answer::GiveUp,

            // A disk busy for a moment.
            LevelError::CannotRead { .. } | LevelError::CannotWrite { .. } => Answer::soon(),
        }
    }
}
