//! The one HTTP client the bot uses, **and the timeouts on it.**
//!
//! ## Why this file exists
//!
//! It was `reqwest::Client::new()`, which sets **no timeout at all**. A
//! request that hung hung forever.
//!
//! That is worse than it sounds, because [`keep_trying`](crate::retry) cannot
//! save it. Retrying answers an ERROR, and a hang is not an error — the future
//! simply never finishes, so the retry never gets its turn and neither does
//! anything queued behind it.
//!
//! Two things share this client and both had a way to die quietly:
//!
//! - **The calendar.** The news watcher downloads the week, then checks what
//!   is due, in that order and in one loop. A hung download stops the check
//!   from ever running again — no news card, ever, with nothing logged. And
//!   the whole point of the news watcher is that silence looks exactly like a
//!   quiet week.
//! - **Every card.** A hung send holds its caller. On the close path that is
//!   the look that would have reported the next candle too.
//!
//! Found on the 1 September read-back. Nothing had gone wrong yet — this is
//! the sort of fault that waits for a bad afternoon on someone's wifi.

use std::time::Duration;

/// **How long any one request may take, in total.**
///
/// Ninety seconds, and it is generous on purpose: a card is three pictures
/// going up a phone connection, and a timeout that turns a slow-but-working
/// send into a failure would be its own bug.
///
/// **It MUST stay longer than the inbox holds its line open.** Telegram is
/// asked to wait up to [`HELD_OPEN`](crate::inbox::HELD_OPEN) seconds before
/// answering "nothing new", which is what makes the inbox one request every
/// thirty seconds rather than hundreds. A timeout under that would cut every
/// poll short and look like Telegram being unreachable. A test pins it.
pub const AT_MOST: Duration = Duration::from_secs(90);

/// **How long to wait for the connection itself.**
///
/// Ten seconds. Separate from [`AT_MOST`] because it answers a different
/// question: that one bounds a conversation, this one bounds getting one
/// started. A host that is simply not there should be given up on quickly,
/// and doing so cannot cut short a long poll that has already connected.
pub const TO_CONNECT: Duration = Duration::from_secs(10);

/// The client everything shares.
///
/// **Falls back to an untimed one rather than refusing to start.** Building it
/// only fails if the system TLS store cannot be read, and a bot with no
/// timeouts is still a bot that watches his levels — where one that will not
/// start watches nothing.
pub fn client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(AT_MOST)
        .connect_timeout(TO_CONNECT)
        .build()
        .unwrap_or_else(|trouble| {
            eprintln!("Could not set request timeouts ({trouble}); carrying on without them.");
            reqwest::Client::new()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The one that would break the inbox.**
    ///
    /// Telegram is asked to hold a poll open for [`HELD_OPEN`] seconds. A
    /// request timeout under that cuts every poll short, and a cut-short poll
    /// looks exactly like Telegram being unreachable — so the bot would report
    /// the line as down, forever, on a line that was fine.
    ///
    /// Nothing about the two settings makes the relationship obvious, and they
    /// live in different files. This is the note that says it.
    #[test]
    fn a_request_may_outlast_a_held_open_poll() {
        assert!(
            AT_MOST > Duration::from_secs(crate::inbox::HELD_OPEN),
            "a poll held open for {}s cannot fit in a {}s timeout",
            crate::inbox::HELD_OPEN,
            AT_MOST.as_secs()
        );
    }

    /// Getting connected is bounded more tightly than the conversation, which
    /// is the point of having two settings rather than one.
    #[test]
    fn connecting_gives_up_sooner_than_the_whole_request() {
        assert!(TO_CONNECT < AT_MOST);
    }

    /// **Both are set.** The fault this file exists for was a client with
    /// neither, and a client built without them looks identical from here.
    #[test]
    fn neither_timeout_is_nothing() {
        assert!(AT_MOST > Duration::ZERO);
        assert!(TO_CONNECT > Duration::ZERO);
    }
}
