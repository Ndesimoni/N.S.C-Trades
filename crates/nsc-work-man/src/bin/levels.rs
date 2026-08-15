//! Draw his levels on a chart and send it, so he can hold it up against his own.
//!
//! **This is the test that matters.** Everything else in this project is
//! measured against his levels, so the first question is whether the bands we
//! build land where the ones he drew land.
//!
//! It draws on the weekly, because his levels are years apart and an hourly
//! chart is five days wide.

use std::path::Path;

use anyhow::{Context, Result, bail};
use nsc_work_man::candle::{Bar, normal_candle};
use nsc_work_man::levels::{Timeframe, load_pair, load_thickness};
use nsc_work_man::{card, telegram};

/// How many candles back. Enough weeks to see levels drawn years apart.
const HISTORY: usize = 150;

/// How many candles a "normal" one is averaged over. Fourteen is the usual.
const NORMAL_OVER: usize = 14;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let client = reqwest::Client::new();
    let pair = load_pair(Path::new("config/pairs/XAUUSD.toml"))?;
    let thickness = load_thickness(Path::new("config/levels.toml"))?;

    // Weekly for the chart itself and for how big a weekly candle is; daily
    // only to size the daily bands.
    let weekly = fetch(&client, &pair.symbol, "1week").await?;
    let daily = fetch(&client, &pair.symbol, "1day").await?;

    let weekly_candle = normal_candle(&weekly.iter().collect::<Vec<_>>(), NORMAL_OVER)
        .context("no weekly candles")?;
    let daily_candle = normal_candle(&daily.iter().collect::<Vec<_>>(), NORMAL_OVER)
        .context("no daily candles")?;

    println!("a normal weekly candle is {weekly_candle:.2}, a daily one {daily_candle:.2}\n");

    let bands = pair.bands(
        thickness,
        &[
            (Timeframe::Weekly, weekly_candle),
            (Timeframe::Daily, daily_candle),
            (Timeframe::H4, daily_candle),
        ],
    );

    if bands.is_empty() {
        bail!("no levels in config/pairs — nothing to draw");
    }

    for band in &bands {
        println!(
            "{:7} {:>10}   band {:.2} to {:.2}   ({:.2} thick)",
            band.timeframe.name(),
            band.price.to_string(),
            band.bottom,
            band.top,
            band.thickness()
        );
    }

    let drawn: Vec<&Bar> = weekly.iter().collect();
    let picture = Path::new("preview").join("levels.png");
    card::render(
        "levels.html",
        &drawn,
        &bands,
        "1week",
        pair.digits,
        &picture,
    )?;

    let caption = format!(
        "<b>{}</b> · weekly · {} level{} you drew",
        pair.symbol,
        bands.len(),
        if bands.len() == 1 { "" } else { "s" }
    );

    telegram::send(&client, &[&picture], &caption).await?;
    println!("\nsent to your channel.");

    Ok(())
}

/// Candles, oldest first.
async fn fetch(client: &reqwest::Client, symbol: &str, interval: &str) -> Result<Vec<Bar>> {
    let key = std::env::var("TWELVE_DATA_API_KEY").context("TWELVE_DATA_API_KEY is not set")?;

    let url = format!(
        "https://api.twelvedata.com/time_series\
         ?symbol={symbol}&interval={interval}&outputsize={HISTORY}&timezone=UTC&apikey={key}"
    );

    let body = client.get(&url).send().await?.text().await?;

    let series: nsc_work_man::candle::Series = serde_json::from_str(&body)
        .with_context(|| format!("Twelve Data sent this instead of candles:\n{body}"))?;

    // They come newest first. A chart reads the other way.
    let mut bars = series.values;
    bars.reverse();

    Ok(bars)
}
