//! Sizing a pair's bands, once, at startup.

use anyhow::{Context, Result};
use nsc_core::candle::normal_candle;
use nsc_core::levels::{Band, Pair, Thickness, Timeframe};
use nsc_data::source::{Interval, MarketDataSource};
use nsc_data::sources::ibkr::IbkrConnection;

use crate::retry::keep_trying;

use super::breathe::breathe;

/// How many candles to fetch to work out a normal one.
const HISTORY: usize = 60;

/// How many a "normal" candle is averaged over. Fourteen is the usual.
const NORMAL_OVER: usize = 14;

/// Which chart each of his timeframes is read off.
///
/// **`review/picture.rs` holds the same three.** Both go through `Interval`
/// now rather than through a hand-written string, so the two lists can
/// disagree about which timeframes are fetched but never about what a
/// timeframe IS.
const SIZE_OFF: [(Timeframe, Interval); 3] = [
    (Timeframe::Weekly, Interval::Week),
    (Timeframe::Daily, Interval::Day),
    (Timeframe::H4, Interval::H4),
];

/// Every level of a pair, as a band.
///
/// **This is the only fetching that happens no matter what.** After it, a
/// request only happens when price is actually at one of these.
pub async fn for_pair(
    ibkr: &IbkrConnection,
    pair: &Pair,
    thickness: Thickness,
) -> Result<Vec<Band>> {
    let mut sizes = Vec::new();

    for (timeframe, interval) in SIZE_OFF {
        // Only ask about a timeframe he has actually drawn on. Most pairs have
        // weekly levels and nothing else, and a request for a timeframe with no
        // levels on it is a request spent on nothing.
        //
        // Four pairs across three timeframes is twelve requests. Skipping the
        // empty ones makes it seven.
        if !pair.levels.iter().any(|line| line.timeframe == timeframe) {
            continue;
        }

        let bars = keep_trying(3, || ibkr.candles(&pair.symbol, interval, HISTORY))
            .await
            .with_context(|| {
                format!(
                    "could not size {} bands for {}",
                    interval.spoken(),
                    pair.symbol
                )
            })?;

        breathe().await;

        // They arrive newest first. A normal candle is read the way a chart is.
        let mut bars: Vec<_> = bars.iter().collect();
        bars.reverse();

        if let Some(size) = normal_candle(&bars, NORMAL_OVER) {
            sizes.push((timeframe, size));
        }
    }

    Ok(pair.bands(thickness, &sizes))
}
