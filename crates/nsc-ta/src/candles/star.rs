//! Three candles: a push, a stall, and a push back the other way.

use nsc_core::candle::Candle;
use nsc_core::pattern::{Bias, CandleShape, PatternSighting};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::config::CandleSettings;
use crate::error::TaError;

/// Is this three-candle run a star?
///
/// A morning star is at a bottom: a strong fall, a small candle that settles
/// nothing, then a strong rise that takes back a good part of the fall. An
/// evening star is the same at a top.
///
/// Three things have to be true, and the third is what does the work:
///
///   - the two outer candles have real bodies, or it is three shrugs in a row
///   - the middle one has almost no body, which is the stall
///   - the third **closes well back into the first candle's body**
///
/// Without that last test, any small candle followed by any down candle would
/// be an evening star. It is what separates a reversal from a pause.
///
/// The only three-candle shape in the project. Everything else here reads one
/// candle or two.
pub(super) fn look(
    first: &Candle,
    middle: &Candle,
    last: &Candle,
    settings: &CandleSettings,
) -> Result<Option<PatternSighting>, TaError> {
    let (Some(one), Some(two), Some(shape)) = (
        first.proportions(),
        middle.proportions(),
        last.proportions(),
    ) else {
        return Ok(None);
    };

    let outer = settings.star_min_outer_body_share;
    if one.body() < outer
        || shape.body() < outer
        || two.body() > settings.star_max_middle_body_share
    {
        return Ok(None);
    }

    // The push and the push back have to be opposite ways round.
    let bias = if first.is_down() && last.is_up() {
        Bias::Bullish
    } else if first.is_up() && last.is_down() {
        Bias::Bearish
    } else {
        return Ok(None);
    };

    if !closed_back_into(first, last, settings) {
        return Ok(None);
    }

    Ok(Some(PatternSighting::new(
        CandleShape::Star,
        bias,
        last.open_time(),
        3,
        shape,
    )?))
}

/// Did the third candle give back enough of the first candle's body?
fn closed_back_into(first: &Candle, last: &Candle, settings: &CandleSettings) -> bool {
    let body = first.body().abs();
    if body.value() <= Decimal::ZERO {
        return false;
    }

    let Some(needed) = Decimal::from_f64(settings.star_min_close_into_first) else {
        return false;
    };

    let taken_back = if last.is_up() {
        // A morning star: the first candle fell, so measure up from its close.
        last.close() - first.close()
    } else {
        first.close() - last.close()
    };

    taken_back.value() >= body.value() * needed
}
