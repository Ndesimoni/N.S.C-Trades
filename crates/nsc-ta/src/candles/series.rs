//! Every shape in a whole history at once.

use nsc_core::candle::Candle;
use nsc_core::pattern::PatternSighting;

use super::look_at;
use crate::config::CandleSettings;
use crate::error::TaError;
use crate::indicators::atr::atr_series;

/// Finds every candlestick shape in a run of candles.
///
/// ATR is worked out as it goes, so each candle is judged against how big a
/// normal candle was **at the time** rather than against today's. Judging a
/// quiet week by this week's volatility would find shapes that were not there.
///
/// Candles before ATR has warmed up produce nothing rather than an error. The
/// start of a history is not a fault, it is just the start of a history.
///
/// This runs the same [`look_at`] the live bot uses, so the two cannot drift.
pub fn find_patterns(
    candles: &[Candle],
    settings: &CandleSettings,
    atr_period: usize,
) -> Result<Vec<PatternSighting>, TaError> {
    let atr = atr_series(candles, atr_period)?;
    let mut found = Vec::new();

    for (index, at_the_time) in atr.iter().enumerate() {
        let Some(at_the_time) = at_the_time else {
            continue;
        };

        let from = index.saturating_sub(1);
        found.extend(look_at(&candles[from..=index], *at_the_time, settings)?);
    }

    Ok(found)
}
