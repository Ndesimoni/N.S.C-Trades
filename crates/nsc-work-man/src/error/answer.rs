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

/// Does the job, and tries again if the trouble says it is worth it.
///
/// Gives up the moment the trouble says to — a wrong key is wrong on the
/// fifth go — and gives up after `attempts` even when it says otherwise,
/// because "keep trying" is not the same as "forever".
///
/// The wait doubles each time. Their end being busy is rarely fixed by asking
/// again immediately.
pub async fn keep_trying<T, E, F, Fut>(attempts: u32, mut job: F) -> Result<T, E>
where
    E: Knows,
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    let mut left = attempts.max(1);

    loop {
        let trouble = match job().await {
            Ok(done) => return Ok(done),
            Err(trouble) => trouble,
        };

        left -= 1;

        let Some(wait) = trouble.answer().wait() else {
            return Err(trouble);
        };

        if left == 0 {
            return Err(trouble);
        }

        tokio::time::sleep(wait * (attempts - left)).await;
    }
}
