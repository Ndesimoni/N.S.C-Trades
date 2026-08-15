//! The answer itself, and the retrying that uses it.

use std::time::Duration;

/// What to do about it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Answer {
    /// Try again. The line hiccupped, they were busy, the socket dropped.
    ///
    /// Carries how long to wait first — a rate limit wants longer than a
    /// dropped packet.
    TryAgain(Duration),

    /// Stop, and say so. The key is wrong, the pair is not on the plan, the
    /// file cannot be read. Trying again changes nothing.
    GiveUp,
}

impl Answer {
    /// Wait a moment and try again.
    pub fn soon() -> Self {
        Answer::TryAgain(Duration::from_secs(3))
    }

    /// They asked us to slow down. Wait properly.
    pub fn in_a_while() -> Self {
        Answer::TryAgain(Duration::from_secs(60))
    }

    pub fn worth_trying_again(self) -> bool {
        matches!(self, Answer::TryAgain(_))
    }

    /// How long to wait before trying. Nothing, if there is no point trying.
    pub fn wait(self) -> Option<Duration> {
        match self {
            Answer::TryAgain(how_long) => Some(how_long),
            Answer::GiveUp => None,
        }
    }
}

/// Anything that can go wrong and knows what to do about it.
pub trait Knows {
    fn answer(&self) -> Answer;
}
