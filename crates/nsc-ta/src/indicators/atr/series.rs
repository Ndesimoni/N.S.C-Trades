//! ATR for a whole history at once.

use nsc_core::candle::Candle;
use nsc_core::price::PriceDistance;

use super::Atr;
use crate::error::TaError;

/// Works out ATR for a whole history at once.
///
/// The list that comes back lines up with the candles: position 0 is the ATR
/// after the first candle, and so on. The early ones are `None` because there
/// was not enough history yet.
///
/// This feeds the candles through the same [`Atr`] the live bot uses. Not a
/// second copy of the maths — the same struct. So the backtester and the bot
/// cannot drift apart, because there is only one piece of code to drift.
pub fn atr_series(
    candles: &[Candle],
    period: usize,
) -> Result<Vec<Option<PriceDistance>>, TaError> {
    let mut atr = Atr::new(period)?;
    let mut out = Vec::with_capacity(candles.len());

    for candle in candles {
        out.push(atr.update(candle)?);
    }

    Ok(out)
}
