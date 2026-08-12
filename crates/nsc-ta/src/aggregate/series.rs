//! Building a whole history of bigger candles at once.

use nsc_core::candle::Candle;
use nsc_core::timeframe::{DayBoundary, Timeframe};

use super::Aggregator;
use crate::error::TaError;

/// Builds every finished bigger candle in a run of smaller ones.
///
/// **The last bucket is left out**, because nothing here can know whether more
/// smaller candles are coming. On a history that ends mid-session that is a
/// part-formed candle, and handing it back as finished is the exact mistake
/// this module exists to prevent.
///
/// A backtest that wants that last candle is a backtest reading the future.
///
/// This feeds candles through the same [`Aggregator`] the live bot uses. Not a
/// second copy of the logic — the same struct.
pub fn aggregate(
    candles: &[Candle],
    from: Timeframe,
    into: Timeframe,
    boundary: &DayBoundary,
) -> Result<Vec<Candle>, TaError> {
    let mut builder = Aggregator::new(from, into, *boundary)?;
    let mut bigger = Vec::new();

    for candle in candles {
        bigger.extend(builder.update(candle)?);
    }

    Ok(bigger)
}
