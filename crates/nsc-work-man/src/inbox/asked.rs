//! The two questions he can ask outright.
//!
//! `/help` because the command list lives in a file on his Mac and he is on a
//! phone, and `/status` because "is it running, and is anything close?" turns
//! up at three in the afternoon as well as at seven in the morning.

use std::path::PathBuf;

use anyhow::{Context, Result};
use chrono::Utc;
use nsc_core::when::beat_words;
use tokio::sync::watch;

use crate::card::{self, Alive};
use crate::telegram;
use crate::watch::Snapshot;

use super::OWNER;
use super::talking::say;

/// What he can send, and what each one does.
///
/// **Also registered with Telegram**, so these appear in the tap-list beside
/// the message box and he never types one.
pub const COMMANDS: [(&str, &str); 5] = [
    ("status", "Is it running, and what is close?"),
    ("pairs", "See your pairs, and change one"),
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

    match sent {
        Ok(_) => println!("The command menu is set."),
        Err(trouble) => eprintln!(
            "Could not set the command menu: {}. They still work typed.",
            trouble.without_url()
        ),
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

    let out = PathBuf::from("preview").join("status.png");
    let picture = card::heartbeat(&alive, &quiet, &stamp, &out).context("could not draw it")?;

    telegram::send_to(
        client,
        &OWNER.to_string(),
        &[&picture],
        &beat_words(now.pairs.len(), now.zones()),
    )
    .await
    .map_err(Into::into)
}
