//! What a candle did to the leg it arrived on.

use nsc_core::price::PriceDistance;
use nsc_core::swing::Swing;

use super::leg::Leg;

/// What the finder does with a candle.
pub(super) enum Step {
    /// Nothing proved itself. Carry on.
    Continue,

    /// The run is over. One or two swings are now known, and a fresh leg
    /// starts.
    Confirmed {
        swings: Vec<Swing>,
        next: Leg,
        run: PriceDistance,
    },
}
