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

use super::{OWNER, PREVIEW, pulse};

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
            "⚠️ <b>The price line is down.</b> Nothing is being watched.",
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

        say(
            client,
            Wrong::LineBack,
            Some((Utc::now() - since).num_minutes().max(1)),
            "",
            "✅ <b>The price line is back.</b> Watching again.",
            pulse,
        )
        .await
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
    let caption = "🛑 <b>The bot has stopped.</b> Nothing is being watched.";

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

/// Draws one, and gives back where it landed.
fn draw(wrong: Wrong, minutes: Option<i64>, what: &str) -> anyhow::Result<PathBuf> {
    let stamp = Utc::now().format("%-d %b · %H:%M UTC").to_string();
    let out = PathBuf::from(PREVIEW).join("trouble.png");

    Ok(card::trouble(wrong, minutes, what, &stamp, &out)?)
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
    caption: &str,
    pulse: &mut pulse::Pulse,
) -> Result<()> {
    println!("{}", caption.replace("<b>", "").replace("</b>", ""));

    let picture = draw(wrong, minutes, what)?;
    telegram::send_to(client, &OWNER.to_string(), &[&picture], caption).await?;

    pulse.spoke(Utc::now());

    Ok(())
}
