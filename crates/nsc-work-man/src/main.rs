//! A signal bot. It sends signals and places no trades.
//!
//! Right now it does one thing, which is the whole of step one: fetch the most
//! recently **finished** gold candle, draw a card, and send it to Telegram.
//!
//! What that proves is the boring half — the login works, the candle arrives,
//! and the message lands. Those are four things that each eat a week, and they
//! eat it whether they are done first or last.
//!
//! The flow lives here and nothing else does. Each step is a file:
//!
//! ```text
//!   settings.rs   what to fetch, and how to say it
//!   feed.rs       asking Twelve Data
//!   candle/       one candle, and whether it has finished
//!   card.rs       filling in a template, letting Chrome draw it
//!   message.rs    the caption
//!   telegram.rs   sending it
//! ```

use anyhow::{Result, bail};
use chrono::Utc;

use nsc_work_man::candle::Bar;
use nsc_work_man::error::keep_trying;
use nsc_work_man::{card, feed, message, settings, telegram};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let client = reqwest::Client::new();
    let now = Utc::now();

    // Tries again on a hiccup, stops dead on a wrong key. The difference is
    // the whole reason the library has named troubles.
    let series = keep_trying(3, || feed::candles(&client)).await?;

    // Keep only the candles that have finished.
    //
    // The newest is almost always the hour still running, and it must not be
    // drawn or quoted — its high is not its high and its close is not its
    // close. This is the lookahead rule, and it applies to the picture just as
    // much as to any analysis. A wrong price on a chart gets believed exactly
    // like a wrong number in a table.
    let mut finished: Vec<&Bar> = Vec::new();
    for bar in &series.values {
        if bar.is_finished(now)? {
            finished.push(bar);
        }
    }

    // Newest first in the list, so the first finished one is the newest.
    let Some(latest) = finished.first().copied() else {
        bail!("none of the candles that came back have finished yet");
    };

    let caption = message::build(latest)?;
    println!("{caption}\n");

    // Oldest first, because that is the direction a chart is read in.
    let mut oldest_first: Vec<&Bar> = finished.clone();
    oldest_first.reverse();

    // A candle closing is one picture. The chart card carries its own open,
    // high, low and range along the bottom, so it stands up on its own.
    //
    // The other cards in `assets/card/` belong to other messages — a price
    // alert and a signal will each pick their own, which is why `telegram::send`
    // takes a list rather than one picture.
    // Everything drawn lands in preview/, which is gitignored. Stable paths,
    // so docs/visuals.md can point at them and the last card is always there
    // to look at without running anything.
    let chart_card = std::path::Path::new("preview").join("chart.png");
    card::render(
        "chart.html",
        &oldest_first,
        &[],
        settings::SYMBOL,
        settings::INTERVAL,
        settings::DIGITS,
        &chart_card,
    )?;

    let readout_card = std::path::Path::new("preview").join("readout.png");
    card::render(
        "readout.html",
        &oldest_first,
        &[],
        settings::SYMBOL,
        settings::INTERVAL,
        settings::DIGITS,
        &readout_card,
    )?;

    telegram::send(&client, &[&chart_card, &readout_card], &caption).await?;

    println!("sent to your channel.");

    Ok(())
}
