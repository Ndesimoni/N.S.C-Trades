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

    serde_json::from_str(&body).map_err(|_| FeedError::NotCandles(shortened(body)))
}

/// A network failure, with the address taken off it.
///
/// **THE API KEY IS IN THE URL**, and `reqwest` puts the URL it was trying
/// into its message. So "could not reach Twelve Data" carried the key in full
/// — into the terminal, and from there into any log or screenshot of what went
/// wrong.
///
/// The rule was already written three lines up: never print the url, the key
/// is in it. It was followed on the way out and never applied to the way it
/// fails, which is the path that actually prints.
fn quietly(trouble: reqwest::Error) -> String {
    trouble.without_url().to_string()
}

/// Enough of a reply to recognise it, and no more.
///
/// **What comes back when it is not candles can be a whole web page.** That
/// string is the error, and the error ends up on a trouble card and in the
/// terminal. The first line of an HTML error page says what it is; the other
/// four thousand characters do not.
pub(super) fn shortened(body: String) -> String {
    const ENOUGH: usize = 300;

    match body.char_indices().nth(ENOUGH) {
        None => body,
        Some((at, _)) => format!("{}…", &body[..at]),
    }
}
