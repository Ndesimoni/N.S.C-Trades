//! Drawing a pair's levels so he can see where they landed.
//!
//! **This is the check that matters.** Everything in this project is measured
//! against his levels, so the first question is always whether the band we
//! build sits where the one he drew sits.
//!
//! Reading a price back at him only proves he can read his own typing. A
//! picture shows the *place*, which is the thing that actually goes wrong.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::candle::{Bar, Series, normal_candle};
use crate::card;
use crate::levels::{Pair, Thickness, Timeframe};

/// How many candles back. Enough weeks to hold levels drawn years apart.
const HISTORY: usize = 150;

/// How many candles a "normal" one is averaged over. Fourteen is the usual.
const NORMAL_OVER: usize = 14;

/// Draws every level a pair has, on its weekly chart.
///
/// Weekly because his levels are years apart and an hourly chart is five days
/// wide. A daily level shows as a thin line there — correct, and a reminder
/// that a daily level is really for looking at on a daily chart.
pub async fn picture_of(
    client: &reqwest::Client,
    pair: &Pair,
    thickness: Thickness,
    out: &Path,
) -> Result<PathBuf> {
    let weekly = candles(client, &pair.symbol, "1week").await?;
    let daily = candles(client, &pair.symbol, "1day").await?;

    let weekly_candle = normal_candle(&weekly.iter().collect::<Vec<_>>(), NORMAL_OVER)
        .context("no weekly candles came back")?;
    let daily_candle = normal_candle(&daily.iter().collect::<Vec<_>>(), NORMAL_OVER)
        .context("no daily candles came back")?;

    let bands = pair.bands(
        thickness,
        &[
            (Timeframe::Weekly, weekly_candle),
            (Timeframe::Daily, daily_candle),
            // A 4-hour candle is not fetched — sizing its band off the daily is
            // close enough to look at, and this picture is for looking at.
            (Timeframe::H4, daily_candle),
        ],
    );

    let drawn: Vec<&Bar> = weekly.iter().collect();

    card::render(
        "levels.html",
        &drawn,
        &bands,
        &pair.symbol,
        "1week",
        pair.digits,
        out,
    )
}

/// Candles, oldest first — the direction a chart is read in.
async fn candles(client: &reqwest::Client, symbol: &str, interval: &str) -> Result<Vec<Bar>> {
    let key = std::env::var("TWELVE_DATA_API_KEY").context("TWELVE_DATA_API_KEY is not set")?;

    let url = format!(
        "https://api.twelvedata.com/time_series\
         ?symbol={symbol}&interval={interval}&outputsize={HISTORY}&timezone=UTC&apikey={key}"
    );

    let body = client.get(&url).send().await?.text().await?;

    let series: Series = serde_json::from_str(&body)
        .with_context(|| format!("Twelve Data sent this instead of candles:\n{body}"))?;

    let mut bars = series.values;
    bars.reverse();

    Ok(bars)
}
