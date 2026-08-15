//! Draw a card without waiting for the market to do anything.
//!
//!     cargo run -p nsc-work-man --bin alert -- XAUUSD            approaching
//!     cargo run -p nsc-work-man --bin alert -- XAUUSD 4120       in the zone
//!     cargo run -p nsc-work-man --bin alert -- XAUUSD 4120 found already in
//!     cargo run -p nsc-work-man --bin alert -- XAUUSD close      rung 2
//!
//! **This is the design loop, not the bot.** Changing how a card looks means
//! looking at it, and the market reaches a level when it feels like it.
//!
//! With no price it makes one up just outside the pair's first band — the
//! state hardest to draw, where price is close enough to the edge that the
//! labels crowd.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use nsc_core::candle::{Bar, normal_candle};
use nsc_core::levels::{
    self, Band, Nearness, News, Pair, Thickness, Timeframe, action, nearness, what_it_did,
};
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

    if wanted == "heartbeat" {
        return heartbeat(&client).await;
    }

    let file = Path::new("config/pairs").join(format!("{wanted}.toml"));

    let pair = levels::load_pair(&file)
        .with_context(|| format!("no levels for {wanted} — is there a {}?", file.display()))?;

    let thickness = levels::load_thickness(Path::new("config/levels.toml"))?;
    let (band, bars) = first_band(&client, &pair, thickness).await?;

    let asked = std::env::args().nth(2);

    if asked.as_deref() == Some("close") {
        return draw_close(&client, &pair, &band, &bars, thickness).await;
    }

    draw_alert(&client, &pair, &band, thickness, asked).await
}

/// The heartbeat, so it can be looked at without waiting for a quiet day.
async fn heartbeat(client: &reqwest::Client) -> Result<()> {
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

/// Rung 1 — price at the zone.
async fn draw_alert(
    client: &reqwest::Client,
    pair: &Pair,
    band: &Band,
    thickness: Thickness,
    asked: Option<String>,
) -> Result<()> {
    let reach = pair.reach(thickness);

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
        println!("note: {price} is outside the zone and its reach — drawing it anyway");
    }

    let stamp = Utc::now().format("%-d %b · %H:%M UTC").to_string();
    let out = PathBuf::from("preview").join("alert.png");

    let picture = card::alert(pair, band, near, news, price, reach, &stamp, &out)?;
    let caption = levels::caption(pair, band, near, news, price);

    telegram::send_to(client, &OWNER.to_string(), &[&picture], &caption).await?;

    println!(
        "{} band {} to {}",
        band.timeframe.name(),
        band.bottom,
        band.top
    );
    println!("price {price} · fires {reach} out · {near:?} · {news:?}");
    println!("\ndrawn to {} and sent to you.", out.display());

    Ok(())
}

/// Rung 2 — the newest finished hourly candle, against that band.
async fn draw_close(
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
                "using a made-up {} level at {at} so the candle meets it",
                moved.timeframe.name()
            );
            moved
        }
        None => *band,
    };

    let did = what_it_did(&band, bar);
    let was = action(&band, bar, thickness.kiss_depth);
    let out = PathBuf::from("preview").join("close.png");

    let picture = card::closed(pair, &band, bar, did, was, "1h", &out)?;
    let caption = levels::closed_caption(pair, &band, bar, did, was, "1h");

    telegram::send_to(client, &OWNER.to_string(), &[&picture], &caption).await?;

    println!(
        "{} candle {} — {was:?} ({did:?})",
        pair.symbol, bar.datetime
    );
    println!("\ndrawn to {} and sent to you.", out.display());

    Ok(())
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
async fn candles(client: &reqwest::Client, symbol: &str, interval: &str) -> Result<Vec<Bar>> {
    let series = keep_trying(3, || feed::for_pair(client, symbol, interval, HISTORY))
        .await
        .with_context(|| format!("could not get {interval} candles for {symbol}"))?;

    let mut bars = series.values;
    bars.reverse();

    Ok(bars)
}
