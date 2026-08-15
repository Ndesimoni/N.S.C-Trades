//! Loading, the calendar, the socket, and the two things that can happen.
//!
//! **It is meant to run for weeks.** Everything here is shaped by that: the
//! line will drop, and dropping must not be the end of it.

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use nsc_core::levels::{Thickness, load_thickness};
use nsc_core::when::{self, Allowed, Rules};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use super::{CALENDAR, THICKNESS, Watching, closes, prices, pulse, reload, resumed, trouble};

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

    // The same reading used when he sends a level mid-run. One way of turning
    // the files into bands, so the two cannot drift.
    let (mut watching, _) = reload::again(&client, thickness, HashMap::new()).await?;

    if watching.is_empty() {
        anyhow::bail!("no pairs have levels — send some with the inbox program");
    }

    // These outlive the socket. Rebuilt on every reconnect, a dropped line
    // would re-announce every zone price is already at and forget which
    // candles it had already reported.
    let mut closes = closes::Closes::new();
    let mut awake = resumed::Awake::new();
    let mut pulse = pulse::Pulse::new();
    let mut trouble = trouble::Trouble::new();
    let mut files = reload::Files::look();

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

            // **And levels are still picked up.** The weekend is exactly when
            // he does his chart work, and the check used to live inside the
            // socket loop — which does not run on a quiet day. A level sent on
            // Sunday would have sat there unarmed until Tuesday.
            if files.changed() {
                println!("The levels changed. Reading them again.");
                watching = armed(&client, thickness, watching, &mut pulse).await?;
            }

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
            &mut files,
        )
        .await
        {
            // The line closed cleanly, the session did, or he sent a level.
            // Nothing is wrong.
            Ok(Closed::Line) => trouble.mended(&client, &mut pulse).await?,

            // **He sent a level.** Read them again and open the line to the
            // new set — the subscription is fixed when the socket opens, so a
            // pair added to a live one would never be asked about.
            Ok(Closed::LevelsChanged) => {
                trouble.mended(&client, &mut pulse).await?;
                watching = armed(&client, thickness, watching, &mut pulse).await?;

                // Straight back, no thirty-second pause. He is standing there
                // having just sent it.
                continue;
            }

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

/// Why the line stopped being held open.
enum Closed {
    /// The other side hung up, or the session ended.
    Line,

    /// A levels file changed. He has sent something.
    LevelsChanged,
}

/// Reads the levels again and tells him which are now being watched.
async fn armed(
    client: &reqwest::Client,
    thickness: Thickness,
    watching: HashMap<String, Watching>,
    pulse: &mut pulse::Pulse,
) -> Result<HashMap<String, Watching>> {
    let (fresh, armed) = reload::again(client, thickness, watching).await?;

    if armed {
        reload::say_it_is_armed(client, &fresh, pulse).await?;
    }

    Ok(fresh)
}

/// Says out loud what the calendar is allowing, so the terminal is not silent
/// for reasons he cannot see.
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
/// **Returns when the line closes, when the session goes quiet, or when he
/// sends a level.** The caller decides what happens next; this does not.
#[allow(clippy::too_many_arguments)]
async fn listen(
    client: &reqwest::Client,
    watching: &mut HashMap<String, Watching>,
    thickness: Thickness,
    calendar: &Rules,
    closes: &mut closes::Closes,
    awake: &mut resumed::Awake,
    pulse: &mut pulse::Pulse,
    files: &mut reload::Files,
) -> Result<Closed> {
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
                    return Ok(Closed::Line);
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
                    return Ok(Closed::Line);
                }

                // Checked by the clock on the files, so the normal answer —
                // nothing happened — costs one look at a folder.
                if files.changed() {
                    println!("The levels changed. Reading them again.");
                    return Ok(Closed::LevelsChanged);
                }

                closes.look(client, watching, thickness, calendar, pulse).await?;
            }
        }
    }
}
