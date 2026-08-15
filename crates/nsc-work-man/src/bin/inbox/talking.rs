//! Saying things to Telegram.

use anyhow::{Context, Result, anyhow};
use serde_json::{Value, json};

use super::OWNER;

/// Sends a message, with buttons or without.
///
/// `keyboard` is a list of rows, each row a list of button words. Passing
/// nothing takes the buttons away and gives him his own keyboard back.
pub async fn say(
    client: &reqwest::Client,
    token: &str,
    text: &str,
    keyboard: Option<Value>,
) -> Result<()> {
    let markup = match keyboard {
        Some(rows) => json!({
            "keyboard": rows,
            "resize_keyboard": true,
            "one_time_keyboard": true,
        }),
        None => json!({ "remove_keyboard": true }),
    };

    let reply: Value = client
        .post(format!("https://api.telegram.org/bot{token}/sendMessage"))
        .json(&json!({
            "chat_id": OWNER,
            "text": text,
            "parse_mode": "HTML",
            "reply_markup": markup,
        }))
        .send()
        .await
        .map_err(|trouble| anyhow!("could not reach Telegram: {}", trouble.without_url()))?
        .json()
        .await
        .context("Telegram answered, but not with JSON")?;

    if reply["ok"] != true {
        println!(
            "Telegram refused: {}",
            reply["description"].as_str().unwrap_or("no reason given")
        );
    }

    Ok(())
}
