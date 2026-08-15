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

use crate::telegram;

use super::{OWNER, pulse};

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

        let words = format!(
            "⚠️ <b>The price line is down.</b>\n\n\
             It has been {} minutes and it is still not opening.\n\
             Nothing is being watched while this lasts.\n\n\
             <i>{what}</i>\n\n\
             It keeps trying. You will get a message when it is back.",
            down.num_minutes().max(1),
        );

        say(client, &words, pulse).await
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

        let words = format!(
            "✅ <b>The price line is back.</b>\n\n\
             It was down {} minutes. Watching again.",
            (Utc::now() - since).num_minutes().max(1),
        );

        say(client, &words, pulse).await
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
    let words = format!(
        "🛑 <b>The bot has stopped.</b>\n\n\
         This one does not fix itself, so it is not trying again.\n\n\
         <i>{what}</i>\n\n\
         Nothing is being watched until you start it.",
    );

    // It says whether it managed to. There is nothing left to try if the
    // message itself cannot go, but a terminal that claims it told him when it
    // did not is worse than one that admits it.
    match telegram::send_words(client, &OWNER.to_string(), &words).await {
        Ok(()) => eprintln!("told him it stopped."),
        Err(trouble) => eprintln!("could not even tell him it stopped: {trouble}"),
    }
}

/// Sends, and counts as having spoken today.
///
/// **Trouble counts.** He heard from the bot, so he knows it is alive — which
/// is the only thing the heartbeat was going to tell him.
async fn say(client: &reqwest::Client, words: &str, pulse: &mut pulse::Pulse) -> Result<()> {
    println!("{}", words.replace("<b>", "").replace("</b>", ""));

    telegram::send_words(client, &OWNER.to_string(), words).await?;
    pulse.spoke(Utc::now());

    Ok(())
}
