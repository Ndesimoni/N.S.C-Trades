//! The heartbeat card, drawn without waiting for a quiet morning.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use nsc_core::candle::normal_candle;
use nsc_core::levels::{self, Timeframe};
use nsc_work_man::card::Wrong;
use nsc_work_man::{card, telegram};

use nsc_data::source::Interval;
use nsc_data::sources::ibkr::IbkrConnection;

use nsc_work_man::places::{OWNER, PAIRS, PREVIEW, THICKNESS};

use super::{NORMAL_OVER, candles};

/// The heartbeat, so it can be looked at without waiting for a quiet day.
pub async fn heartbeat(client: &reqwest::Client, ibkr: &IbkrConnection) -> Result<()> {
    let names = levels::known(Path::new(PAIRS));
    let thickness = levels::load_thickness(Path::new(THICKNESS))?;

    // Sized off real candles, one request per pair. The bot itself does this
    // once at startup and never again; here it is the price of seeing the card
    // without waiting for a quiet morning.
    let mut loaded = Vec::new();
    for name in &names {
        let pair = levels::load_pair(&Path::new(PAIRS).join(format!("{name}.toml")))?;
        let weekly = candles(ibkr, &pair.symbol, Interval::Week).await?;
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

    let alive: Vec<card::Alive> = loaded
        .iter()
        .map(|(pair, bands, price)| card::Alive {
            pair: pair.clone(),
            bands: bands.clone(),
            price: *price,
        })
        .collect();

    let stamp = Utc::now().format("%-d %b · %H:%M UTC").to_string();
    let out = PathBuf::from(PREVIEW).join("heartbeat.png");

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
        "{} pairs, {zones} zones.\n\nDrawn to {} and sent.",
        alive.len(),
        out.display()
    );

    Ok(())
}

/// The three trouble cards, so they can be looked at without breaking
/// anything to see one.
pub async fn trouble(client: &reqwest::Client, which: Option<String>) -> Result<()> {
    let (wrong, minutes) = match which.as_deref() {
        Some("back") => (Wrong::LineBack, Some(12)),
        Some("stopped") => (Wrong::Stopped, None),
        _ => (Wrong::LineDown, Some(7)),
    };

    let detail = match wrong {
        Wrong::Stopped => {
            "cannot read the calendar at config/when.toml: No such file or directory (os error 2)"
        }
        _ => "the price line would not open: IO error: Connection refused (os error 61)",
    };

    let stamp = Utc::now().format("%-d %b · %H:%M UTC").to_string();
    let out = PathBuf::from(PREVIEW).join("trouble.png");

    let picture = card::trouble(wrong, minutes, detail, &stamp, &out)?;

    telegram::send_to(
        client,
        &OWNER.to_string(),
        &[&picture],
        "🎨 <b>Preview.</b> This is what a trouble card looks like — nothing is wrong.",
    )
    .await?;
    println!("{wrong:?} — drawn to {} and sent.", out.display());

    Ok(())
}

/// The line he gets when a level he just sent goes live.
pub async fn armed(client: &reqwest::Client, ibkr: &IbkrConnection) -> Result<()> {
    let thickness = levels::load_thickness(Path::new(THICKNESS))?;

    nsc_work_man::watch::say_it_is_armed(client, ibkr, thickness).await?;
    println!("Sent.");

    Ok(())
}
