//! Asking the feed, and never letting that end the run.

use anyhow::{Context, Result};
use nsc_core::candle::Bar;
use nsc_data::source::{Interval, MarketDataSource};
use nsc_data::sources::ibkr::IbkrConnection;

use crate::retry::keep_trying;
use crate::watch::breathe::breathe;

/// The timeframes he executes on. The level's own timeframe says how thick the
/// band is; these say which candles report.
///
/// **Swap in the five-line version below to watch the rules work.** Five
/// minutes at a zone is a card every five minutes, which is unbearable to
/// trade on and exactly what you want when you are checking the thing fires
/// at all. Put it back to two when you have seen what you needed to see.
pub(super) const REPORT_ON: [Interval; 2] = [Interval::H1, Interval::H4];

// pub(super) const REPORT_ON: [Interval; 5] = [
//     Interval::Min5,
//     Interval::Min15,
//     Interval::Min30,
//     Interval::H1,
//     Interval::H4,
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
    ibkr: &IbkrConnection,
    symbol: &str,
    interval: Interval,
) -> Result<Vec<Bar>> {
    let bars = keep_trying(3, || ibkr.candles(symbol, interval, FEW))
        .await
        .with_context(|| {
            format!(
                "could not get the {} candle for {symbol}",
                interval.spoken()
            )
        })?;

    breathe().await;

    Ok(bars)
}
