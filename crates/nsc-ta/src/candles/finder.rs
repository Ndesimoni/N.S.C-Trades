//! Asking every detector what the newest candle is.

use nsc_core::candle::Candle;
use nsc_core::pattern::PatternSighting;
use nsc_core::price::PriceDistance;

use super::{belt_hold, doji, engulfing, inside_bar, pin_bar, star, tweezers};
use crate::config::CandleSettings;
use crate::error::TaError;

/// Every shape that completes on the last candle of `window`.
///
/// Give it the newest candle and the two before it. Anything longer is
/// ignored — no shape here is more than three candles — and a shorter window
/// still works, it simply cannot produce the shapes that need more candles
/// than it has.
///
/// ## One candle can be several things
///
/// A candle with no body and a long tail is a pin bar and a doji at once.
/// Both come back. They are two true statements about the same candle, and
/// deciding which matters needs the level it happened at and the trend it
/// happened in — which is the rules layer's job, not this one's.
///
/// ## It never looks left
///
/// Textbook descriptions bolt the context onto the pattern: a hammer *after a
/// downtrend*. That half is the strategy's. The same candle in open space is
/// still a hammer; it is simply not a trade.
pub fn look_at(
    window: &[Candle],
    atr: PriceDistance,
    settings: &CandleSettings,
) -> Result<Vec<PatternSighting>, TaError> {
    let Some(newest) = window.last() else {
        return Ok(Vec::new());
    };

    if !newest.is_complete() {
        return Err(TaError::IncompleteCandle {
            open_time: newest.open_time(),
        });
    }

    let mut seen = Vec::new();

    seen.extend(pin_bar::look(newest, settings)?);
    seen.extend(doji::look(newest, settings)?);
    seen.extend(belt_hold::look(newest, atr, settings)?);

    if let Some(before) = back(window, 2) {
        seen.extend(engulfing::look(before, newest, settings)?);
        seen.extend(tweezers::look(before, newest, atr, settings)?);
        seen.extend(inside_bar::look(before, newest)?);

        if let Some(first) = back(window, 3) {
            seen.extend(star::look(first, before, newest, settings)?);
        }
    }

    Ok(seen)
}

/// The candle `nth` from the end, counting the last one as the first.
fn back(window: &[Candle], nth: usize) -> Option<&Candle> {
    window.len().checked_sub(nth).and_then(|at| window.get(at))
}
