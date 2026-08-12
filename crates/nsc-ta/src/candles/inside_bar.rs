//! A candle that fits entirely inside the one before it.

use nsc_core::candle::Candle;
use nsc_core::pattern::{Bias, CandleShape, PatternSighting};

use crate::error::TaError;

/// Does this candle sit wholly inside the one before it?
///
/// High no higher, low no lower. The market spent the whole session inside
/// yesterday's range and settled nothing.
///
/// **It points nowhere**, which is why the bias is neutral. An inside bar is a
/// coil — the part that matters is which way price leaves it, and that is a
/// rule rather than a shape. Calling it bullish or bearish here would be
/// inventing a direction it does not have.
///
/// Two identical candles are not an inside bar. Nothing narrowed, so nothing
/// coiled.
///
/// No settings. This one is pure geometry: either the range is inside or it is
/// not, and there is no threshold to get wrong.
pub(super) fn look(first: &Candle, second: &Candle) -> Result<Option<PatternSighting>, TaError> {
    let Some(shape) = second.proportions() else {
        return Ok(None);
    };

    let contained = second.high() <= first.high() && second.low() >= first.low();
    let narrower = second.high() < first.high() || second.low() > first.low();

    if !contained || !narrower {
        return Ok(None);
    }

    Ok(Some(PatternSighting::new(
        CandleShape::InsideBar,
        Bias::Neutral,
        second.open_time(),
        2,
        shape,
    )?))
}
