//! Draw an alert card without waiting for price to reach a level.
//!
//!     cargo run -p nsc-work-man --bin alert -- XAUUSD 4132.90
//!
//! **This is the design loop, not the bot.** Changing how the card looks means
//! looking at it, and the market reaches a level when it feels like it. Give
//! it a pair and a price and it draws the alert that price would have caused.
//!
//! With no price it makes one up just outside the pair's first band, which is
//! the state hardest to get right — price close enough to the edge that the
//! labels crowd.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use nsc_core::levels::{Band, Nearness, Pair, Thickness, Timeframe, nearness};
use nsc_core::{candle::normal_candle, levels};
use nsc_work_man::{card, feed, retry::keep_trying, telegram};
use rust_decimal::Decimal;

const OWNER: i64 = 6089491075;
const HISTORY: usize = 60;
const NORMAL_OVER: usize = 14;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let client = reqwest::Client::new();

    let wanted = std::env::args().nth(1).unwrap_or_else(|| "XAUUSD".into());
    let file = Path::new("config/pairs").join(format!("{wanted}.toml"));

    let pair = load(&file, &wanted)?;
    let thickness = levels::load_thickness(Path::new("config/levels.toml"))?;

    let band = first_band(&client, &pair, thickness).await?;
    let reach = pair.reach(thickness);

    // No price given: sit just outside the band, where the labels crowd.
    let price = match std::env::args().nth(2) {
        Some(text) => text.parse().context("that price is not a number")?,
        None => band.top + reach / Decimal::TWO,
    };

    let near = nearness(&band, price, reach);
    if near == Nearness::Away {
        println!(
            "note: {price} is nowhere near {} to {} — the card will say approaching anyway",
            band.bottom, band.top
        );
    }

    let stamp = Utc::now().format("%-d %b · %H:%M UTC").to_string();
    let out = PathBuf::from("preview").join("alert.png");

    let picture = card::alert(&pair, &band, near, price, reach, &stamp, &out)?;
    let caption = levels::caption(&pair, &band, near, price);

    telegram::send_to(&client, &OWNER.to_string(), &[&picture], &caption).await?;

    println!(
        "{} band {} to {}",
        band.timeframe.name(),
        band.bottom,
        band.top
    );
    println!("price {price} · fires {reach} out · {near:?}");
    println!("\ndrawn to {} and sent to you.", out.display());

    Ok(())
}

fn load(file: &Path, wanted: &str) -> Result<Pair> {
    levels::load_pair(file)
        .with_context(|| format!("no levels for {wanted} — is there a {}?", file.display()))
}

/// The pair's first band, sized off real candles.
async fn first_band(client: &reqwest::Client, pair: &Pair, thickness: Thickness) -> Result<Band> {
    let line = pair.levels.first().context("that pair has no levels")?;

    let interval = match line.timeframe {
        Timeframe::Weekly => "1week",
        Timeframe::Daily => "1day",
        Timeframe::H4 => "4h",
    };

    let series = keep_trying(3, || {
        feed::for_pair(client, &pair.symbol, interval, HISTORY)
    })
    .await
    .with_context(|| format!("could not size the {interval} band for {}", pair.symbol))?;

    let mut bars: Vec<_> = series.values.iter().collect();
    bars.reverse();

    let size = normal_candle(&bars, NORMAL_OVER).context("no candles came back")?;

    pair.bands(thickness, &[(line.timeframe, size)])
        .into_iter()
        .next()
        .context("the level could not be turned into a band")
}
