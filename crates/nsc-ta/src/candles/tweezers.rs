//! Two candles reaching the same price and turning there.

use nsc_core::candle::Candle;
use nsc_core::pattern::{Bias, CandleShape, PatternSighting};
use nsc_core::price::PriceDistance;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::config::CandleSettings;
use crate::error::TaError;

/// Do these two candles top out — or bottom out — at the same price?
///
/// "The same" needs a tolerance. Two candles never share a high to the tick,
/// and demanding they do would find almost nothing. The tolerance is in normal
/// candles, so it means the same thing on gold as on EURUSD.
///
/// Opposite colours, per the textbook: up into the price then down away from
/// it is a rejection, two candles going the same way is not.
///
/// Worth knowing: this is the exact shape swing detection used to throw away.
/// The old finder refused ties, because neither candle strictly beat the
/// other. The rewritten one tracks a running extreme instead, so a tweezer
/// top can be a swing high as well.
pub(super) fn look(
    first: &Candle,
    second: &Candle,
    atr: PriceDistance,
    settings: &CandleSettings,
) -> Result<Option<PatternSighting>, TaError> {
    let Some(shape) = second.proportions() else {
        return Ok(None);
    };

    let Some(tolerance) = tolerance(atr, settings) else {
        return Ok(None);
    };

    let opposite = (first.is_up() && second.is_down()) || (first.is_down() && second.is_up());
    if !opposite {
        return Ok(None);
    }

    let tops_match = (second.high() - first.high()).abs() <= tolerance;
    let bottoms_match = (second.low() - first.low()).abs() <= tolerance;

    // Both matching means the two candles cover the same ground entirely.
    // That is a stalled market rather than a rejection at one end, and calling
    // it a top or a bottom would be picking one at random.
    let bias = match (tops_match, bottoms_match) {
        (true, false) => Bias::Bearish,
        (false, true) => Bias::Bullish,
        _ => return Ok(None),
    };

    Ok(Some(PatternSighting::new(
        CandleShape::Tweezers,
        bias,
        second.open_time(),
        2,
        shape,
    )?))
}

fn tolerance(atr: PriceDistance, settings: &CandleSettings) -> Option<PriceDistance> {
    if atr.value() <= Decimal::ZERO {
        return None;
    }

    let share = Decimal::from_f64(settings.tweezer_tolerance_atr)?;

    Some(PriceDistance::new(atr.value() * share))
}
