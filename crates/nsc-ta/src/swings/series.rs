//! Finding swings across a whole history at once.

use nsc_core::candle::Candle;
use nsc_core::swing::Swing;

use super::SwingFinder;
use crate::config::SwingSettings;
use crate::error::TaError;

/// Finds every swing in a run of candles.
///
/// The swings come back in the order they were **confirmed**, not the order
/// they sit on the chart. Those are nearly the same thing, but not quite —
/// and confirmation order is the one that matters, because it is the order
/// you would have learned about them in real time.
///
/// The last few candles produce nothing, because there are not enough
/// candles after them yet to tell. That is correct, not a gap.
///
/// This feeds candles through the same [`SwingFinder`] the live bot uses.
/// Not a second copy of the logic — the same struct. So the backtester and
/// the bot cannot drift apart, because there is only one thing to drift.
pub fn find_swings(
    candles: &[Candle],
    settings: SwingSettings,
    atr_period: usize,
) -> Result<Vec<Swing>, TaError> {
    let mut finder = SwingFinder::new(settings, atr_period)?;
    let mut found = Vec::new();

    for candle in candles {
        found.extend(finder.update(candle)?);
    }

    Ok(found)
}
