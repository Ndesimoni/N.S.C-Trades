//! Holding the price line open, and everything that can end it.

use std::collections::HashMap;

use anyhow::{Context, Result};
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use nsc_core::levels::Thickness;
use nsc_core::when::{self, Allowed, Rules};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use super::run::snapshot;
use super::standing::Snapshot;
use super::{Kit, Watching, prices};
use tokio::sync::watch as tell_of;

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
    tell: &tell_of::Sender<Snapshot>,
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

                // **The opening hours are watched but not spoken about.** A
                // zone touched at the open and abandoned twenty minutes later
                // is noise, and a buzz he learns to ignore costs him the one
                // that mattered. Prices are still fed in, so when the window
                // ends the greeting knows exactly where price stands.
                let settled = when::settled(Utc::now(), calendar);

                // **The price is recorded first, and the order matters.**
                //
                // The greeting reports which zones price is RESTING IN, and
                // nothing is resting anywhere until a price has been fed in. On
                // the very first price of a session the greeting used to run
                // first, find nothing, send nothing — and mark the session
                // greeted. The report of where price already stood, which is
                // the whole reason this waits for the opening hours to pass,
                // never came at all.
                prices::heard(client, watching, thickness, &heard, &mut kit.pulse, settled)
                    .await?;

                kit.awake
                    .greet(client, watching, thickness, calendar, &mut kit.pulse)
                    .await?;
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

                // Held through the opening hours, same as the price alerts.
                // The first candle report after the window covers what
                // happened during it.
                if when::settled(Utc::now(), calendar) {
                    kit.closes
                        .look(client, watching, thickness, calendar, &mut kit.pulse)
                        .await?;
                }

                // Where price last was has moved on. Anything asking /status
                // should see today, not where it stood when the line opened.
                let _ = tell.send(snapshot(watching, calendar));
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

    // **Asking about nothing is not the same as being refused everything.**
    // Nought equals nought, so an empty subscription used to read as a total
    // refusal and reported the line as broken.
    if !asked.is_empty() && failed.len() == asked.len() {
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

#[cfg(test)]
mod tests {
    use super::check_the_answer;
    use futures_util::stream;
    use tokio_tungstenite::tungstenite::Message;

    /// One reply, then the stream ends.
    fn answering(
        said: &str,
    ) -> impl futures_util::StreamExt<Item = Result<Message, Error>> + Unpin
where {
        stream::iter(vec![Ok(Message::Text(said.to_string()))])
    }

    use tokio_tungstenite::tungstenite::Error;

    /// **Nought refused out of nought asked is not a total refusal.**
    ///
    /// Removing the last pair left this subscribing to no symbols. The feed
    /// answered with an empty fails list, `0 == 0` came out true, and it
    /// reported every pair as refused — which `run` then treated as the price
    /// line breaking and told him so.
    #[tokio::test]
    async fn asking_about_nothing_is_not_being_refused_everything() {
        let said = r#"{"event":"subscribe-status","status":"ok","fails":[]}"#;

        let answer = check_the_answer(&mut answering(said), &[]).await;
        assert!(answer.is_ok(), "it should not call that a refusal");
    }

    /// Every pair refused really is fatal — nothing would ever arrive.
    #[tokio::test]
    async fn every_pair_refused_still_stops() {
        let said = r#"{"event":"subscribe-status","fails":[{"symbol":"XAU/USD"}]}"#;

        let answer = check_the_answer(&mut answering(said), &["XAU/USD".to_string()]).await;
        assert!(answer.is_err());
    }

    /// Some refused, some not: said out loud, but it carries on.
    #[tokio::test]
    async fn one_bad_pair_does_not_stop_the_others() {
        let said = r#"{"event":"subscribe-status","fails":[{"symbol":"XAU/USD"}]}"#;
        let asked = ["XAU/USD".to_string(), "EUR/USD".to_string()];

        assert!(check_the_answer(&mut answering(said), &asked).await.is_ok());
    }
}
