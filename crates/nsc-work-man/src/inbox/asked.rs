//! The two questions he can ask outright.
//!
//! `/help` because the command list lives in a file on his Mac and he is on a
//! phone, and `/status` because "is it running, and is anything close?" turns
//! up at three in the afternoon as well as at seven in the morning.

use std::path::PathBuf;

use anyhow::Result;
use chrono::Utc;
use nsc_core::when::beat_words;
use tokio::sync::watch;

use crate::card::{self, Alive};
use crate::telegram;
use crate::watch::Snapshot;

use super::talking::say;
use crate::places::{OWNER, PREVIEW};

/// What he can send, and what each one does.
///
/// **Also registered with Telegram**, so these appear in the tap-list beside
/// the message box and he never types one.
pub const COMMANDS: [(&str, &str); 7] = [
    ("status", "Is it running, and what is close?"),
    ("news", "What is coming up — today or this week"),
    ("pairs", "See your pairs, and change one"),
    ("chart", "See a pair's chart"),
    ("level", "Add a level"),
    ("remove", "Stop watching a pair"),
    ("restore", "Put a stopped pair back"),
];

pub async fn help(client: &reqwest::Client, token: &str) -> Result<()> {
    let list: Vec<String> = COMMANDS
        .iter()
        .map(|(name, what)| format!("/{name} — {what}"))
        .collect();

    let words = format!(
        "<b>What you can send me</b>\n\n{}\n\n\
         After saving levels you also get <b>↩ Undo</b>, which takes back only \
         what that message added.\n\n\
         Everything else arrives on its own. Nothing on a quiet hour.",
        list.join("\n"),
    );

    say(client, token, &words, None).await
}

/// Tells Telegram the commands exist, so they show in the tap-list.
///
/// **Done once at startup.** It is not worth failing over — if it does not
/// take, every command still works by typing it.
pub async fn register(client: &reqwest::Client, token: &str) {
    let commands: Vec<_> = COMMANDS
        .iter()
        .map(|(name, what)| serde_json::json!({ "command": name, "description": what }))
        .collect();

    let sent = client
        .post(format!("https://api.telegram.org/bot{token}/setMyCommands"))
        .json(&serde_json::json!({ "commands": commands }))
        .send()
        .await;

    // **Reaching them is not the same as them agreeing.** Telegram answers 200
    // with `{"ok": false}` when it refuses, so checking only for a network
    // error printed "the command menu is set" over a menu that never appeared.
    match sent {
        Err(trouble) => eprintln!(
            "Could not set the command menu: {}. They still work typed.",
            trouble.without_url()
        ),

        Ok(reply) => match reply.json::<serde_json::Value>().await {
            Ok(said) if said["ok"] == true => println!("The command menu is set."),

            Ok(said) => eprintln!(
                "Telegram would not set the command menu: {}. They still work typed.",
                said["description"].as_str().unwrap_or("no reason given"),
            ),

            Err(trouble) => eprintln!(
                "Telegram answered the command menu oddly: {}. They still work typed.",
                trouble.without_url()
            ),
        },
    }
}

/// Where everything stands, right now.
///
/// **The same card as the morning heartbeat.** That one only comes on a day
/// nothing else did; this is the same question asked whenever he likes.
pub async fn status(
    client: &reqwest::Client,
    token: &str,
    standing: &watch::Receiver<Snapshot>,
) -> Result<()> {
    // Copied out before drawing. Holding the borrow across an await would keep
    // the watcher from publishing while Chrome runs.
    let now = standing.borrow().clone();

    if now.pairs.is_empty() {
        return say(
            client,
            token,
            "Nothing is being watched. Send /level.",
            None,
        )
        .await;
    }

    // **A quiet day gets words and no picture.** The card's one useful column
    // is how far price is from the nearest zone, and on a quiet day no price
    // has arrived to measure from — so every row would read as a dash. It also
    // saves running Chrome for the best part of ten seconds to say nothing.
    if now.quiet {
        return say(client, token, &resting(&now), None).await;
    }

    let alive: Vec<Alive<'_>> = now
        .pairs
        .iter()
        .map(|one| Alive {
            pair: &one.pair,
            bands: one.bands.clone(),
            price: one.price,
        })
        .collect();

    let hours = (Utc::now() - now.opened).num_hours();
    let quiet = format!("{hours} hour{}", if hours == 1 { "" } else { "s" });
    let stamp = Utc::now().format("%-d %b · %H:%M UTC").to_string();

    let words = beat_words(now.pairs.len(), now.zones());
    let out = PathBuf::from(PREVIEW).join("status.png");

    // **It answers either way.** This is the one command whose whole job is
    // "are you alive", and it used to reply "Could not do that" whenever the
    // card would not go — the single most misleading thing it could say.
    //
    // The words carry the answer; the picture only carries it better. Both the
    // drawing and the sending are covered, because a photo Telegram refuses
    // leaves him just as unanswered as one Chrome never drew.
    let sent = match card::heartbeat(&alive, &quiet, &stamp, &out) {
        Ok(picture) => telegram::send_to(client, &OWNER.to_string(), &[&picture], &words)
            .await
            .map_err(anyhow::Error::from),

        Err(trouble) => Err(trouble.into()),
    };

    if let Err(trouble) = sent {
        eprintln!("Could not send the status card: {trouble:#}");
        return say(client, token, &words, None).await;
    }

    Ok(())
}

/// What to say on a day nothing is watched.
///
/// **The card is the wrong shape for it.** Its useful column is how far price
/// is from the nearest zone, and on a quiet day no price has arrived — so
/// every row reads as a dash and the message says nothing at all.
fn resting(now: &Snapshot) -> String {
    format!(
        "😴 <b>Resting</b>\n\n\
         The market is shut, or today is one you have set aside. Nothing is \
         watched and nothing is fetched.\n\n\
         <b>{}</b> pairs · <b>{}</b> zones are loaded and ready.\n\n\
         <i>It opens the line when the next session does.</i>",
        now.pairs.len(),
        now.zones(),
    )
}
