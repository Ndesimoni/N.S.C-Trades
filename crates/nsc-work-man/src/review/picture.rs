//! Fetching the candles and drawing them.

use std::path::Path;

use anyhow::{Context, Result};

use crate::card;
use crate::retry::keep_trying;
use nsc_core::candle::{Bar, normal_candle};
use nsc_core::levels::{Pair, Thickness, Timeframe};
use nsc_data::source::{Interval, MarketDataSource};
use nsc_data::sources::ibkr::IbkrConnection;

use super::drawn::{Drawn, on_the_chart};

/// How many candles back. Enough weeks to hold levels drawn years apart.
const HISTORY: usize = 150;

/// How many candles a "normal" one is averaged over. Fourteen is the usual.
const NORMAL_OVER: usize = 14;

/// Which candles each of his charts is drawn from.
///
/// **`watch/bands.rs` holds this same list**, beside the fetch it does. They
/// are both `Interval` now, so the two can disagree about which timeframes
/// get fetched but never about what a timeframe IS.
fn interval(chart: Timeframe) -> Interval {
    match chart {
        Timeframe::Weekly => Interval::Week,
        Timeframe::Daily => Interval::Day,
        Timeframe::H4 => Interval::H4,
    }
}

/// Draws every level a pair has, on the chart he asked for.
///
/// **The weekly is the one that shows everything.** His levels are years
/// apart, so it is the only chart wide enough to hold them together. A daily
/// level shows as a thin line there — correct, and a reminder that a daily
/// level is really for looking at on a daily chart.
///
/// **The other two are for reading one level closely**, and they will often
/// show no band at all. See [`Drawn`].
///
/// **A band is sized off its own chart whatever is drawn.** A weekly band is
/// 0.35 of a normal WEEKLY candle even when it is shown on a 4-hour chart, so
/// the weekly and the daily are fetched every time. Only the 4-hour costs a
/// request of its own.
pub async fn picture_of(
    ibkr: &IbkrConnection,
    pair: &Pair,
    thickness: Thickness,
    chart: Timeframe,
    out: &Path,
) -> Result<Drawn> {
    let weekly = candles(ibkr, &pair.symbol, Interval::Week).await?;
    let daily = candles(ibkr, &pair.symbol, Interval::Day).await?;

    let weekly_candle = normal_candle(&weekly.iter().collect::<Vec<_>>(), NORMAL_OVER)
        .context("no weekly candles came back")?;
    let daily_candle = normal_candle(&daily.iter().collect::<Vec<_>>(), NORMAL_OVER)
        .context("no daily candles came back")?;

    let bands = pair.bands(
        thickness,
        &[
            (Timeframe::Weekly, weekly_candle),
            (Timeframe::Daily, daily_candle),
            // A 4-hour candle is not fetched — sizing its band off the daily is
            // close enough to look at, and this picture is for looking at.
            (Timeframe::H4, daily_candle),
        ],
    );

    // **Two of the three are already in hand.** They were fetched for the band
    // sizes above, so asking again would be a request spent on candles we are
    // already holding.
    let shown = match chart {
        Timeframe::Weekly => weekly,
        Timeframe::Daily => daily,
        Timeframe::H4 => candles(ibkr, &pair.symbol, interval(chart)).await?,
    };

    // How much price the drawn candles actually cover. A band outside it is on
    // the pair but not on this picture.
    let low = shown.iter().map(|bar| bar.low).min();
    let high = shown.iter().map(|bar| bar.high).max();

    let on_it = match (low, high) {
        (Some(low), Some(high)) => on_the_chart(&bands, low, high),
        // No candles came back. `render` refuses that below, so this only has
        // to be honest rather than clever.
        _ => 0,
    };

    let drawn: Vec<&Bar> = shown.iter().collect();

    let picture = card::render(
        "chart.html",
        &drawn,
        &bands,
        &pair.symbol,
        card::as_written(interval(chart)),
        pair.digits,
        out,
    )?;

    Ok(Drawn {
        picture,
        on_it,
        altogether: bands.len(),
    })
}

/// Candles, oldest first — the direction a chart is read in.
///
/// **Tries again if the trouble says it is worth it.** A dropped line clears
/// on its own; a wrong key does not, and stops on the first go rather than
/// looking like a dead connection for a minute.
async fn candles(ibkr: &IbkrConnection, symbol: &str, interval: Interval) -> Result<Vec<Bar>> {
    let mut bars = keep_trying(3, || ibkr.candles(symbol, interval, HISTORY))
        .await
        .with_context(|| format!("could not get {} candles for {symbol}", interval.spoken()))?;

    bars.reverse();

    Ok(bars)
}
