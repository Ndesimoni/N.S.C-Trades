//! The two zone cards — price at a level, and a candle at one.

use std::path::PathBuf;

use anyhow::{Context, Result};
use chrono::Utc;
use nsc_core::candle::Bar;
use nsc_core::levels::{
    self, Band, Nearness, News, Pair, Thickness, action, nearness, what_it_did,
};
use nsc_work_man::{card, telegram};
use rust_decimal::Decimal;

use nsc_work_man::places::{OWNER, PREVIEW};

/// Rung 1 — price at the zone.
pub async fn draw_alert(
    client: &reqwest::Client,
    pair: &Pair,
    band: &Band,
    thickness: Thickness,
    asked: Option<String>,
) -> Result<()> {
    let share = pair.reach_share(thickness);
    let reach = band.thickness() * share;

    // No price given: sit just outside the band, where the labels crowd.
    let price = match asked {
        Some(text) => text.parse().context("that price is not a number")?,
        None => band.top + reach / Decimal::TWO,
    };

    let news = match std::env::args().nth(3).as_deref() {
        Some("found") => News::Already,
        _ => News::Fresh,
    };

    let near = nearness(band, price, reach);
    if near == Nearness::Away {
        println!("Note — {price} is outside the zone and its reach. Drawing it anyway.");
    }

    let stamp = Utc::now().format("%-d %b · %H:%M UTC").to_string();
    let out = PathBuf::from(PREVIEW).join("alert.png");

    let picture = card::alert(pair, band, near, news, price, reach, &stamp, &out)?;
    let caption = levels::caption(pair, band, near, news, price);

    telegram::send_to(client, &OWNER.to_string(), &[&picture], &caption).await?;

    println!(
        "{} band {} to {}",
        band.timeframe.name(),
        band.bottom,
        band.top
    );
    println!("Price {price} · fires {reach} out · {near:?} · {news:?}");
    println!("\nDrawn to {} and sent to you.", out.display());

    Ok(())
}

/// Rung 2 — the newest finished hourly candle, against that band.
pub async fn draw_close(
    client: &reqwest::Client,
    pair: &Pair,
    band: &Band,
    bars: &[Bar],
    thickness: Thickness,
) -> Result<()> {
    let now = Utc::now();

    let bar = bars
        .iter()
        .rev()
        .find(|bar| bar.finished_by(now, 60).unwrap_or(false))
        .or_else(|| bars.last())
        .context("no candles came back")?;

    // His real zone may be thousands of points from where price is today, and
    // a card of a candle that MISSED shows nothing about the design. So the
    // preview may be given a level to sit the band on.
    let band = match std::env::args().nth(3) {
        Some(text) => {
            let at: Decimal = text.parse().context("that level is not a number")?;
            let moved = Band::around(band.timeframe, at, band.thickness(), Decimal::ONE);
            println!(
                "Using a made-up {} level at {at}, so the candle meets it.",
                moved.timeframe.name()
            );
            moved
        }
        None => *band,
    };

    let did = what_it_did(&band, bar);
    let was = action(&band, bar, thickness.kiss_depth);
    let out = PathBuf::from(PREVIEW).join("close.png");

    let picture = card::closed(pair, &band, bar, did, was, "1h", &out)?;
    let caption = levels::closed_caption(pair, &band, bar, did, was, "1h");

    telegram::send_to(client, &OWNER.to_string(), &[&picture], &caption).await?;

    println!(
        "{} candle {} — {was:?} ({did:?})",
        pair.symbol, bar.datetime
    );
    println!("\nDrawn to {} and sent to you.", out.display());

    Ok(())
}
