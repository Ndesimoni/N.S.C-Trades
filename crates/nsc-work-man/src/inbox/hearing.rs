//! Asking Telegram what he has sent, over and over, forever.

use anyhow::{Context, Result, anyhow, bail};
use serde_json::Value;
use tokio::sync::watch;

use super::asked;
use super::conversation::{Adding, handle};
use super::talking::{plainly, say};
use crate::places::OWNER;

/// How long to wait before listening again after a failure.
const AGAIN: std::time::Duration = std::time::Duration::from_secs(15);

/// Listens forever, and never gives up.
///
/// **It is spawned beside the watcher, so it must not be able to stop.** If it
/// ends, levels he sends go nowhere and nothing says so — which is the exact
/// failure that made it worth folding in.
pub async fn run(client: reqwest::Client, standing: watch::Receiver<crate::watch::Snapshot>) {
    loop {
        if let Err(trouble) = listen(&client, &standing).await {
            eprintln!("The inbox stopped listening: {trouble:#}");
        }

        tokio::time::sleep(AGAIN).await;
    }
}

async fn listen(
    client: &reqwest::Client,
    standing: &watch::Receiver<crate::watch::Snapshot>,
) -> Result<()> {
    let token = std::env::var("TELEGRAM_BOT_TOKEN").context("TELEGRAM_BOT_TOKEN is not set")?;
    let mut adding = Adding::default();
    let mut seen_up_to: i64 = 0;

    asked::register(client, &token).await;

    loop {
        // Telegram holds the line open rather than answering "nothing"
        // instantly. One request every thirty seconds, not hundreds.
        //
        // **`web::AT_MOST` has to be longer than this**, or every poll is cut
        // short and reads as Telegram being unreachable.
        let url = format!(
            "https://api.telegram.org/bot{token}/getUpdates?offset={}&timeout={}",
            seen_up_to + 1,
            crate::inbox::HELD_OPEN
        );

        let reply: Value = client
            .get(&url)
            .send()
            .await
            .map_err(|trouble| anyhow!("could not reach Telegram: {}", trouble.without_url()))?
            .json()
            .await
            .context("Telegram answered, but not with JSON")?;

        for update in updates(&reply)? {
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

            if let Err(trouble) = handle(client, &token, text.trim(), &mut adding, standing).await {
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

/// **What Telegram's answer to a poll actually means.**
///
/// Only `result` was ever read, and a refusal carries no `result` — so a
/// refused poll looked exactly like a quiet minute and the loop span on
/// silently, forever, while he sent messages nothing was reading.
fn updates(reply: &Value) -> Result<Vec<Value>> {
    if reply["ok"] == true {
        return Ok(reply["result"].as_array().cloned().unwrap_or_default());
    }

    let why = reply["description"]
        .as_str()
        .unwrap_or("Telegram gave no reason");

    // **The one he will actually hit.** Telegram hands each message to
    // whichever copy asks first, so two running bots split his messages
    // between them at random and each looks like it is ignoring him.
    if reply["error_code"] == 409 {
        bail!("another copy of this bot is already running — {why}");
    }

    bail!("Telegram would not hand over messages: {why}")
}

#[cfg(test)]
mod tests {
    use super::updates;
    use serde_json::json;

    #[test]
    fn a_quiet_minute_is_no_messages_and_no_trouble() {
        let said = json!({ "ok": true, "result": [] });

        assert_eq!(updates(&said).expect("no trouble").len(), 0);
    }

    #[test]
    fn messages_come_back_in_order() {
        let said = json!({ "ok": true, "result": [{ "update_id": 1 }, { "update_id": 2 }] });
        let got = updates(&said).expect("no trouble");

        assert_eq!(got.len(), 2);
        assert_eq!(got[0]["update_id"], 1);
    }

    /// **The one that was silent.** Two copies running is the likeliest way
    /// this ever fails, and the old code read it as a quiet minute — so the
    /// second copy span forever, saying nothing, while his messages went to
    /// the other one at random.
    #[test]
    fn two_copies_running_is_said_out_loud() {
        let said = json!({
            "ok": false,
            "error_code": 409,
            "description": "Conflict: terminated by other getUpdates request",
        });

        let trouble = updates(&said).expect_err("that is not a quiet minute");
        assert!(
            trouble.to_string().contains("another copy"),
            "it has to say WHICH thing is wrong, got: {trouble}"
        );
    }

    /// A bad token, a revoked bot. Different evening, same rule: say so.
    #[test]
    fn any_other_refusal_is_said_too() {
        let said = json!({ "ok": false, "error_code": 401, "description": "Unauthorized" });

        assert!(updates(&said).is_err());
    }
}
