//! The sending itself.

use std::path::Path;

use super::SendError;

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
        .map_err(|trouble| SendError::Unreachable(quietly(trouble)))?
        .json()
        .await
        .map_err(|trouble| SendError::Unreachable(quietly(trouble)))?;

    // Telegram refuses politely — `ok: false` inside a perfectly normal reply.
    // The same trap a feed sets when it refuses with a 200 and an error code
    // in the body, met twice in one afternoon,
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

/// **One picture, with buttons under it.**
///
/// A `sendPhoto`, so the keyboard can ride on the card itself.
///
/// **This is the one thing `sendMediaGroup` cannot do.** Telegram refuses
/// `reply_markup` on a group of photos, so a setup that goes out as one
/// grouped message can never carry its own buttons — they had to arrive as a
/// separate text message underneath, which is what he saw and did not want:
/// *"they are in a different card. Feed them in the same card."*
///
/// So the two charts go as a group and the setup card comes through here, with
/// the tick and the cross on it.
pub async fn send_with_buttons(
    client: &reqwest::Client,
    chat: &str,
    picture: &Path,
    caption: &str,
    keyboard: serde_json::Value,
) -> Result<(), SendError> {
    let token =
        std::env::var("TELEGRAM_BOT_TOKEN").map_err(|_| SendError::NotSet("TELEGRAM_BOT_TOKEN"))?;

    let bytes = std::fs::read(picture).map_err(|trouble| SendError::NoPicture {
        path: picture.display().to_string(),
        detail: trouble.to_string(),
    })?;

    let name = picture
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "card.png".into());

    let form = reqwest::multipart::Form::new()
        .text("chat_id", chat.to_string())
        .text("caption", caption.to_string())
        .text("parse_mode", "HTML")
        .text("reply_markup", keyboard.to_string())
        .part(
            "photo",
            reqwest::multipart::Part::bytes(bytes).file_name(name),
        );

    let reply: serde_json::Value = client
        .post(format!("https://api.telegram.org/bot{token}/sendPhoto"))
        .multipart(form)
        .send()
        .await
        .map_err(|trouble| SendError::Unreachable(quietly(trouble)))?
        .json()
        .await
        .map_err(|trouble| SendError::Unreachable(quietly(trouble)))?;

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

/// **Words with buttons under them.**
///
/// The same message as [`send_words`], plus an inline keyboard.
///
/// It exists because **Telegram does not allow buttons on a media group**, and
/// a setup goes out as three pictures in one. So the pictures land and this
/// follows them, carrying the two buttons and a line saying which setup they
/// belong to.
pub async fn ask_words(
    client: &reqwest::Client,
    chat: &str,
    text: &str,
    keyboard: serde_json::Value,
) -> Result<(), SendError> {
    let token =
        std::env::var("TELEGRAM_BOT_TOKEN").map_err(|_| SendError::NotSet("TELEGRAM_BOT_TOKEN"))?;

    let reply: serde_json::Value = client
        .post(format!("https://api.telegram.org/bot{token}/sendMessage"))
        .json(&serde_json::json!({
            "chat_id": chat,
            "text": text,
            "parse_mode": "HTML",
            "reply_markup": keyboard,
        }))
        .send()
        .await
        .map_err(|trouble| SendError::Unreachable(quietly(trouble)))?
        .json()
        .await
        .map_err(|trouble| SendError::Unreachable(quietly(trouble)))?;

    // **Refused is not sent.** Answering Ok here would have everything
    // upstream believe he had been asked when he had not.
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

/// Words on their own, with no picture.
///
/// **Alerts stopped using this** — they go as a card now, because Telegram
/// gives text no colour, no size and no layout, so every message ended up
/// looking like every other one. See `card::alert`.
///
/// The heartbeat moved to a card too, for the same reason. What is left for
/// this is anything genuinely one line and not worth a picture — errors worth
/// telling him about, and replies while he is sending levels.
pub async fn send_words(client: &reqwest::Client, chat: &str, text: &str) -> Result<(), SendError> {
    let token =
        std::env::var("TELEGRAM_BOT_TOKEN").map_err(|_| SendError::NotSet("TELEGRAM_BOT_TOKEN"))?;

    let reply: serde_json::Value = client
        .post(format!("https://api.telegram.org/bot{token}/sendMessage"))
        .json(&serde_json::json!({
            "chat_id": chat,
            "text": text,
            "parse_mode": "HTML",
        }))
        .send()
        .await
        .map_err(|trouble| SendError::Unreachable(quietly(trouble)))?
        .json()
        .await
        .map_err(|trouble| SendError::Unreachable(quietly(trouble)))?;

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

/// A network failure, with the address taken off it.
///
/// **THE TOKEN IS IN THE URL**, and `reqwest` puts the URL it was trying into
/// the message. So "could not reach Telegram" arrived in the terminal with the
/// bot token printed in full — and from there into any log, any screenshot,
/// any paste of what went wrong.
///
/// It is the same rule already followed for the feed's key, which was written
/// down as "never print the url" and then not applied to the error path.
fn quietly(trouble: reqwest::Error) -> String {
    trouble.without_url().to_string()
}
