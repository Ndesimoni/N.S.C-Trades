//! Asking the feed, and never letting that end the run.

use anyhow::{Context, Result};
use nsc_core::candle::Bar;

use crate::watch::BREATHE;
use crate::{feed, retry::keep_trying};

/// The timeframes he executes on. The level's own timeframe says how thick the
/// band is; these say which candles report.
///
/// **Swap in the five-line version below to watch the rules work.** Five
/// minutes at a zone is a card every five minutes, which is unbearable to
/// trade on and exactly what you want when you are checking the thing fires
/// at all. Put it back to two when you have seen what you needed to see.
pub(super) const REPORT_ON: [(&str, i64); 2] = [("1h", 60), ("4h", 240)];

// pub(super) const REPORT_ON: [(&str, i64); 5] = [
//     ("5min", 5),
//     ("15min", 15),
//     ("30min", 30),
//     ("1h", 60),
//     ("4h", 240),
// ];

/// How many candles to ask for. Three is enough to find a finished one whether
/// or not the newest is still forming.
const FEW: usize = 3;

/// The last few candles, newest first.
///
/// **Which one has finished is asked of the clock, never of position in the
/// list.** The newest is usually still running, but not always — ask at
/// 16:00:02 and you get either the 16:00 candle already open, if a price has
/// landed, or the 15:00 one now finished, if none has.
pub(super) async fn fetch(
    client: &reqwest::Client,
    symbol: &str,
    interval: &str,
) -> Result<Vec<Bar>> {
    let series = keep_trying(3, || feed::for_pair(client, symbol, interval, FEW))
        .await
        .with_context(|| format!("could not get the {interval} candle for {symbol}"))?;

    tokio::time::sleep(BREATHE).await;

    Ok(series.values)
}
