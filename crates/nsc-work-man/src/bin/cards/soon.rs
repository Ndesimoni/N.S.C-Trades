//! The news card, drawn against the real calendar.
//!
//! **It needs no TWS.** The economic calendar is a plain web page with no key
//! on it, so this is the one card that can be looked at with the gateway shut.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use nsc_core::news::{self, Event, Span, minutes_until, together, within};
use nsc_data::news::fetch;
use nsc_work_man::places::{NEWS, OWNER, PREVIEW};
use nsc_work_man::{card, telegram};

/// Draws the next release on the real calendar.
///
/// **Real events, not invented ones.** A made-up card looks better than the
/// real thing ever does, which is the opposite of what a preview is for — the
/// titles the calendar actually uses are long and they are what has to fit.
pub async fn news(client: &reqwest::Client, busiest: bool) -> Result<()> {
    let rules = news::load(Path::new(NEWS)).with_context(|| format!("could not read {NEWS}"))?;

    let parsed = fetch(client, &rules.url)
        .await
        .context("could not download the economic calendar")?;

    println!(
        "{} events this week, {} unreadable.",
        parsed.events.len(),
        parsed.unreadable
    );

    let now = Utc::now();

    // The next release he would actually be told about — everything he asked
    // for, still ahead of us, soonest first.
    let mut ahead: Vec<&Event> = parsed
        .events
        .iter()
        .filter(|event| rules.wants(event.impact) && event.at > now)
        .collect();

    ahead.sort_by_key(|event| event.at);

    // Copied out, not borrowed — the group below reads the same list.
    let Some(next) = ahead.first().copied() else {
        println!("Nothing left on the calendar this week that you asked for.");
        return Ok(());
    };

    // Everything printing at the same second shares the card, exactly as the
    // watcher would group it.
    let grouped = together(&ahead);

    // **The busiest group is the one worth looking at.** One release is the
    // easy case; three Australian CPI numbers in the same second are what the
    // layout has to survive, and they are the reason grouping exists at all.
    let group: Vec<&Event> = if busiest {
        grouped
            .into_iter()
            .max_by_key(Vec::len)
            .unwrap_or_else(|| vec![next])
    } else {
        grouped.into_iter().next().unwrap_or_else(|| vec![next])
    };

    let Some(next) = group.first().copied() else {
        return Ok(());
    };

    let at = next.at;

    let minutes = minutes_until(next, now);
    let when = at.format("%H:%M UTC, %-d %b").to_string();
    let stamp = now.format("%-d %b · %H:%M UTC").to_string();
    let out = PathBuf::from(PREVIEW).join("news.png");

    let picture = card::coming(&group, minutes, &when, &stamp, &out)?;

    // **Sent, like every other preview here.** Seeing a card on a phone is not
    // the same as seeing it on a Mac — the type is smaller, the crop is
    // different, and that is where he actually reads it.
    //
    // Marked as a preview on its face, because a news card arriving out of the
    // blue is indistinguishable from the real thing, and this one is showing a
    // release that may be days away.
    telegram::send_to(
        client,
        &OWNER.to_string(),
        &[&picture],
        &format!(
            "🎨 <b>Preview.</b> This is what a news card looks like — \
             nothing is due right now.\n{} · {when}",
            next.currency
        ),
    )
    .await?;

    println!(
        "\n{} · {} release{} at {when}, {minutes} min away\nDrawn to {} and sent.",
        next.currency,
        group.len(),
        if group.len() == 1 { "" } else { "s" },
        picture.display()
    );

    Ok(())
}

/// The list he gets from `/news` — today, or the rest of the week.
///
/// Real events, and the whole point of looking is how tall it gets: a week is
/// eighteen releases across six days, and today is three or four.
pub async fn calendar(client: &reqwest::Client, span: Span) -> Result<()> {
    let rules = news::load(Path::new(NEWS)).with_context(|| format!("could not read {NEWS}"))?;

    let parsed = fetch(client, &rules.url)
        .await
        .context("could not download the economic calendar")?;

    let now = Utc::now();
    let wanted = within(&parsed.events, now, span, &rules);

    println!(
        "{} events this week · {} clear the bar for {}",
        parsed.events.len(),
        wanted.len(),
        if span == Span::Today {
            "today"
        } else {
            "the week"
        },
    );

    if wanted.is_empty() {
        println!("Nothing to draw — the bot would answer in words, not a card.");
        return Ok(());
    }

    let out = PathBuf::from(PREVIEW).join("calendar.png");
    let picture = card::calendar(&wanted, span, now, &out)?;

    telegram::send_to(
        client,
        &OWNER.to_string(),
        &[&picture],
        &format!(
            "🎨 <b>Preview.</b> This is the list /news gives — \
             {} release{}.",
            wanted.len(),
            if wanted.len() == 1 { "" } else { "s" }
        ),
    )
    .await?;

    println!("Drawn to {} and sent.", picture.display());

    Ok(())
}
