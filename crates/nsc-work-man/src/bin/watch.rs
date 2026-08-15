//! Watch price against the levels he drew, and say when it reaches one.
//!
//! **Rung 1 of the ladder.** Price touches a level — an alert, one line, no
//! picture. Nothing has formed yet; it only means his zone is live.
//!
//! Silence is the normal state. Prices arrive about once a second and almost
//! all of them are nowhere near anything.
//!
//! It costs **no requests at all** to run. The candles that size the bands are
//! fetched once at the start; after that the prices come down the socket for
//! free. That is what killed the design where a candle was fetched every close
//! on every pair whether anything had happened or not.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use nsc_core::candle::normal_candle;
use nsc_core::levels::{
    self, Band, Nearness, Pair, Thickness, Timeframe, Watch, known, load_pair, load_thickness,
};
use nsc_work_man::{card, feed, retry::keep_trying, telegram};
use rust_decimal::Decimal;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

const PAIRS: &str = "config/pairs";
const THICKNESS: &str = "config/levels.toml";

/// Where the drawn card and its page are left, so the design can be opened in
/// a browser and changed without running any of this again.
const PREVIEW: &str = "preview";

/// How many candles a "normal" one is averaged over.
const NORMAL_OVER: usize = 14;

/// How many to fetch to work that out.
const HISTORY: usize = 60;

/// How long to wait between requests at startup.
///
/// **The limit is 8 a minute**, and sizing the bands needs one request per
/// pair per timeframe — four pairs across three timeframes is twelve, all
/// wanted at once.
///
/// Seven and a half seconds keeps it to eight a minute exactly. The whole
/// startup then takes a minute and a half, which nobody is waiting on.
///
/// The same arithmetic bites hardest on a Friday at 21:00 UTC, when the hour,
/// the 4-hour, the day and the week all end on the same second.
const BREATHE: std::time::Duration = std::time::Duration::from_millis(7_500);

/// Where his own working goes. Alerts are not signals.
const OWNER: i64 = 6089491075;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let client = reqwest::Client::new();
    let thickness = load_thickness(Path::new(THICKNESS))?;

    let mut watching: HashMap<String, (Pair, Watch)> = HashMap::new();

    // The bands are worked out once, at the start. After this, nothing is
    // fetched — every price arrives on the socket for nothing.
    for name in known(Path::new(PAIRS)) {
        let pair = load_pair(&Path::new(PAIRS).join(format!("{name}.toml")))?;
        let bands = bands_for(&client, &pair, thickness).await?;

        if bands.is_empty() {
            println!("{name} — no levels, skipping");
            continue;
        }

        println!("{} — watching {} level(s)", pair.symbol, bands.len());
        let watch = Watch::over(bands, pair.reach(thickness));
        watching.insert(pair.symbol.clone(), (pair, watch));
    }

    if watching.is_empty() {
        anyhow::bail!("no pairs have levels — send some with the inbox program");
    }

    listen(&client, &mut watching, thickness).await
}

/// Every level of a pair, as a band.
async fn bands_for(
    client: &reqwest::Client,
    pair: &Pair,
    thickness: Thickness,
) -> Result<Vec<Band>> {
    let mut sizes = Vec::new();

    for (timeframe, interval) in [
        (Timeframe::Weekly, "1week"),
        (Timeframe::Daily, "1day"),
        (Timeframe::H4, "4h"),
    ] {
        // Only ask about a timeframe he has actually drawn on. Most pairs
        // have weekly levels and nothing else, and a request for a timeframe
        // with no levels on it is a request spent on nothing.
        if !pair.levels.iter().any(|line| line.timeframe == timeframe) {
            continue;
        }

        let series = keep_trying(3, || {
            feed::for_pair(client, &pair.symbol, interval, HISTORY)
        })
        .await
        .with_context(|| format!("could not size {} bands for {}", interval, pair.symbol))?;

        tokio::time::sleep(BREATHE).await;

        let mut bars: Vec<_> = series.values.iter().collect();
        bars.reverse();

        if let Some(size) = normal_candle(&bars, NORMAL_OVER) {
            sizes.push((timeframe, size));
        }
    }

    Ok(pair.bands(thickness, &sizes))
}

/// Holds the line open and watches every price that comes down it.
async fn listen(
    client: &reqwest::Client,
    watching: &mut HashMap<String, (Pair, Watch)>,
    thickness: Thickness,
) -> Result<()> {
    let key = std::env::var("TWELVE_DATA_API_KEY").context("TWELVE_DATA_API_KEY is not set")?;
    let url = format!("wss://ws.twelvedata.com/v1/quotes/price?apikey={key}");

    // Never print `url`. The key is in it.
    let (mut socket, _) = connect_async(&url)
        .await
        .context("the price line would not open")?;

    let symbols: Vec<&str> = watching.keys().map(String::as_str).collect();
    let ask = serde_json::json!({
        "action": "subscribe",
        "params": { "symbols": symbols.join(",") }
    });

    socket
        .send(Message::Text(ask.to_string()))
        .await
        .context("could not ask for prices")?;

    println!("\nlistening. Nothing will be said unless price reaches a level.\n");

    while let Some(heard) = socket.next().await {
        let heard = heard.context("the price line broke")?;

        let Ok(said) = serde_json::from_str::<serde_json::Value>(&heard.to_string()) else {
            continue;
        };

        if said["event"] != "price" {
            continue;
        }

        let (Some(symbol), Some(price)) = (said["symbol"].as_str(), price_in(&said)) else {
            continue;
        };

        let Some((pair, watch)) = watching.get_mut(symbol) else {
            continue;
        };

        for (band, near) in watch.arrive(price) {
            println!("{symbol} reached {}", band.price);

            shout(client, pair, &band, near, price, pair.reach(thickness)).await?;
        }
    }

    println!("the other side hung up.");
    Ok(())
}

/// The price out of a message, as a Decimal — never through a float.
fn price_in(said: &serde_json::Value) -> Option<Decimal> {
    said["price"]
        .as_str()
        .and_then(|text| text.parse().ok())
        .or_else(|| {
            said["price"]
                .as_f64()
                .and_then(|number| Decimal::try_from(number).ok())
        })
}

/// Draws the alert and sends it as a picture.
///
/// **An alert is not a signal.** No entry, no stop, no target — because there
/// is no trade. Price has arrived where he would be waiting and nothing has
/// formed yet. The card says so on its own face.
///
/// Each pair gets its own file so two alerts landing seconds apart cannot
/// overwrite each other's picture between drawing and sending.
async fn shout(
    client: &reqwest::Client,
    pair: &Pair,
    band: &Band,
    near: Nearness,
    price: Decimal,
    reach: Decimal,
) -> Result<()> {
    let stamp = Utc::now().format("%-d %b · %H:%M UTC").to_string();
    let out = PathBuf::from(PREVIEW).join(format!("alert-{}.png", pair.symbol.replace('/', "")));

    let picture = card::alert(pair, band, near, price, reach, &stamp, &out)
        .with_context(|| format!("could not draw the alert for {}", pair.symbol))?;

    telegram::send_to(
        client,
        &OWNER.to_string(),
        &[&picture],
        &levels::caption(pair, band, near, price),
    )
    .await
    .context("could not send the alert")
}
