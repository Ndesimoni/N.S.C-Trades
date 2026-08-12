//! A long wick with a small body at the far end of it.

use nsc_core::candle::Candle;
use nsc_core::pattern::{Bias, CandleShape, PatternSighting};

use crate::config::CandleSettings;
use crate::error::TaError;

/// Is this candle a pin bar?
///
/// Four measurements, all shares of the candle's own height:
///
///   - the tail is several times the body
///   - the body is small
///   - the wick on the other side, the nose, is smaller still
///
/// The nose test is what keeps the body at the far end from the tail. Without
/// it a candle with long wicks both sides — which is a doji, and means the
/// opposite thing — would pass.
///
/// A tail pointing down is bullish. Textbook calls that a hammer after a
/// downtrend and a plain pin bar elsewhere, but the shape is the same and the
/// downtrend part belongs to the rules.
pub(super) fn look(
    candle: &Candle,
    settings: &CandleSettings,
) -> Result<Option<PatternSighting>, TaError> {
    let Some(shape) = candle.proportions() else {
        return Ok(None);
    };

    if shape.shorter_wick() > settings.pin_max_nose_share {
        return Ok(None);
    }

    // The body has to sit in the far end of the candle, not just be small.
    // Measuring nose plus body from that end is what puts it there — a candle
    // with a quarter nose, a quarter body and a half tail passes every other
    // test here, and its body is sitting in the middle. That is a spinning top
    // leaning one way, not a rejection.
    if shape.shorter_wick() + shape.body() > settings.pin_max_body_share {
        return Ok(None);
    }

    // No body at all means an endlessly long tail, which passes. That candle
    // is also a doji, and both get reported — they are two true things about
    // one candle, and the rules decide which matters.
    let long_enough = shape
        .tail_to_body()
        .is_none_or(|ratio| ratio >= settings.pin_min_tail_to_body);

    if !long_enough {
        return Ok(None);
    }

    let bias = if shape.tail_points_down() {
        Bias::Bullish
    } else {
        Bias::Bearish
    };

    Ok(Some(PatternSighting::new(
        CandleShape::PinBar,
        bias,
        candle.open_time(),
        1,
        shape,
    )?))
}
