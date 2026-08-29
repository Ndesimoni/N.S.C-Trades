//! Drawing the card and sending it.

use std::path::PathBuf;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use nsc_core::news::{Event, minutes_until};

use crate::places::{OWNER, PREVIEW};
use crate::retry::keep_trying;
use crate::{card, telegram};

/// Draws one release and sends it.
///
/// `group` is everything printing at the same moment — three Australian CPI
/// numbers share a second, and they share a card.
pub(super) async fn card(
    client: &reqwest::Client,
    group: &[&Event],
    now: DateTime<Utc>,
) -> Result<()> {
    let Some(first) = group.first() else {
        return Ok(());
    };

    let minutes = minutes_until(first, now);
    let when = first.at.format("%H:%M UTC, %-d %b").to_string();
    let stamp = now.format("%-d %b · %H:%M UTC").to_string();

    // **One file, reused, like every other card.**
    //
    // A name per release was the first attempt — it cannot collide, but it
    // leaves a picture and a page behind for every release forever, which is
    // about a hundred files a week nothing ever clears up.
    //
    // Reusing it is safe because `telegram::send_to` reads the whole picture
    // into memory before it opens the request, and the groups below are sent
    // one after another rather than at the same time. So the next draw cannot
    // overwrite a file still waiting to go.
    let out = PathBuf::from(PREVIEW).join("news.png");

    let picture = draw(group, minutes, when.clone(), stamp, out).await?;

    let words = words(group, minutes, &when);
    let owner = OWNER.to_string();
    let pictures = [picture.as_path()];

    keep_trying(3, || telegram::send_to(client, &owner, &pictures, &words))
        .await
        .context("could not send the news card")?;

    println!("News — {} in {minutes} min ({when}).", first.title);

    Ok(())
}

/// Runs Chrome **off the price loop**.
///
/// Drawing a card is a blocking wait of two to ten seconds. Left in the async
/// task it holds a Tokio worker for all of it — harmless on eight cores, and
/// on the one-core box this is meant to be hosted on it stops everything.
///
/// `spawn_blocking` has a pool for exactly this. It needs owned values, so the
/// events are cloned first: small structs, and nothing next to running Chrome.
async fn draw(
    group: &[&Event],
    minutes: i64,
    when: String,
    stamp: String,
    out: PathBuf,
) -> Result<PathBuf> {
    let owned: Vec<Event> = group.iter().map(|event| (*event).clone()).collect();

    tokio::task::spawn_blocking(move || {
        let borrowed: Vec<&Event> = owned.iter().collect();
        card::coming(&borrowed, minutes, &when, &stamp, &out)
    })
    .await
    .context("the news card was interrupted while drawing")?
    .context("could not draw the news card")
}

/// The caption under the picture.
///
/// **It has to stand on its own.** A card that fails to load, or a phone on a
/// bad line, leaves only this — so it says what, when, and how many rather
/// than "see above".
fn words(group: &[&Event], minutes: i64, when: &str) -> String {
    let Some(first) = group.first() else {
        return String::new();
    };

    // **Worded exactly as the card words it.** A caption saying one thing
    // over a picture saying another is the mistake that put *4,094.00* on a
    // card and *4094* underneath it — the same number, twice, reading as two.
    let how_soon = match minutes {
        m if m > 90 => {
            let hours = (m as f64 / 60.0).round() as i64;
            format!("in {hours} hour{}", if hours == 1 { "" } else { "s" })
        }
        m if m > 1 => format!("in {m} minutes"),
        1 => "in a minute".to_string(),
        0 => "right now".to_string(),
        _ => "just printed".to_string(),
    };

    let what = if group.len() == 1 {
        format!("{} {}", first.currency, first.title)
    } else {
        format!("{} — {} releases", first.currency, group.len())
    };

    format!("<b>{what}</b> {how_soon}\n{when}")
}
