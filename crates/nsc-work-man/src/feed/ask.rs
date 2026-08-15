//! The asking itself.

use nsc_core::candle::Series;

use super::FeedError;

/// Fetch the most recent candles for a pair and timeframe.
///
/// The newest comes back **first** — the list runs backwards through time —
/// and the newest is usually the one still forming. Deciding which have
/// finished is `Bar::finished_by`, and it asks the clock.
///
/// **There is no version of this that knows which pair to ask about.** There
/// was, reading a hardcoded gold, and it made it possible to fetch the wrong
/// instrument by simply forgetting to say which.
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
        .map_err(|trouble| FeedError::Unreachable(quietly(trouble)))?;

    let body = reply
        .text()
        .await
        .map_err(|trouble| FeedError::Unreachable(quietly(trouble)))?;

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

/// A network failure, with the address taken off it.
///
/// **THE TOKEN IS IN THE URL**, and `reqwest` puts the URL it was trying into
/// the message. So "could not reach Twelve Data" arrived in the terminal with the
/// bot token printed in full — and from there into any log, any screenshot,
/// any paste of what went wrong.
///
/// It is the same rule already followed for the URL, which was written
/// down as "never print the url" and then not applied to the error path.
fn quietly(trouble: reqwest::Error) -> String {
    trouble.without_url().to_string()
}
