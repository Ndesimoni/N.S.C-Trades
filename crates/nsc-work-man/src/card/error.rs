//! What can go wrong drawing a card.

use thiserror::Error;

use nsc_core::error::{Answer, Knows};

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
