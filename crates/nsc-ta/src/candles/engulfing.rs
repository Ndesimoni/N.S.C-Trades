//! One candle's body swallowing the one before it.

use nsc_core::candle::Candle;
use nsc_core::pattern::{Bias, CandleShape, PatternSighting};
use nsc_core::price::Price;

use crate::config::CandleSettings;
use crate::error::TaError;

/// Did this candle's body completely cover the one before it?
///
/// **Bodies only. Wicks are ignored** — that is the standard definition, and
/// it is the one worth keeping: the body is where the market spent the
/// session, and a wick is where it was rejected.
///
/// The two must be opposite colours, or it is a continuation rather than a
/// turn. And the first body has to be a real body, because otherwise almost
/// anything engulfs a doji and the word stops meaning anything.
pub(super) fn look(
    first: &Candle,
    second: &Candle,
    settings: &CandleSettings,
) -> Result<Option<PatternSighting>, TaError> {
    let (Some(before), Some(shape)) = (first.proportions(), second.proportions()) else {
        return Ok(None);
    };

    if before.body() < settings.engulfing_min_first_body_share {
        return Ok(None);
    }

    let opposite = (first.is_up() && second.is_down()) || (first.is_down() && second.is_up());
    if !opposite {
        return Ok(None);
    }

    if !covers(body_of(second), body_of(first)) {
        return Ok(None);
    }

    let bias = if second.is_up() {
        Bias::Bullish
    } else {
        Bias::Bearish
    };

    Ok(Some(PatternSighting::new(
        CandleShape::Engulfing,
        bias,
        second.open_time(),
        2,
        shape,
    )?))
}

/// The bottom and top of a candle's body, whichever way round it closed.
fn body_of(candle: &Candle) -> (Price, Price) {
    (
        candle.open().min(candle.close()),
        candle.open().max(candle.close()),
    )
}

fn covers(outer: (Price, Price), inner: (Price, Price)) -> bool {
    outer.0 <= inner.0 && outer.1 >= inner.1
}
