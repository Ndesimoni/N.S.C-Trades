//! Holding the price line open, and everything that can end it.

use std::collections::HashMap;

use anyhow::{Context, Result};
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use nsc_core::levels::Thickness;
use nsc_core::when::{self, Allowed, Rules};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use super::{Kit, Watching, prices};

/// Why the line stopped being held open.
pub enum Closed {
    /// The other side hung up, or the session ended.
    Line,

    /// A levels file changed. He has sent something.
    LevelsChanged,
}

/// Holds the line open and watches every price that comes down it.
///
/// **Returns when the line closes, when the session goes quiet, or when he
/// sends a level.** The caller decides what happens next; this does not.
pub async fn listen(
    client: &reqwest::Client,
    watching: &mut HashMap<String, Watching>,
    thickness: Thickness,
    calendar: &Rules,
    kit: &mut Kit,
) -> Result<Closed> {
    let key = std::env::var("TWELVE_DATA_API_KEY").context("TWELVE_DATA_API_KEY is not set")?;
    let url = format!("wss://ws.twelvedata.com/v1/quotes/price?apikey={key}");

    // Never print `url`. The key is in it.
    let (mut socket, _) = connect_async(&url)
        .await
        .context("the price line would not open")?;

    let symbols: Vec<String> = watching.keys().cloned().collect();
    let ask = serde_json::json!({
        "action": "subscribe",
        "params": { "symbols": symbols.join(",") }
    });

    socket
        .send(Message::Text(ask.to_string()))
        .await
        .context("could not ask for prices")?;

    check_the_answer(&mut socket, &symbols).await?;

    loop {
        tokio::select! {
            heard = socket.next() => {
                let Some(heard) = heard else {
                    println!("The other side hung up.");
                    return Ok(Closed::Line);
                };

                let heard = heard.context("the price line broke")?;

                kit.awake.greet(client, watching, thickness, &mut kit.pulse).await?;
                prices::heard(client, watching, thickness, &heard, &mut kit.pulse).await?;
            }

            _ = kit.closes.next_check() => {
                kit.closes.tick();

                // The heartbeat is checked here as well as in `run`, because a
                // busy line means this loop is where the time is spent.
                kit.pulse.maybe(client, watching, calendar).await?;

                // Gone quiet — the weekend, or Monday. Hand back and let `run`
                // put the socket away rather than draining a line nobody is
                // reading.
                if when::allowed(Utc::now(), calendar) == Allowed::Silence {
                    println!("The session has closed. Standing down.");
                    return Ok(Closed::Line);
                }

                // Checked by the clock on the files, so the normal answer —
                // nothing happened — costs one look at a folder.
                if kit.files.changed() {
                    println!("The levels changed. Reading them again.");
                    return Ok(Closed::LevelsChanged);
                }

                kit.closes.look(client, watching, thickness, calendar, &mut kit.pulse).await?;
            }
        }
    }
}

/// **Reads what they said to the subscription**, rather than assuming it took.
///
/// They answer with `status`, a `success` list and a `fails` list. Nothing read
/// it, and the failure that hides in that gap is a quiet one: a pair they will
/// not serve, or a key over its quota, gets refused *per symbol* while the
/// socket stays perfectly open. No prices ever arrive for it, nothing errors,
/// and the bot sits there looking like a market where nothing is happening.
///
/// A guard was added earlier for the socket closing without a word — but the
/// subscription reply IS a word, so it defeated that guard as well.
async fn check_the_answer<S>(socket: &mut S, asked: &[String]) -> Result<()>
where
    S: StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    let Some(heard) = socket.next().await else {
        anyhow::bail!("the line shut before answering the subscription");
    };

    let heard = heard.context("the line broke while answering the subscription")?;
    let Ok(said) = serde_json::from_str::<serde_json::Value>(&heard.to_string()) else {
        // Not their status message. Prices flow, which is the answer that
        // matters — leave it be rather than refusing to start over a stray.
        return Ok(());
    };

    if said["event"] != "subscribe-status" {
        return Ok(());
    }

    let failed: Vec<String> = said["fails"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|item| item["symbol"].as_str().map(str::to_owned))
        .collect();

    if failed.len() == asked.len() {
        anyhow::bail!("they refused every pair: {}", failed.join(", "));
    }

    if !failed.is_empty() {
        // Not fatal — the rest are being watched. But it must be said, because
        // one silent pair looks exactly like one quiet pair.
        eprintln!(
            "They will not send prices for: {}. Watching the other {}.",
            failed.join(", "),
            asked.len() - failed.len(),
        );
    }

    Ok(())
}
