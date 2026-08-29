//! The one way in.

use nsc_core::candle::Bar;
use nsc_core::levels::Band;
use nsc_ta::pattern;
use rust_decimal::Decimal;

use super::Rules;
use super::place::{self, Placing};
use super::shape::{Traded, traded};

/// A shape he trades, at a level he drew.
#[derive(Debug, Clone)]
pub struct Signal {
    pub shape: Traded,

    /// The band it printed at, and where against it.
    pub band: Band,
    pub placing: Placing,

    /// **Did the shape's own candle close outside the band?**
    ///
    /// Reported, never required. He was asked whether the break was the
    /// trigger and answered that the shape at the zone is a signal either way
    /// — so this is a fact about the signal rather than a gate on it.
    pub broke_out: bool,
}

/// Looks for a signal on the candle that just closed.
///
/// **`ending_at` is the safety, and the name is the point.** `bars` are the
/// candles up to and including the one being judged, and there is no argument
/// that could let this see forwards. The rule that matters most in this
/// project — never use price the market had not printed yet — is the shape of
/// the function rather than a discipline.
///
/// `normal` is how big a normal candle was **at that moment**, not today.
/// `bands` are his levels on that pair, already sized.
pub fn look(
    bars: &[&Bar],
    bands: &[Band],
    normal: Decimal,
    patterns: &pattern::Rules,
    rules: &Rules,
) -> Option<Signal> {
    // A shape first, because it is the cheap test and most candles have none.
    let found = pattern::ending_at(bars, normal, patterns)?;
    let shape = traded(found)?;

    let last = bars.last()?;

    // **The shape says which candle reached the level, not this function.** A
    // harami reaches on its big first candle and a march on the one it started
    // from, so the whole slice goes over rather than the last bar.
    let (band, placing) = place::nearest(shape.touching(bars)?, bands, rules)?;

    Some(Signal {
        shape,
        band: *band,
        placing,

        // **The candle's CLOSE, not its wick.** A tail through the band is the
        // level being tested; a close outside it is the level being left.
        broke_out: last.close > band.top || last.close < band.bottom,
    })
}
