//! Loading, the calendar, the line, and the two things that can happen.
//!
//! **It is meant to run for weeks.** Everything here is shaped by that: the
//! line will drop, and dropping must not be the end of it.

use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use chrono::Utc;
use nsc_core::levels::load_thickness;
use nsc_core::when::{self, Allowed, Rules};
use nsc_data::sources::ibkr::IbkrConnection;

use crate::places::{CALENDAR, NEWS, PATTERNS, STRATEGY, THICKNESS};
use crate::watch::line::{self, Closed};
use crate::watch::standing;
use crate::watch::{Kit, reload, trouble};

use super::armed::armed;
use super::picture::snapshot;
use super::retiring;

/// How long to wait before opening the line again after it drops.
///
/// Long enough not to hammer them if they are refusing connections, short
/// enough that a hiccup costs him half a minute of watching.
const AGAIN: std::time::Duration = std::time::Duration::from_secs(30);

/// How often to wake on a day nothing is watched.
///
/// **A minute, because this is also how late the session can open.** Nothing
/// is fetched on a quiet day, so waking costs a look at the calendar and a
/// glance at four files — and sleeping through the moment the market opens
/// costs him the first ten minutes of it.
///
/// It is also how long a level he sends at the weekend sits unarmed, and the
/// weekend is when he does his chart work.
const WHILE_QUIET: std::time::Duration = std::time::Duration::from_secs(60);

