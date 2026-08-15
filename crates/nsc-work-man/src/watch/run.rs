//! Loading, the calendar, the socket, and the two things that can happen.
//!
//! **It is meant to run for weeks.** Everything here is shaped by that: the
//! line will drop, and dropping must not be the end of it.

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use nsc_core::levels::{Thickness, Watch, known, load_pair, load_thickness};
use nsc_core::when::{self, Allowed, Rules};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use super::{CALENDAR, PAIRS, THICKNESS, Watching, bands, closes, prices, pulse, resumed, trouble};

/// How long to wait before opening the line again after it drops.
///
/// Long enough not to hammer them if they are refusing connections, short
/// enough that a hiccup costs him half a minute of watching.
const AGAIN: std::time::Duration = std::time::Duration::from_secs(30);

/// How often to wake on a day nothing is watched.
///
/// **Only the heartbeat happens on those days**, and it is due at one moment.
/// Ten minutes is close enough to it and cheap — nothing is fetched.
const WHILE_QUIET: std::time::Duration = std::time::Duration::from_secs(600);

pub async fn run() -> Result<()> {
    dotenvy::dotenv().ok();

    let client = reqwest::Client::new();
    let thickness = load_thickness(Path::new(THICKNESS))?;
    let calendar = when::load(Path::new(CALENDAR))?;

    let mut watching = load(&client, thickness).await?;

    // These outlive the socket. Rebuilt on every reconnect, a dropped line
    // would re-announce every zone price is already at and forget which
    // candles it had already reported.
    let mut closes = closes::Closes::new();
    let mut awake = resumed::Awake::new();
    let mut pulse = pulse::Pulse::new();
    let mut trouble = trouble::Trouble::new();

    say_what_the_calendar_allows(&calendar);

    // **Forever.** The line dropping is not a reason to stop — and it used to
    // be: the socket closing returned Ok and the process exited successfully.
    // The heartbeat went with it, so a dead bot and a quiet day looked exactly
    // the same, which is the one thing the heartbeat exists to tell apart.
    loop {
        if when::allowed(Utc::now(), &calendar) == Allowed::Silence {
            // Nothing to watch, so nothing is opened. The heartbeat still
            // goes out — that is the whole point of it on a quiet day.
            pulse.maybe(&client, &watching, &calendar).await?;
            tokio::time::sleep(WHILE_QUIET).await;
            continue;
        }

        match listen(
            &client,
            &mut watching,
            thickness,
            &calendar,
            &mut closes,
            &mut awake,
            &mut pulse,
        )
        .await
        {
            // The line closed cleanly, or the session did. Nothing is wrong.
            Ok(()) => trouble.mended(&client, &mut pulse).await?,

            Err(broke) => {
                eprintln!("The price line broke: {broke:#}");
                trouble
                    .broke(&client, &format!("{broke:#}"), &calendar, &mut pulse)
                    .await?;
            }
        }

        eprintln!("Opening it again in {} seconds.", AGAIN.as_secs());
        tokio::time::sleep(AGAIN).await;
    }
}

/// Every pair that has a levels file, with its bands sized.
///
/// **The only fetching that happens no matter what.** After this nothing is
/// asked for unless price is actually at one of them.
async fn load(client: &reqwest::Client, thickness: Thickness) -> Result<HashMap<String, Watching>> {
    let mut watching: HashMap<String, Watching> = HashMap::new();

    for name in known(Path::new(PAIRS)) {
        let pair = load_pair(&Path::new(PAIRS).join(format!("{name}.toml")))?;
        let found = bands::for_pair(client, &pair, thickness).await?;

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

    Ok(watching)
}

fn say_what_the_calendar_allows(calendar: &Rules) {
    match when::allowed(Utc::now(), calendar) {
        Allowed::Silence => println!(
            "\nToday is quiet. Nothing is watched and nothing is fetched.\n\
             It will open the line when the next session does.\n"
        ),
        Allowed::WatchOnly => {
            println!("\nListening. Reporting only — no trade is suggested yet.\n")
        }
        Allowed::Anything => {
            println!("\nListening. Nothing will be said unless price reaches a level.\n")
        }
    }
}

/// Holds the line open and watches every price that comes down it.
///
/// **Returns when the line closes, or when the session goes quiet.** Either
/// way the caller decides what happens next; this does not.
#[allow(clippy::too_many_arguments)]
async fn listen(
    client: &reqwest::Client,
    watching: &mut HashMap<String, Watching>,
    thickness: Thickness,
    calendar: &Rules,
    closes: &mut closes::Closes,
    awake: &mut resumed::Awake,
    pulse: &mut pulse::Pulse,
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

    // A line that opens and shuts without a word is not working — a key over
    // its quota does exactly that. Returning Ok for it would have this
    // reconnect every thirty seconds forever without ever saying so, which is
    // the same silence as being dead.
    let mut heard_anything = false;

    loop {
        tokio::select! {
            heard = socket.next() => {
                let Some(heard) = heard else {
                    if !heard_anything {
                        anyhow::bail!("the line opened and shut without sending anything");
                    }

                    println!("The other side hung up.");
                    return Ok(());
                };

                let heard = heard.context("the price line broke")?;
                heard_anything = true;

                awake.greet(client, watching, thickness, pulse).await?;
                prices::heard(client, watching, thickness, &heard, pulse).await?;
            }

            _ = closes.next_check() => {
                closes.tick();

                // The heartbeat is checked here as well as in `run`, because a
                // busy line means this loop is where the time is spent.
                pulse.maybe(client, watching, calendar).await?;

                // Gone quiet — the weekend, or Monday. Hand back and let `run`
                // put the socket away rather than draining a line nobody is
                // reading.
                if when::allowed(Utc::now(), calendar) == Allowed::Silence {
                    println!("The session has closed. Standing down.");
                    return Ok(());
                }

                closes.look(client, watching, thickness, calendar, pulse).await?;
            }
        }
    }
}
