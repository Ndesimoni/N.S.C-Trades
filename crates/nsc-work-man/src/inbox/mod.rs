//! Listen to what he sends the bot, and save it.
//!
//! The other side of Telegram. `telegram/` talks; this listens.
//!
//! **It runs inside the bot**, alongside the watcher. It was a second program
//! for a while, and that meant two terminals and remembering both — and if the
//! inbox was not up, a level he sent went nowhere and nothing said so.
//!
//! Buttons are not set up anywhere — the bot sends them with a message, and
//! tapping one sends that word back as an ordinary message. A button is a
//! shortcut for typing, nothing more.
//!
//! ```text
//!   /level      ->  which pair?      [XAUUSD] [GBPUSD] [+ new pair]
//!   XAUUSD      ->  which timeframe? [Weekly] [Daily] [4-hour]
//!   Weekly      ->  send prices
//!   4520 4000   ->  saved, and it says back what the pair now holds
//! ```
//!
//! **The buttons are the files in `config/pairs/`.** Not a list in this file —
//! that was the mistake the old `settings.rs` made, and two lists disagree in
//! the end.

use anyhow::{Context, Result, anyhow};
use nsc_core::levels::Timeframe;
use serde_json::Value;

/// Only he may write levels.
///
/// **Channel posts carry no sender at all** — Telegram strips it, because a
/// post is from the channel rather than from a person. So the private chat is
/// the only place the bot can tell who is talking.
const OWNER: i64 = 6089491075;

const PAIRS: &str = "config/pairs";
const TIMEFRAMES: [(&str, Timeframe); 3] = [
    ("Weekly", Timeframe::Weekly),
    ("Daily", Timeframe::Daily),
    ("4-hour", Timeframe::H4),
];

const NEW_PAIR: &str = "+ new pair";
const UNDO: &str = "↩ Undo";

/// Stopping a pair takes two taps, not one.
///
/// It throws away every level he has drawn for that pair — months of chart
/// work — and it is done by tapping a button on a phone while doing something
/// else.
const YES: &str = "✓ Yes, stop it";
const NO: &str = "✗ Keep it";

mod conversation;
mod pairs;
mod picture;
mod talking;

use conversation::{Adding, handle};
use talking::say;

pub use talking::plainly;

/// How long to wait before listening again after a failure.
const AGAIN: std::time::Duration = std::time::Duration::from_secs(15);

/// Listens forever, and never gives up.
///
/// **It is spawned beside the watcher, so it must not be able to stop.** If it
/// ends, levels he sends go nowhere and nothing says so — which is the exact
/// failure that made it worth folding in.
pub async fn run(client: reqwest::Client) {
    loop {
        if let Err(trouble) = listen(&client).await {
            eprintln!("The inbox stopped listening: {trouble:#}");
        }

        tokio::time::sleep(AGAIN).await;
    }
}

async fn listen(client: &reqwest::Client) -> Result<()> {
    let token = std::env::var("TELEGRAM_BOT_TOKEN").context("TELEGRAM_BOT_TOKEN is not set")?;
    let mut adding = Adding::default();
    let mut seen_up_to: i64 = 0;

    loop {
        // `timeout=30` makes Telegram hold the line open rather than answering
        // "nothing" instantly. One request every thirty seconds, not hundreds.
        let url = format!(
            "https://api.telegram.org/bot{token}/getUpdates?offset={}&timeout=30",
            seen_up_to + 1
        );

        let reply: Value = client
            .get(&url)
            .send()
            .await
            .map_err(|trouble| anyhow!("could not reach Telegram: {}", trouble.without_url()))?
            .json()
            .await
            .context("Telegram answered, but not with JSON")?;

        for update in reply["result"].as_array().into_iter().flatten() {
            seen_up_to = update["update_id"].as_i64().unwrap_or(seen_up_to);
            let message = &update["message"];

            // Anything not from him is ignored without a word. A bot that
            // argues with strangers is a bot that tells them it exists.
            if message["from"]["id"].as_i64() != Some(OWNER) {
                continue;
            }

            let Some(text) = message["text"].as_str() else {
                continue;
            };

            println!("You said: {text}");

            if let Err(trouble) = handle(client, &token, text.trim(), &mut adding).await {
                println!("  -> {trouble:#}");
                say(
                    client,
                    &token,
                    &format!("Could not do that:\n{}", plainly(&format!("{trouble:#}"))),
                    None,
                )
                .await?;
            }
        }
    }
}
