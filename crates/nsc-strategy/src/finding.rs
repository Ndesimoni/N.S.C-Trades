//! The one way in.

use nsc_core::candle::Bar;
use nsc_core::levels::Band;
use nsc_ta::pattern;
use rust_decimal::Decimal;

use super::Rules;
use super::place::{self, Placing};
use super::refused::Refused;
use super::shape::{Traded, traded};
use super::standing::Standing;

/// A shape he trades, and where it printed.
#[derive(Debug, Clone)]
pub struct Signal {
    pub shape: Traded,

    /// **The two tiers** — in the zone, or within half a band of it. See
    /// `standing.rs`.
    pub standing: Standing,

    /// How big the shape is, in normal candles. **On the card**, because it
    /// says how plainly the thing happened.
    pub reach: Decimal,
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
///
/// ## It says WHY it said no
///
/// The answer is `Result`, not `Option`, since 2 September 2026. Nothing is
/// the honest answer for the market and a useless one for the record —
/// `CLAUDE.md` has asked for the refusals to be saved since the beginning, and
/// they cannot be saved if the only thing handed back is the absence of a
/// signal. See [`Refused`].
///
/// ## No level, no signal
///
/// **A shape away from every one of his zones says nothing at all**, however
/// big it is. He settled that on 30 August after a day of the other way:
/// `nsc-bull` and `nsc-bear` measured with no level under them came back at
/// 38%, and four messages a day of that is four a day of nothing.
pub fn look(
    bars: &[&Bar],
    bands: &[Band],
    normal: Decimal,
    patterns: &pattern::Rules,
    rules: &Rules,
) -> Result<Signal, Refused> {
    // A shape first, because it is the cheap test and most candles have none.
    let Some(found) = pattern::ending_at(bars, normal, patterns) else {
        return Err(Refused::NoShape);
    };

    let Some(shape) = traded(found) else {
        return Err(Refused::NotHis { pattern: found });
    };

    let (Some(last), Some(reach)) = (bars.last(), shape.reach(bars, normal)) else {
        return Err(Refused::Unmeasurable { shape });
    };

    // **The shape says which candle reached the level, not this function.** A
    // harami reaches on its big first candle and a march on the one it started
    // from, so the whole slice goes over rather than the last bar.
    let Some(touching) = shape.touching(bars) else {
        return Err(Refused::Unmeasurable { shape });
    };

    let standing = match place::nearest(touching, bands, rules) {
        Some((band, Placing::Inside)) => Standing::Inside {
            band: *band,

            // **The candle's CLOSE, not its wick.** A tail through the band is
            // the level being tested; a close outside it is the level being
            // left.
            broke_out: last.close > band.top || last.close < band.bottom,
        },

        Some((band, placing)) => Standing::Close {
            band: *band,
            placing,
        },

        // Nowhere near a zone. **The level is what makes a shape worth
        // anything**, so there is nothing to say — but it is worth writing
        // down, and that is what `Refused` is for.
        None => return Err(Refused::NoLevel { shape, touching }),
    };

    Ok(Signal {
        shape,
        standing,
        reach,
    })
}
