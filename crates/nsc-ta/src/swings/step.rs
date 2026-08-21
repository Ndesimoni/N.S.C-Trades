//! What a candle did to the leg it arrived on.

use nsc_core::swing::Swing;
use rust_decimal::Decimal;

use super::leg::Leg;

/// What the finder does with a candle.
pub(super) enum Step {
    /// Nothing proved itself. Carry on.
    Continue,

    /// The run is over. **One or two swings** are now known, and a fresh leg
    /// starts.
    ///
    /// Two, on the shallow route: the peak is a swing high and the bottom of
    /// the pause is a swing low, and both become knowable on the same candle.
    Confirmed {
        swings: Vec<Swing>,
        next: Leg,
        run: Decimal,
    },
}
