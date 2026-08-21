//! Telling him when something has gone wrong.
//!
//! **Quiet about hiccups, loud about outages.**
//!
//! The price line drops. It always will — the feed closes an idle one, the
//! wifi blinks, a router reboots. Almost all of those fix themselves in
//! seconds, and a message for each is the same mistake as a candle every hour:
//! he learns the buzz means nothing, and then ignores the one that meant
//! something.
//!
//! So nothing is said until it has been down a while, and then it is said
//! once. **With a second message when it comes back** — "it broke" on its own
//! leaves him checking his phone all evening.

use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use nsc_core::when::Rules;

use std::path::PathBuf;

use crate::card::{self, Wrong};
use crate::telegram;

use super::pulse;
use crate::places::{OWNER, PREVIEW};

/// Remembers how long the line has been down, and whether he knows.
pub struct Trouble {
    /// When it first went wrong. `None` while everything is working.
    since: Option<DateTime<Utc>>,

    /// Whether he has been told about this outage.
    told: bool,
}

impl Trouble {
    pub fn new() -> Self {
        Trouble {
            since: None,
            told: false,
        }
    }

    /// The line broke. Says so only once it has been down long enough.
    ///
    /// `what` is what actually went wrong, in the words the code used. He is
    /// not going to debug it, but "connection refused" and "invalid API key"
    /// are different evenings.
    pub async fn broke(
        &mut self,
        client: &reqwest::Client,
        what: &str,
        calendar: &Rules,
        pulse: &mut pulse::Pulse,
    ) -> Result<()> {
        let now = Utc::now();
        let since = *self.since.get_or_insert(now);

        if self.told {
            return Ok(());
        }

        let down = now - since;
        if down < Duration::minutes(calendar.trouble_after_minutes) {
            return Ok(());
        }

        self.told = true;

        say(
            client,
            Wrong::LineDown,
            Some(down.num_minutes().max(1)),
            what,
            pulse,
        )
        .await
    }

    /// It is working again.
    ///
    /// **Only says so if he was told it was broken.** A recovery message for an
    /// outage he never heard about is a buzz that explains nothing.
    pub async fn mended(
        &mut self,
        client: &reqwest::Client,
        pulse: &mut pulse::Pulse,
    ) -> Result<()> {
        let Some(since) = self.since.take() else {
            return Ok(());
        };

        let told = self.told;
        self.told = false;

        if !told {
            return Ok(());
        }

        let minutes = (Utc::now() - since).num_minutes().max(1);

        say(client, Wrong::LineBack, Some(minutes), "", pulse).await
    }
}

/// The last thing it says before it stops.
///
/// **For trouble it cannot recover from** — a key it will never be given, a
/// config file that will not parse. Retrying those forever is the bot spinning
/// while he assumes it is watching.
///
/// It gives up quietly if even this fails. There is nothing left to try.
pub async fn dying(client: &reqwest::Client, what: &str) {
    let caption = card::caption(Wrong::Stopped);

    // **Falls back to plain words if the card cannot be drawn.** Chrome may be
    // the very thing that is broken, and the message matters more than the
    // picture — this is the one card whose failure must not swallow its own
    // message.
    let sent = match draw(Wrong::Stopped, None, what) {
        Ok(picture) => telegram::send_to(client, &OWNER.to_string(), &[&picture], caption).await,
        Err(trouble) => {
            eprintln!("Could not draw the card: {trouble}");
            telegram::send_words(client, &OWNER.to_string(), caption).await
        }
    };

    match sent {
        Ok(()) => eprintln!("Sent you the message."),
        Err(trouble) => eprintln!("Could not even send that message: {trouble}"),
    }
}

/// Takes the secrets back out of whatever went wrong.
///
/// **The detail on a trouble card is an error chain**, and an error chain
/// picks up whatever the failing code was holding. reqwest puts the URL it was
/// trying into its message, and the key and the token both live in a URL — so
/// "could not reach Telegram" arrived with the bot token printed in full.
///
/// The two places that happens are fixed at the source. This is here because
/// the next one has not been written yet, and a card goes to Telegram and is
/// left on disk in `preview/`.
pub(super) fn scrub(what: &str) -> String {
    let mut clean = what.to_string();

    // **The Twelve Data key is still on this list on purpose**, though nothing
    // has asked that feed for anything since 20 August 2026. The key is still
    // sitting in his `.env`, and a value that exists is a value that can end
    // up in an error message. It comes off this list when it comes out of the
    // file, not before.
    for name in ["TWELVE_DATA_API_KEY", "TELEGRAM_BOT_TOKEN"] {
        // Short ones are ignored. An unset or placeholder value could be a
        // couple of characters, and replacing those would gut the message.
        if let Ok(secret) = std::env::var(name)
            && secret.len() > 6
        {
            clean = clean.replace(&secret, "…");
        }
    }

    clean
}

/// Draws one, and gives back where it landed.
fn draw(wrong: Wrong, minutes: Option<i64>, what: &str) -> anyhow::Result<PathBuf> {
    let stamp = Utc::now().format("%-d %b · %H:%M UTC").to_string();
    let out = PathBuf::from(PREVIEW).join("trouble.png");

    Ok(card::trouble(wrong, minutes, &scrub(what), &stamp, &out)?)
}

/// Sends, and counts as having spoken today.
///
/// **Trouble counts.** He heard from the bot, so he knows it is alive — which
/// is the only thing the heartbeat was going to tell him.
async fn say(
    client: &reqwest::Client,
    wrong: Wrong,
    minutes: Option<i64>,
    what: &str,
    pulse: &mut pulse::Pulse,
) -> Result<()> {
    let caption = card::caption(wrong);
    println!("{}", caption.replace("<b>", "").replace("</b>", ""));

    let picture = draw(wrong, minutes, what)?;
    telegram::send_to(client, &OWNER.to_string(), &[&picture], caption).await?;

    pulse.spoke(Utc::now());

    Ok(())
}

/// Reachable from the tests, and nowhere else.
///
/// The scrubbing itself is not worth making public — but a secret leaking is
/// worth a test, and a test cannot check what it cannot call.
#[cfg(test)]
pub(crate) fn scrub_for_tests(what: &str) -> String {
    scrub(what)
}
