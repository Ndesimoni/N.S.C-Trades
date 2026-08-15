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
    // Which pair, from the command line. `cargo run --bin levels -- GBPUSD`.
    let wanted = std::env::args().nth(1).unwrap_or_else(|| "XAUUSD".into());
    let file = Path::new("config/pairs").join(format!("{wanted}.toml"));

    let pair = load_pair(&file)
        .with_context(|| format!("no levels for {wanted} — is there a {}?", file.display()))?;
    let thickness = load_thickness(Path::new("config/levels.toml"))?;

    // Weekly for the chart itself and for how big a weekly candle is; daily
    // only to size the daily bands.
    let weekly = fetch(&client, &pair.symbol, "1week").await?;
    let daily = fetch(&client, &pair.symbol, "1day").await?;

    let weekly_candle = normal_candle(&weekly.iter().collect::<Vec<_>>(), NORMAL_OVER)
        .context("no weekly candles")?;
    let daily_candle = normal_candle(&daily.iter().collect::<Vec<_>>(), NORMAL_OVER)
        .context("no daily candles")?;

    // Rounded to THE PAIR'S OWN precision. Two decimals says everything about
    // gold and nothing at all about the pound.
    let show = |value: rust_decimal::Decimal| value.round_dp(pair.digits).to_string();

    println!(
        "a normal weekly candle is {}, a daily one {}\n",
        show(weekly_candle),
        show(daily_candle)
    );

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
            "{:7} {:>10}   band {} to {}   ({} thick)",
            band.timeframe.name(),
            show(band.price),
            show(band.bottom),
            show(band.top),
            show(band.thickness())
        );
    }

    let drawn: Vec<&Bar> = weekly.iter().collect();
    let picture = Path::new("preview").join("levels.png");
    card::render(
        "levels.html",
        &drawn,
        &bands,
        &pair.symbol,
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
