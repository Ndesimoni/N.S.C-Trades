//! A long candle that opens at one extreme and runs from there.

use nsc_core::candle::Candle;
use nsc_core::pattern::{Bias, CandleShape, PatternSighting};
use nsc_core::price::PriceDistance;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::config::CandleSettings;
use crate::error::TaError;

/// Is this candle a belt-hold?
///
/// A bullish one opens at its low and closes near its high — no wick
/// underneath, one long push. Bearish is the mirror.
///
/// "No wick" cannot mean exactly zero. In real forex there is nearly always a
/// tick or two below the open, so a small share is allowed.
///
/// This is the one shape where size matters as well as proportion. A tiny
/// candle with the same proportions is not a belt-hold, it is a quiet minute,
/// so the candle also has to be tall in normal candles.
pub(super) fn look(
    candle: &Candle,
    atr: PriceDistance,
    settings: &CandleSettings,
) -> Result<Option<PatternSighting>, TaError> {
    let Some(shape) = candle.proportions() else {
        return Ok(None);
    };

    if shape.body() < settings.belt_hold_min_body_share || !tall_enough(candle, atr, settings) {
        return Ok(None);
    }

    let opening_wick = if candle.is_up() {
        shape.lower_wick()
    } else {
        shape.upper_wick()
    };

    if opening_wick > settings.belt_hold_max_open_wick_share {
        return Ok(None);
    }

    let bias = if candle.is_up() {
        Bias::Bullish
    } else if candle.is_down() {
        Bias::Bearish
    } else {
        // Opened and closed at the same price, so it has no side. It cannot
        // have a big body either, so this is unreachable in practice — but
        // guessing a direction here would be inventing one.
        return Ok(None);
    };

    Ok(Some(PatternSighting::new(
        CandleShape::BeltHold,
        bias,
        candle.open_time(),
        1,
        shape,
    )?))
}

fn tall_enough(candle: &Candle, atr: PriceDistance, settings: &CandleSettings) -> bool {
    if atr.value() <= Decimal::ZERO {
        return false;
    }

    let Some(needed) = Decimal::from_f64(settings.belt_hold_min_atr_multiple) else {
        return false;
    };

    candle.range().value() >= atr.value() * needed
}
