//! Reading trend across a whole history at once.

use nsc_core::candle::Candle;
use nsc_core::structure::StructureEvent;
use nsc_core::swing::Swing;

use super::StructureReader;
use crate::config::StructureSettings;
use crate::error::TaError;

/// Finds everything that happened at an old extreme in a run of candles —
/// the ones price took, and the ones it tried and could not hold past.
///
/// Takes the swings as [`crate::swings::find_swings`] gives them back — in
/// confirmation order — and hands each one to the reader on the candle it
/// became knowable, so nothing is used a moment before it existed.
///
/// This feeds candles through the same [`StructureReader`] the live bot uses.
/// Not a second copy of the logic — the same struct.
pub fn read_structure(
    candles: &[Candle],
    swings: &[Swing],
    settings: &StructureSettings,
) -> Result<Vec<StructureEvent>, TaError> {
    let mut reader = StructureReader::new(settings.clone())?;
    let mut found = Vec::new();
    let mut next = 0;

    for candle in candles {
        let from = next;
        while swings
            .get(next)
            .is_some_and(|swing| swing.confirmed_at() <= candle.open_time())
        {
            next += 1;
        }

        found.extend(reader.update(candle, &swings[from..next])?);
    }

    Ok(found)
}
