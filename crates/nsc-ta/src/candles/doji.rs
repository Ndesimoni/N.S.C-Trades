//! Open and close in nearly the same place.

use nsc_core::candle::Candle;
use nsc_core::pattern::{Bias, CandleShape, DojiKind, PatternSighting};

use crate::config::CandleSettings;
use crate::error::TaError;

/// Is this candle a doji, and which kind?
///
/// One rule — almost no body — and then the wicks say which sort it is. They
/// are not the same event: a dragonfly is a rejection of lower prices and a
/// gravestone is a rejection of higher ones.
///
/// The bias is always neutral. A doji is the market failing to pick a side,
/// which is precisely why it needs context to mean anything.
pub(super) fn look(
    candle: &Candle,
    settings: &CandleSettings,
) -> Result<Option<PatternSighting>, TaError> {
    let Some(shape) = candle.proportions() else {
        return Ok(None);
    };

    if shape.body() > settings.doji_max_body_share {
        return Ok(None);
    }

    let missing = settings.doji_max_missing_wick_share;
    let no_upper = shape.upper_wick() <= missing;
    let no_lower = shape.lower_wick() <= missing;

    let kind = match (no_upper, no_lower) {
        (true, true) => DojiKind::Plain,
        (true, false) => DojiKind::Dragonfly,
        (false, true) => DojiKind::Gravestone,
        (false, false) => DojiKind::LongLegged,
    };

    Ok(Some(PatternSighting::new(
        CandleShape::Doji(kind),
        Bias::Neutral,
        candle.open_time(),
        1,
        shape,
    )?))
}
