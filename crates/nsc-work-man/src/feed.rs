//! Asking Twelve Data for candles.
//!
//! One request, one answer. The live price stream is a separate thing and does
//! not exist yet — see `PROGRESS.md`.
//!
//! **The candle is never computed.** It comes from the feed finished, exactly
//! as it appears on the chart. Building one out of smaller candles or out of
//! ticks would produce something close to the broker's, never the same, and
//! then nobody could say which was right.

use anyhow::{Context, Result};

use crate::candle::Series;
use crate::settings::{HISTORY, INTERVAL, SYMBOL};

/// Fetch the most recent candles.
///
/// The newest comes back **first** — the list runs backwards through time —
/// and the newest is usually the one still forming. Deciding which have
/// finished is `Bar::is_finished`, and it asks the clock.
pub async fn candles(client: &reqwest::Client) -> Result<Series> {
    let key = std::env::var("TWELVE_DATA_API_KEY")
        .context("TWELVE_DATA_API_KEY is not set. Is there a .env file in the project root?")?;

    // `timezone=UTC` is not optional. Without it they answer in the exchange's
    // local time, and everything in this project is UTC.
    let url = format!(
        "https://api.twelvedata.com/time_series\
         ?symbol={SYMBOL}&interval={INTERVAL}&outputsize={HISTORY}&timezone=UTC&apikey={key}"
    );

    // Never print `url`. The key is in it.
    let body = client
        .get(&url)
        .send()
        .await
        .context("could not reach Twelve Data at all — is the machine online?")?
        .text()
        .await
        .context("Twelve Data answered, but the reply could not be read")?;

    // Twelve Data refuses with a perfectly ordinary reply — `{"code": 401,
    // "status": "error"}` and a 200. So a reply that parses is not a reply that
    // worked, and the body goes into the error where it can be read.
    serde_json::from_str(&body)
        .with_context(|| format!("Twelve Data did not send candles. It sent this instead:\n{body}"))
}
