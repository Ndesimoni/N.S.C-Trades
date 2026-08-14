//! Sending to the channel.

use std::path::Path;

use anyhow::{Context, Result, bail};

/// Post several pictures as one message.
///
/// Telegram calls this a media group. It arrives as a single message and buzzes
/// the phone once, but the pictures sit apart with a gap and **each one opens
/// on its own when tapped** — which is why the cards are separate files rather
/// than one tall picture.
///
/// The caption goes on the first picture and shows under the whole group. Put
/// it on every picture and Telegram repeats it under every picture.
pub async fn send(client: &reqwest::Client, pictures: &[&Path], caption: &str) -> Result<()> {
    let token = std::env::var("TELEGRAM_BOT_TOKEN").context("TELEGRAM_BOT_TOKEN is not set")?;
    let chat = std::env::var("TELEGRAM_CHAT_ID").context("TELEGRAM_CHAT_ID is not set")?;

    let mut form = reqwest::multipart::Form::new().text("chat_id", chat);
    let mut described = Vec::new();

    for (position, picture) in pictures.iter().enumerate() {
        let name = format!("photo{position}");

        let bytes = std::fs::read(picture)
            .with_context(|| format!("could not read {} back", picture.display()))?;

        form = form.part(
            name.clone(),
            reqwest::multipart::Part::bytes(bytes).file_name(format!("{name}.png")),
        );

        described.push(if position == 0 {
            serde_json::json!({
                "type": "photo",
                "media": format!("attach://{name}"),
                "caption": caption,
                "parse_mode": "HTML",
            })
        } else {
            serde_json::json!({ "type": "photo", "media": format!("attach://{name}") })
        });
    }

    form = form.text("media", serde_json::json!(described).to_string());

    let reply: serde_json::Value = client
        .post(format!(
            "https://api.telegram.org/bot{token}/sendMediaGroup"
        ))
        .multipart(form)
        .send()
        .await
        .context("could not reach Telegram")?
        .json()
        .await
        .context("Telegram answered, but not with JSON")?;

    // Telegram refuses politely — `ok: false` inside a perfectly normal reply.
    // The same trap Twelve Data sets with its 401, met twice in one afternoon,
    // so it is a pattern rather than bad luck.
    if reply["ok"] != true {
        bail!(
            "Telegram refused the message: {}",
            reply["description"].as_str().unwrap_or("no reason given")
        );
    }

    Ok(())
}
