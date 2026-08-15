//! The heartbeat card, drawn without waiting for a quiet morning.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use nsc_core::candle::normal_candle;
use nsc_core::levels::{self, Timeframe};
use nsc_work_man::{card, telegram};

use super::{NORMAL_OVER, OWNER, candles};

/// The heartbeat, so it can be looked at without waiting for a quiet day.
pub async fn heartbeat(client: &reqwest::Client) -> Result<()> {
    let names = levels::known(Path::new("config/pairs"));
    let thickness = levels::load_thickness(Path::new("config/levels.toml"))?;

    // Sized off real candles, one request per pair. The bot itself does this
    // once at startup and never again; here it is the price of seeing the card
    // without waiting for a quiet morning.
    let mut loaded = Vec::new();
    for name in &names {
        let pair = levels::load_pair(&Path::new("config/pairs").join(format!("{name}.toml")))?;
        let weekly = candles(client, &pair.symbol, "1week").await?;
        let size = normal_candle(&weekly.iter().collect::<Vec<_>>(), NORMAL_OVER)
            .context("no candles came back")?;

        let bands = pair.bands(
            thickness,
            &[
                (Timeframe::Weekly, size),
                (Timeframe::Daily, size),
                (Timeframe::H4, size),
            ],
        );

        let price = weekly.last().map(|bar| bar.close);
        loaded.push((pair, bands, price));
    }

    let alive: Vec<card::Alive<'_>> = loaded
        .iter()
        .map(|(pair, bands, price)| card::Alive {
            pair,
            bands: bands.clone(),
            price: *price,
        })
        .collect();

    let stamp = Utc::now().format("%-d %b · %H:%M UTC").to_string();
    let out = PathBuf::from("preview").join("heartbeat.png");

    let picture = card::heartbeat(&alive, "10 hours", &stamp, &out)?;
    let zones: usize = alive.iter().map(|a| a.bands.len()).sum();

    telegram::send_to(
        client,
        &OWNER.to_string(),
        &[&picture],
        &nsc_core::when::beat_words(alive.len(), zones),
    )
    .await?;

    println!(
        "{} pairs, {zones} zones\n\ndrawn to {} and sent.",
        alive.len(),
        out.display()
    );

    Ok(())
}
