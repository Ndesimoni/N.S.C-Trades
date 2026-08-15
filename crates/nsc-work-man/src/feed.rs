//! Asking Twelve Data for candles.
//!
//! One request, one answer. The live price stream is a separate thing.
//!
//! **The candle is never computed.** It comes from the feed finished, exactly
//! as it appears on his chart. Building one out of smaller candles or out of
//! ticks would produce something close to the broker's, never the same, and
//! then nobody could say which was right.

use thiserror::Error;

use crate::candle::Series;
use crate::settings::{HISTORY, INTERVAL, SYMBOL};
use crate::trouble::{Answer, Knows};

/// What can go wrong asking for candles.
///
/// Named rather than lumped together, because **a bad key and a busy server
/// need opposite responses**. Retry the key forever and it looks exactly like
/// a dead connection.
#[derive(Debug, Error)]
pub enum FeedError {
    /// The key is not in `.env`.
    #[error("TWELVE_DATA_API_KEY is not set. Is there a .env file in the project root?")]
    NoKey,

    /// Could not reach them at all.
    #[error("could not reach Twelve Data: {0}")]
    Unreachable(String),

    /// They answered, and said no.
    ///
    /// **The code decides everything.** 401 is a wrong key and will be wrong
    /// forever; 429 is "slow down" and will be fine in a minute.
    #[error("Twelve Data refused: {code} {message}")]
    Refused { code: u16, message: String },

    /// They answered with something that is not candles.
    #[error("Twelve Data did not send candles:\n{0}")]
    NotCandles(String),
}

impl Knows for FeedError {
    fn answer(&self) -> Answer {
        match self {
            // No key is no key. Waiting will not put one there.
            FeedError::NoKey => Answer::GiveUp,

            // The line, or their end. Both clear on their own.
            FeedError::Unreachable(_) => Answer::soon(),

            FeedError::Refused { code, .. } => match code {
                // Too many requests. They have TOLD us to wait, so wait
                // properly rather than hammering.
                429 => Answer::in_a_while(),

                // Their end fell over.
                500..=599 => Answer::soon(),

                // A wrong key, a pair not on the plan, a malformed request.
                // None of those get better by asking again.
                _ => Answer::GiveUp,
            },

            // Could be a blip, could be their shape changing. Worth one more
            // go — whoever is retrying will stop counting eventually.
            FeedError::NotCandles(_) => Answer::soon(),
        }
    }
}

/// Fetch the most recent candles.
///
/// The newest comes back **first** — the list runs backwards through time —
/// and the newest is usually the one still forming. Deciding which have
/// finished is `Bar::is_finished`, and it asks the clock.
pub async fn candles(client: &reqwest::Client) -> Result<Series, FeedError> {
    for_pair(client, SYMBOL, INTERVAL, HISTORY).await
}

/// The same, for any pair and timeframe.
pub async fn for_pair(
    client: &reqwest::Client,
    symbol: &str,
    interval: &str,
    count: usize,
) -> Result<Series, FeedError> {
    let key = std::env::var("TWELVE_DATA_API_KEY").map_err(|_| FeedError::NoKey)?;

    // `timezone=UTC` is not optional. Without it they answer in the exchange's
    // local time, and everything in this project is UTC.
    let url = format!(
        "https://api.twelvedata.com/time_series\
         ?symbol={symbol}&interval={interval}&outputsize={count}&timezone=UTC&apikey={key}"
    );

    // Never print `url`. The key is in it.
    let reply = client
        .get(&url)
        .send()
        .await
        .map_err(|trouble| FeedError::Unreachable(trouble.to_string()))?;

    let body = reply
        .text()
        .await
        .map_err(|trouble| FeedError::Unreachable(trouble.to_string()))?;

    // **Twelve Data refuses with a perfectly ordinary reply** — a 200, and
    // `{"code": 401, "status": "error"}` in the body. So a reply that parses is
    // not a reply that worked, and the code has to be read out of the body
    // rather than off the response.
    if let Ok(said) = serde_json::from_str::<serde_json::Value>(&body)
        && said["status"] == "error"
    {
        return Err(FeedError::Refused {
            code: said["code"].as_u64().unwrap_or(0) as u16,
            message: said["message"].as_str().unwrap_or("no reason given").into(),
        });
    }

    serde_json::from_str(&body).map_err(|_| FeedError::NotCandles(body))
}
