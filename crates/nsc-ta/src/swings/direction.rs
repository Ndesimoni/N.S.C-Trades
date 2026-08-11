//! Which way round a leg is.
//!
//! Every comparison in the finder is "further along the run" or "further back
//! against it", and which of `>` and `<` that means flips with the direction.
//! Written once here, those flips cannot get out of step with each other —
//! which they silently would if each check had its own `match`.

use nsc_core::candle::Candle;
use nsc_core::price::Price;
use nsc_core::swing::SwingKind;

/// The candle's price in the direction the run is going, and the one against
/// it. On the way up that is the high and the low.
pub(super) fn sides(hunting: SwingKind, candle: &Candle) -> (Price, Price) {
    match hunting {
        SwingKind::High => (candle.high(), candle.low()),
        SwingKind::Low => (candle.low(), candle.high()),
    }
}

/// Has price got further along the run than this mark?
pub(super) fn is_beyond(hunting: SwingKind, price: Price, mark: Price) -> bool {
    match hunting {
        SwingKind::High => price > mark,
        SwingKind::Low => price < mark,
    }
}

/// Has price come further back against the run than this mark?
pub(super) fn is_behind(hunting: SwingKind, price: Price, mark: Price) -> bool {
    is_beyond(hunting, mark, price)
}
