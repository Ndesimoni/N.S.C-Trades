//! Sizing a pair's bands, once, at startup.

use anyhow::{Context, Result};
use nsc_core::candle::normal_candle;
use nsc_core::levels::{Band, Pair, Thickness, Timeframe};
use nsc_work_man::{feed, retry::keep_trying};

use super::BREATHE;

/// How many candles to fetch to work out a normal one.
const HISTORY: usize = 60;

/// How many a "normal" candle is averaged over. Fourteen is the usual.
const NORMAL_OVER: usize = 14;

/// Every level of a pair, as a band.
///
/// **This is the only fetching that happens no matter what.** After it, a
/// request only happens when price is actually at one of these.
pub async fn for_pair(
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
        // Only ask about a timeframe he has actually drawn on. Most pairs have
        // weekly levels and nothing else, and a request for a timeframe with no
        // levels on it is a request spent on nothing.
        //
        // Four pairs across three timeframes is twelve requests. Skipping the
        // empty ones makes it seven.
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
