//! Draw any card without waiting for the market to do anything.
//!
//!     cargo run -p nsc-work-man --bin cards -- XAUUSD             approaching
//!     cargo run -p nsc-work-man --bin cards -- XAUUSD 4120        in the zone
//!     cargo run -p nsc-work-man --bin cards -- XAUUSD 4120 found  already in
//!     cargo run -p nsc-work-man --bin cards -- XAUUSD close       a close
//!     cargo run -p nsc-work-man --bin cards -- XAUUSD close 4375.6 sofar
//!     cargo run -p nsc-work-man --bin cards -- heartbeat          the quiet day
//!     cargo run -p nsc-work-man --bin cards -- armed             a level went live
//!     cargo run -p nsc-work-man --bin cards -- trouble down       the line is off
//!     cargo run -p nsc-work-man --bin cards -- trouble back       it is back
//!     cargo run -p nsc-work-man --bin cards -- trouble stopped    it gave up
//!
//! **This is the design loop, not the bot.** Changing how a card looks means
//! looking at it, and the market reaches a level when it feels like it.
//!
//! With no price it makes one up just outside the pair\'s first band — the
//! state hardest to draw, where price is close enough to the edge that the
//! labels crowd.

mod beat;
mod zone;

use std::path::Path;

use anyhow::{Context, Result};
use nsc_core::candle::{Bar, normal_candle};
use nsc_core::levels::{self, Band, Pair, Thickness, Timeframe};
use nsc_work_man::{feed, retry::keep_trying};

/// His own inbox. Nothing drawn here is a signal.
pub const OWNER: i64 = 6089491075;

const HISTORY: usize = 60;
pub const NORMAL_OVER: usize = 14;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let client = reqwest::Client::new();
    let wanted = std::env::args().nth(1).unwrap_or_else(|| "XAUUSD".into());

    if wanted == "heartbeat" {
        return beat::heartbeat(&client).await;
    }

    if wanted == "armed" {
        return beat::armed(&client).await;
    }

    if wanted == "trouble" {
        return beat::trouble(&client, std::env::args().nth(2)).await;
    }

    let file = Path::new("config/pairs").join(format!("{wanted}.toml"));
    let pair = levels::load_pair(&file)
        .with_context(|| format!("no levels for {wanted} — is there a {}?", file.display()))?;

    let thickness = levels::load_thickness(Path::new("config/levels.toml"))?;
    let (band, bars) = first_band(&client, &pair, thickness).await?;

    let asked = std::env::args().nth(2);

    if asked.as_deref() == Some("close") {
        return zone::draw_close(&client, &pair, &band, &bars, thickness).await;
    }

    zone::draw_alert(&client, &pair, &band, thickness, asked).await
}

/// The pair's first band, sized off real candles, plus hourly candles to draw.
async fn first_band(
    client: &reqwest::Client,
    pair: &Pair,
    thickness: Thickness,
) -> Result<(Band, Vec<Bar>)> {
    let line = pair.levels.first().context("that pair has no levels")?;

    let interval = match line.timeframe {
        Timeframe::Weekly => "1week",
        Timeframe::Daily => "1day",
        Timeframe::H4 => "4h",
    };

    let sizing = candles(client, &pair.symbol, interval).await?;
    let size = normal_candle(&sizing.iter().collect::<Vec<_>>(), NORMAL_OVER)
        .context("no candles came back")?;

    let band = pair
        .bands(thickness, &[(line.timeframe, size)])
        .into_iter()
        .next()
        .context("the level could not be turned into a band")?;

    let hourly = candles(client, &pair.symbol, "1h").await?;

    Ok((band, hourly))
}

/// Candles, oldest first — the direction a chart is read in.
pub async fn candles(client: &reqwest::Client, symbol: &str, interval: &str) -> Result<Vec<Bar>> {
    let series = keep_trying(3, || feed::for_pair(client, symbol, interval, HISTORY))
        .await
        .with_context(|| format!("could not get {interval} candles for {symbol}"))?;

    let mut bars = series.values;
    bars.reverse();

    Ok(bars)
}
