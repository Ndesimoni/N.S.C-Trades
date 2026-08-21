//! Drawing a card and sending it.
//!
//! **An alert is not a signal.** No entry, no stop, no target — because there
//! is no trade. The cards say so on their own face, and if these two ever
//! start looking alike the price watcher has become a strategy nobody
//! reviewed.

use std::path::PathBuf;

use crate::retry::keep_trying;
use crate::{card, telegram};
use anyhow::{Context, Result};
use chrono::Utc;
use nsc_core::candle::Bar;
use nsc_core::levels::{self, Action, AtZone, Band, Nearness, News, Pair};
use rust_decimal::Decimal;

use crate::places::{OWNER, PREVIEW};

/// Rung 1 — price has reached one of his zones.
pub async fn alert(
    client: &reqwest::Client,
    pair: &Pair,
    band: &Band,
    near: Nearness,
    news: News,
    price: Decimal,
    reach: Decimal,
) -> Result<()> {
    let stamp = Utc::now().format("%-d %b · %H:%M UTC").to_string();
    let out = card_for(pair, "alert");

    let picture = card::alert(pair, band, near, news, price, reach, &stamp, &out)
        .with_context(|| format!("could not draw the alert for {}", pair.symbol))?;

    let caption = levels::caption(pair, band, near, news, price);

    send(client, &picture, &caption)
        .await
        .context("could not send the alert")
}

/// Rung 2 — a candle that touched one of his zones.
///
/// `forming` means it has NOT finished — the twenty-minute look, which is the
/// one place in this project that reads a candle early.
#[allow(clippy::too_many_arguments)]
pub async fn closed(
    client: &reqwest::Client,
    pair: &Pair,
    band: &Band,
    bar: &Bar,
    did: AtZone,
    was: Action,
    interval: &str,
    forming: bool,
) -> Result<()> {
    let what = if forming { "sofar" } else { "close" };
    let out = card_for(pair, &format!("{what}-{interval}"));

    let picture = card::closed(pair, band, bar, did, was, interval, forming, &out)
        .with_context(|| format!("could not draw the close for {}", pair.symbol))?;

    let caption = levels::closed_caption(pair, band, bar, did, was, interval, forming);

    send(client, &picture, &caption)
        .await
        .context("could not send the close")
}

/// Sends, and **tries again if the trouble says it is worth it**.
///
/// It went straight out with no retry, and that lost things. `Watch::arrive`
/// has already marked the band as reached by the time the card goes, so a
/// dropped connection to Telegram meant the alert was never sent AND never
/// tried again — it would not fire until price left the zone and came back.
///
/// `SendError` has known retry-or-give-up since the day it was written. It was
/// simply never asked. A wrong token still stops on the first go rather than
/// three.
async fn send(client: &reqwest::Client, picture: &std::path::Path, caption: &str) -> Result<()> {
    // Built once, outside the closure — the closure runs up to three times,
    // and both of these would otherwise be borrowed from a value that dies at
    // the end of the first go.
    let owner = OWNER.to_string();
    let pictures = [picture];

    keep_trying(3, || telegram::send_to(client, &owner, &pictures, caption)).await?;

    Ok(())
}

/// Each pair gets its own file, so two cards drawn seconds apart cannot
/// overwrite each other's picture between drawing and sending.
fn card_for(pair: &Pair, what: &str) -> PathBuf {
    PathBuf::from(PREVIEW).join(format!("{what}-{}.png", pair.symbol.replace('/', "")))
}