pub async fn run() -> Result<()> {
    crate::secrets::load();

    let client = crate::web::client();
    let thickness = load_thickness(Path::new(THICKNESS))?;
    let calendar = when::load(Path::new(CALENDAR))?;

    // **The line to TWS, opened once.** Every candle and every price comes
    // through it. Failing here is fatal on purpose — unlike a web API this
    // feed needs a program logged in, and a bot that starts up watching
    // nothing is worse than one that says why it did not start.
    let mut ibkr = IbkrConnection::connect().await?;

    // **Before anything is sized: take out the pairs IBKR cannot serve.**
    //
    // A pair it has never heard of is not a pair that is quiet — it is one
    // that can never report anything. Left in place it costs a refused
    // request on every reload and shows up on /pairs and the heartbeat as
    // though it were being watched.
    retiring::the_unservable(&client, &ibkr).await;

    // The same reading used when he sends a level mid-run. One way of turning
    // the files into bands, so the two cannot drift.
    let first = reload::again(&ibkr, thickness, HashMap::new()).await?;
    let mut watching = first.watching;

    // **Two different nothings, and they need different words.** No levels is
    // something he fixes from his phone in a minute. A feed that will not
    // answer is nothing to do with him, and saying "no pairs have levels"
    // would send him looking in the wrong place.
    if watching.is_empty() {
        if first.not_sized.is_empty() {
            anyhow::bail!("no pairs have levels — send the bot /level to add some");
        }

        anyhow::bail!(
            "could not reach the feed to size any bands ({})",
            first.not_sized.join(", ")
        );
    }

    // **The economic calendar runs beside the watcher too**, and on its own
    // clock. It needs no prices, no bands and no IBKR — only the time and the
    // internet — so it does not belong inside the price loop, which blocks for
    // hours at a stretch waiting on the socket.
    //
    // **A missing or broken config/news.toml does NOT stop the bot.** Saying
    // what price is doing at his levels is the job; knowing what is on the
    // calendar is an addition to it. Refusing to start over the addition would
    // trade the whole thing for a part of it.
    //
    // It is said out loud rather than swallowed, because news that quietly
    // never arrives looks exactly like a quiet week.
    match nsc_core::news::load(Path::new(NEWS)) {
        Ok(rules) => {
            let ahead: Vec<String> = {
                let mut marks = rules.warn_at_minutes.clone();
                marks.sort_unstable();
                marks.dedup();
                marks.reverse();
                marks.iter().map(|mark| mark.to_string()).collect()
            };

            println!(
                "Watching the economic calendar — {} events get a card {} minutes ahead.",
                rules.impacts.join(" and ").to_lowercase(),
                ahead.join(" and ")
            );
            tokio::spawn(crate::watch::watch_the_news(client.clone(), rules));
        }
        Err(trouble) => eprintln!(
            "No news warnings: {trouble}\n\
             Everything else is running. Fix {NEWS} and restart to turn them on."
        ),
    }

    // **The inbox runs beside the watcher**, so one command is the whole bot.
    // It was a second program, which meant two terminals and remembering both
    // — and if it was not up, a level he sent went nowhere and nothing said so.
    //
    // They do not talk to each other about levels: the inbox writes a file and
    // the watcher notices it changed. But `/status` has to answer from the
    // LIVE picture — which bands are sized, where price was last — so the
    // watcher publishes a copy of that and the inbox reads the latest.
    let (tell, standing) = standing::channel(snapshot(&watching, &calendar));
    tokio::spawn(crate::inbox::run(client.clone(), standing));

    // This outlives the socket. Rebuilt on every reconnect, a dropped line
    // would re-announce every zone price is already at and forget which
    // candles it had already reported.
    // **Rung 3's settings, read once.** A shape he trades, at a level he drew.
    //
    // **Unreadable settings turn rung 3 off and leave everything else
    // running.** The alerts and the closes are the job; the shape at the level
    // was added on top of them, and refusing to start over the addition would
    // trade the whole thing for a part of it.
    //
    // Said out loud rather than swallowed, because rules that quietly never
    // fire look exactly like a quiet week.
    let rung_three = match crate::watch::rung_three(STRATEGY, PATTERNS) {
        Ok(both) => {
            println!("Rung 3 is on — a shape you trade, at a level you drew.");
            Some(both)
        }
        Err(trouble) => {
            eprintln!(
                "Rung 3 is OFF: {trouble:#}\n\
                 Alerts and closes still run. Fix {STRATEGY} and restart."
            );
            None
        }
    };

    // ── The record, if there is one to reach ──
    //
    // **A database that will not open must not stop the bot.** It is written
    // and never read while the bot is up, so the cost of losing it is a gap in
    // the history — where the cost of refusing to start is every alert he was
    // watching for.
    //
    // It is said out loud rather than swallowed, because a record that quietly
    // stopped filling looks exactly like a quiet week when it is read back
    // months later.
    let record = match std::env::var("DATABASE_URL") {
        Err(_) => {
            println!("No DATABASE_URL — finished candles will not be kept.");
            None
        }

        Ok(url) => match nsc_data::store::open(&url).await {
            Ok(store) => {
                println!("Keeping finished candles in the record.");
                Some(store)
            }

            Err(trouble) => {
                eprintln!(
                    "\n⚠️  Could not open the record, so finished candles will not be kept.\n\
                         {trouble}\n\
                         Everything else runs. `docker compose up -d` and restart.\n"
                );
                None
            }
        },
    };

    let mut kit = Kit::new(rung_three, record);
    let mut trouble = trouble::Trouble::new();

    say_what_the_calendar_allows(&calendar);

    // **Forever.** The line dropping is not a reason to stop — and it used to
    // be: the socket closing returned Ok and the process exited successfully.
    // The heartbeat went with it, so a dead bot and a quiet day looked exactly
    // the same, which is the one thing the heartbeat exists to tell apart.
    loop {
        // **Nothing to watch is not a reason to open a line.** Removing the
        // last pair left this subscribing to no symbols at all, and the feed's
        // answer to that came back looking exactly like every pair being
        // refused — so it reported the price line as down, every thirty
        // seconds, over a bot doing precisely what he asked.
        if watching.is_empty() || when::allowed(Utc::now(), &calendar) == Allowed::Silence {
            // Nothing to watch, so nothing is opened. The heartbeat still
            // goes out — that is the whole point of it on a quiet day.
            kit.pulse.maybe(&client, &watching, &calendar).await?;

            // **And levels are still picked up.** The weekend is exactly when
            // he does his chart work, and the check used to live inside the
            // socket loop — which does not run on a quiet day. A level sent on
            // Sunday would have sat there unarmed until Tuesday.
            if kit.files.changed() {
                println!("The levels changed. Reading them again.");
                watching = armed(&client, &ibkr, thickness, watching, &mut kit).await?;
            }

            let _ = tell.send(snapshot(&watching, &calendar));

            tokio::time::sleep(WHILE_QUIET).await;
            continue;
        }

        let closed = line::listen(
            &client,
            &ibkr,
            &mut watching,
            thickness,
            &calendar,
            &mut kit,
            &tell,
        )
        .await;

        match closed {
            // The line closed cleanly, the session did, or he sent a level.
            // Nothing is wrong.
            Ok(Closed::Line) => trouble.mended(&client, &mut kit.pulse).await?,

            // **He sent a level.** Read them again and open the line to the
            // new set — the subscription is fixed when the socket opens, so a
            // pair added to a live one would never be asked about.
            Ok(Closed::LevelsChanged) => {
                trouble.mended(&client, &mut kit.pulse).await?;
                watching = armed(&client, &ibkr, thickness, watching, &mut kit).await?;

                // Straight back, no thirty-second pause. He is standing there
                // having just sent it.
                continue;
            }

            Err(broke) => {
                eprintln!("The price line broke: {broke:#}");
                trouble
                    .broke(&client, &format!("{broke:#}"), &calendar, &mut kit.pulse)
                    .await?;

                // **The connection itself may be what died.** TWS restarting,
                // or the Mac sleeping, leaves a `Client` that will refuse
                // every subscription from now on — and subscribing again on a
                // dead one fails identically forever. Opening a fresh line is
                // the only thing that fixes it.
                //
                // Failing to reconnect keeps the old line rather than stopping.
                // The gateway may be halfway through starting up, and the next
                // pass round is thirty seconds away.
                match IbkrConnection::connect().await {
                    Ok(fresh) => ibkr = fresh,
                    Err(trouble) => eprintln!("Could not reach IBKR again: {trouble:#}"),
                }
            }
        }

        eprintln!("Opening it again in {} seconds.", AGAIN.as_secs());
        tokio::time::sleep(AGAIN).await;
    }
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

    println!("Send the bot /help. Its commands are in the menu beside the box.\n");
}
