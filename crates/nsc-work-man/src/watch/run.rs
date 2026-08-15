//! Loading, the calendar, the socket, and the two things that can happen.
//!
//! **It is meant to run for weeks.** Everything here is shaped by that: the
//! line will drop, and dropping must not be the end of it.

use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use chrono::Utc;
use nsc_core::levels::{Thickness, load_thickness};
use nsc_core::when::{self, Allowed, Rules};

use super::line::{self, Closed};
use super::{CALENDAR, Kit, THICKNESS, Watching, pulse, reload, trouble};

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

    // **The inbox runs beside the watcher**, so one command is the whole bot.
    // It was a second program, which meant two terminals and remembering both
    // — and if it was not up, a level he sent went nowhere and nothing said so.
    //
    // They do not talk to each other. The inbox writes a file and the watcher
    // notices it changed, which is how it already worked.
    tokio::spawn(crate::inbox::run(client.clone()));

    // The same reading used when he sends a level mid-run. One way of turning
    // the files into bands, so the two cannot drift.
    let (mut watching, _) = reload::again(&client, thickness, HashMap::new()).await?;

    if watching.is_empty() {
        anyhow::bail!("no pairs have levels — send the bot /level to add some");
    }

    // This outlives the socket. Rebuilt on every reconnect, a dropped line
    // would re-announce every zone price is already at and forget which
    // candles it had already reported.
    let mut kit = Kit::new();
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
            kit.pulse.maybe(&client, &watching, &calendar).await?;

            // **And levels are still picked up.** The weekend is exactly when
            // he does his chart work, and the check used to live inside the
            // socket loop — which does not run on a quiet day. A level sent on
            // Sunday would have sat there unarmed until Tuesday.
            if kit.files.changed() {
                println!("The levels changed. Reading them again.");
                watching = armed(&client, thickness, watching, &mut kit.pulse).await?;
            }

            tokio::time::sleep(WHILE_QUIET).await;
            continue;
        }

        match line::listen(&client, &mut watching, thickness, &calendar, &mut kit).await {
            // The line closed cleanly, the session did, or he sent a level.
            // Nothing is wrong.
            Ok(Closed::Line) => trouble.mended(&client, &mut kit.pulse).await?,

            // **He sent a level.** Read them again and open the line to the
            // new set — the subscription is fixed when the socket opens, so a
            // pair added to a live one would never be asked about.
            Ok(Closed::LevelsChanged) => {
                trouble.mended(&client, &mut kit.pulse).await?;
                watching = armed(&client, thickness, watching, &mut kit.pulse).await?;

                // Straight back, no thirty-second pause. He is standing there
                // having just sent it.
                continue;
            }

            Err(broke) => {
                eprintln!("The price line broke: {broke:#}");
                trouble
                    .broke(&client, &format!("{broke:#}"), &calendar, &mut kit.pulse)
                    .await?;
            }
        }

        eprintln!("Opening it again in {} seconds.", AGAIN.as_secs());
        tokio::time::sleep(AGAIN).await;
    }
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

    println!("Send the bot /pairs to see them, or /level to add. docs/telegram.md.\n");
}
