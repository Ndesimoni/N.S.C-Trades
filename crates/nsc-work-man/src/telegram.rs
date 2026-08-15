//! Sending to the channel.

use std::path::Path;

use thiserror::Error;

use crate::trouble::{Answer, Knows};

/// What can go wrong sending.
#[derive(Debug, Error)]
pub enum SendError {
    #[error("{0} is not set")]
    NotSet(&'static str),

    #[error("could not read {path}: {detail}")]
    NoPicture { path: String, detail: String },

    #[error("could not reach Telegram: {0}")]
    Unreachable(String),

    /// **Telegram refuses politely** — `ok: false` inside a perfectly ordinary
    /// reply. A reply that parses is not a message that arrived.
    #[error("Telegram refused: {0}")]
    Refused(String),
}

impl Knows for SendError {
    fn answer(&self) -> Answer {
        match self {
            // A missing token or a missing picture stays missing.
            SendError::NotSet(_) | SendError::NoPicture { .. } => Answer::GiveUp,

            SendError::Unreachable(_) => Answer::soon(),

            // Telegram says "Too Many Requests" in words rather than a code we
            // can match on, so the words are what there is to go by. Anything
            // else — a bad token, a chat that does not exist, a caption too
            // long — is settled and will not change.
            SendError::Refused(why) => {
                if why.contains("Too Many Requests") || why.contains("retry after") {
                    Answer::in_a_while()
                } else {
                    Answer::GiveUp
                }
            }
        }
    }
}

/// Post several pictures as one message.
///
/// Telegram calls this a media group. It arrives as a single message and buzzes
/// the phone once, but the pictures sit apart with a gap and **each one opens
/// on its own when tapped** — which is why the cards are separate files rather
/// than one tall picture.
///
/// The caption goes on the first picture and shows under the whole group. Put
/// it on every picture and Telegram repeats it under every picture.
pub async fn send(
    client: &reqwest::Client,
    pictures: &[&Path],
    caption: &str,
) -> Result<(), SendError> {
    let chat =
        std::env::var("TELEGRAM_CHAT_ID").map_err(|_| SendError::NotSet("TELEGRAM_CHAT_ID"))?;

    send_to(client, &chat, pictures, caption).await
}

/// The same, to a particular chat.
///
/// **Signals go to the channel; his own working goes to the private chat.**
/// A chart he asked for while adding a level is not a signal, and mixing the
/// two turns the channel into a scratchpad.
pub async fn send_to(
    client: &reqwest::Client,
    chat: &str,
    pictures: &[&Path],
    caption: &str,
) -> Result<(), SendError> {
    let token =
        std::env::var("TELEGRAM_BOT_TOKEN").map_err(|_| SendError::NotSet("TELEGRAM_BOT_TOKEN"))?;

    let mut form = reqwest::multipart::Form::new().text("chat_id", chat.to_owned());
    let mut described = Vec::new();

    for (position, picture) in pictures.iter().enumerate() {
        let name = format!("photo{position}");

        let bytes = std::fs::read(picture).map_err(|trouble| SendError::NoPicture {
            path: picture.display().to_string(),
            detail: trouble.to_string(),
        })?;

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
        .map_err(|trouble| SendError::Unreachable(trouble.to_string()))?
        .json()
        .await
        .map_err(|trouble| SendError::Unreachable(trouble.to_string()))?;

    // Telegram refuses politely — `ok: false` inside a perfectly normal reply.
    // The same trap Twelve Data sets with its 401, met twice in one afternoon,
    // so it is a pattern rather than bad luck.
    if reply["ok"] != true {
        return Err(SendError::Refused(
            reply["description"]
                .as_str()
                .unwrap_or("no reason given")
                .to_string(),
        ));
    }

    Ok(())
}
