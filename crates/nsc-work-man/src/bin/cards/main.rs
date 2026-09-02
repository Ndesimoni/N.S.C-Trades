//! Draw any card without waiting for the market to do anything.
//!
//! ```text
//!     cargo run -p nsc-work-man --bin cards -- XAUUSD             approaching
//!     cargo run -p nsc-work-man --bin cards -- XAUUSD 4120        in the zone
//!     cargo run -p nsc-work-man --bin cards -- XAUUSD 4120 found  already in
//!     cargo run -p nsc-work-man --bin cards -- XAUUSD close       a close
//!     cargo run -p nsc-work-man --bin cards -- news               what is coming up
//!     cargo run -p nsc-work-man --bin cards -- news busy          several at once
//!     cargo run -p nsc-work-man --bin cards -- news today         the day's list
//!     cargo run -p nsc-work-man --bin cards -- news week          the week's list
//!     cargo run -p nsc-work-man --bin cards -- setup              a rung 3 signal
//!     cargo run -p nsc-work-man --bin cards -- bundle       all three, one signal
//!     cargo run -p nsc-work-man --bin cards -- charts   both charts, EVERY pair
//!     cargo run -p nsc-work-man --bin cards -- heartbeat          the quiet day
//!     cargo run -p nsc-work-man --bin cards -- armed             a level went live
//!     cargo run -p nsc-work-man --bin cards -- trouble down       the line is off
//!     cargo run -p nsc-work-man --bin cards -- trouble back       it is back
//!     cargo run -p nsc-work-man --bin cards -- trouble stopped    it gave up
//! ```
//!
//! **This is the design loop, not the bot.** Changing how a card looks means
//! looking at it, and the market reaches a level when it feels like it.
//!
//! With no price it makes one up just outside the pair\'s first band — the
//! state hardest to draw, where price is close enough to the edge that the
//! labels crowd.

mod asking;
mod beat;
mod bundle;
mod charts;
mod found;
mod soon;
mod zone;

use std::path::Path;

use anyhow::{Context, Result};
use nsc_core::candle::{Bar, normal_candle};
use nsc_core::levels::{self, Band, Pair, Thickness, Timeframe};
use nsc_data::source::{Interval, MarketDataSource};
use nsc_data::sources::ibkr::IbkrConnection;
use nsc_work_man::places::{PAIRS, THICKNESS};
use nsc_work_man::retry::keep_trying;

const HISTORY: usize = 60;
pub const NORMAL_OVER: usize = 14;

#[tokio::main]
async fn main() -> Result<()> {
    nsc_work_man::secrets::load();

    let client = nsc_work_man::web::client();
    let wanted = std::env::args().nth(1).unwrap_or_else(|| "XAUUSD".into());

    // The trouble cards are the only ones that need no candles at all, so they
    // are answered before TWS is asked for anything.
    if wanted == "trouble" {
        return beat::trouble(&client, std::env::args().nth(2)).await;
    }

    // The news card needs no candles either — the economic calendar is a plain
    // web page. So it is answered before TWS is asked for anything, which
    // means the design can be looked at with the gateway shut.
    // The bank consensus needs no candles either — a plain web endpoint with
    // no key on it. Answered before TWS is asked for anything.
    // Rung 3, drawn on the two gold candles off his own screenshot. No TWS.
    if wanted == "bundle" {
        return bundle::bundle(&client).await;
    }

    if wanted == "setup" {
        return found::setup(&client, std::env::args().nth(2).as_deref()).await;
    }

    if wanted == "news" {
        return match std::env::args().nth(2).as_deref() {
            Some("today") => soon::calendar(&client, nsc_core::news::Span::Today).await,
            Some("week") => soon::calendar(&client, nsc_core::news::Span::Week).await,
            Some("busy") => soon::news(&client, true).await,
            _ => soon::news(&client, false).await,
        };
    }

    // TWS or IB Gateway has to be running. Every card but the trouble one is
    // drawn on real candles — a made-up one would look better than the real
    // thing ever does, which is the opposite of what a preview is for.
    let ibkr = IbkrConnection::connect().await?;

    if wanted == "charts" {
        return charts::every_pair(&client, &ibkr).await;
    }

    if wanted == "heartbeat" {
        return beat::heartbeat(&client, &ibkr).await;
    }

    if wanted == "armed" {
        return beat::armed(&client, &ibkr).await;
    }

    let file = Path::new(PAIRS).join(format!("{wanted}.toml"));
    let pair = levels::load_pair(&file)
        .with_context(|| format!("no levels for {wanted} — is there a {}?", file.display()))?;

    let thickness = levels::load_thickness(Path::new(THICKNESS))?;
    let (band, bars) = first_band(&ibkr, &pair, thickness).await?;

    let asked = std::env::args().nth(2);

    if asked.as_deref() == Some("close") {
        return zone::draw_close(&client, &pair, &band, &bars, thickness).await;
    }

    zone::draw_alert(&client, &pair, &band, thickness, asked).await
}

/// The pair's first band, sized off real candles, plus hourly candles to draw.
async fn first_band(
    ibkr: &IbkrConnection,
    pair: &Pair,
    thickness: Thickness,
) -> Result<(Band, Vec<Bar>)> {
    let line = pair.levels.first().context("that pair has no levels")?;

    let interval = match line.timeframe {
        Timeframe::Weekly => Interval::Week,
        Timeframe::Daily => Interval::Day,
        Timeframe::H4 => Interval::H4,
    };

    let sizing = candles(ibkr, &pair.symbol, interval).await?;
    let size = normal_candle(&sizing.iter().collect::<Vec<_>>(), NORMAL_OVER)
        .context("no candles came back")?;

    let band = pair
        .bands(thickness, &[(line.timeframe, size)])
        .into_iter()
        .next()
        .context("the level could not be turned into a band")?;

    let hourly = candles(ibkr, &pair.symbol, Interval::H1).await?;

    Ok((band, hourly))
}

/// Candles, oldest first — the direction a chart is read in.
pub async fn candles(ibkr: &IbkrConnection, symbol: &str, interval: Interval) -> Result<Vec<Bar>> {
    let mut bars = keep_trying(3, || ibkr.candles(symbol, interval, HISTORY))
        .await
        .with_context(|| format!("could not get {} candles for {symbol}", interval.spoken()))?;

    bars.reverse();

    Ok(bars)
}
