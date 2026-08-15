//! Loading, the calendar, the socket, and the two things that can happen.
//!
//! **Rungs 1 and 2 of the ladder.**
//!
//!     price reaches a zone        an alert card, once per visit
//!     a candle closes there       a card per candle that touched it
//!
//! Silence is the normal state. Prices arrive about once a second and almost
//! all of them are nowhere near anything.
//!
//! **It watches nothing on a Monday.** Not "stays quiet" — nothing checked,
//! nothing fetched, no queue building up to be dumped on him on Tuesday. When
//! Tuesday opens it says what it FOUND rather than pretending it just
//! happened.

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};

use super::{CALENDAR, PAIRS, THICKNESS, Watching, bands, closes, prices, pulse, resumed};
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use nsc_core::levels::{Thickness, Watch, known, load_pair, load_thickness};
use nsc_core::when::{self, Allowed, Rules};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

pub async fn run() -> Result<()> {
    dotenvy::dotenv().ok();

    let client = reqwest::Client::new();
    let thickness = load_thickness(Path::new(THICKNESS))?;
    let calendar = when::load(Path::new(CALENDAR))?;

    let mut watching: HashMap<String, Watching> = HashMap::new();

    // The bands are worked out once, at the start. After this nothing is
    // fetched unless price is actually at one of them.
    for name in known(Path::new(PAIRS)) {
        let pair = load_pair(&Path::new(PAIRS).join(format!("{name}.toml")))?;
        let found = bands::for_pair(&client, &pair, thickness).await?;

        if found.is_empty() {
            println!("{name} — no levels, skipping");
            continue;
        }

        println!("{} — watching {} level(s)", pair.symbol, found.len());
        let watch = Watch::over(found, pair.reach(thickness));
        watching.insert(pair.symbol.clone(), Watching { pair, watch });
    }

    if watching.is_empty() {
        anyhow::bail!("no pairs have levels — send some with the inbox program");
    }

    say_what_the_calendar_allows(&calendar);

    listen(&client, &mut watching, thickness, &calendar).await
}

fn say_what_the_calendar_allows(calendar: &Rules) {
    match when::allowed(Utc::now(), calendar) {
        Allowed::Silence => println!(
            "\ntoday is quiet. Nothing is watched and nothing is fetched.\n\
             It will start speaking when the next session opens."
        ),
        Allowed::WatchOnly => println!("\nlistening. Reporting only — no trade suggested yet.\n"),
        Allowed::Anything => {
            println!("\nlistening. Nothing will be said unless price reaches a level.\n")
        }
    }
}

/// Holds the line open and watches every price that comes down it.
async fn listen(
    client: &reqwest::Client,
    watching: &mut HashMap<String, Watching>,
    thickness: Thickness,
    calendar: &Rules,
) -> Result<()> {
    let key = std::env::var("TWELVE_DATA_API_KEY").context("TWELVE_DATA_API_KEY is not set")?;
    let url = format!("wss://ws.twelvedata.com/v1/quotes/price?apikey={key}");

    // Never print `url`. The key is in it.
    let (mut socket, _) = connect_async(&url)
        .await
        .context("the price line would not open")?;

    let symbols: Vec<&str> = watching.keys().map(String::as_str).collect();
    let ask = serde_json::json!({
        "action": "subscribe",
        "params": { "symbols": symbols.join(",") }
    });

    socket
        .send(Message::Text(ask.to_string()))
        .await
        .context("could not ask for prices")?;

    let mut closes = closes::Closes::new();
    let mut awake = resumed::Awake::new();
    let mut pulse = pulse::Pulse::new();

    loop {
        tokio::select! {
            heard = socket.next() => {
                let Some(heard) = heard else { break };
                let heard = heard.context("the price line broke")?;

                if when::allowed(Utc::now(), calendar) == Allowed::Silence {
                    // Still drained, so the socket does not back up. Nothing is
                    // looked at and nothing is remembered.
                    continue;
                }

                awake.greet(client, watching, thickness, &mut pulse).await?;
                prices::heard(client, watching, thickness, &heard, &mut pulse).await?;
            }

            _ = closes.next_check() => {
                closes.tick();

                // THE HEARTBEAT GOES OUT EVEN ON A SILENT DAY. Monday watches
                // nothing, and without this a quiet Monday and a dead bot look
                // exactly the same.
                pulse.maybe(client, watching, calendar).await?;

                if when::allowed(Utc::now(), calendar) == Allowed::Silence {
                    continue;
                }

                closes.look(client, watching, thickness, calendar, &mut pulse).await?;
            }
        }
    }

    println!("the other side hung up.");
    Ok(())
}
