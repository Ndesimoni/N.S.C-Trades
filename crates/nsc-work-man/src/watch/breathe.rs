//! Waiting between requests, so the feed keeps answering at full speed.

/// How long to wait between requests for candles.
///
/// **IBKR allows 60 historical requests in any ten minutes.** That is one
/// every ten seconds sustained — slightly stricter than the eight a minute the
/// old feed allowed, so this number went up rather than down.
const BREATHE: std::time::Duration = std::time::Duration::from_secs(10);

/// Wait before asking for more candles.
///
/// **Go over the limit and IBKR does not refuse. It PACES.** The request
/// simply takes longer, and then longer — and a candle report that arrives
/// late enough is about a candle he has already watched close on his own
/// screen. There is no error to notice; the bot just gets slower at the one
/// thing it is for.
///
/// The number and the waiting live together on purpose. They were a constant
/// in one file and a `sleep` in two others, which is three places to look when
/// asking why the feed feels slow.
pub(crate) async fn breathe() {
    tokio::time::sleep(BREATHE).await
}
