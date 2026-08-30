//! The one way in.

use nsc_core::candle::Bar;
use nsc_core::levels::Band;
use nsc_ta::pattern;
use rust_decimal::Decimal;

use super::Rules;
use super::place::{self, Placing};
use super::shape::{Traded, traded};
use super::standing::Standing;

/// A shape he trades, and where it printed.
#[derive(Debug, Clone)]
pub struct Signal {
    pub shape: Traded,

    /// **The three tiers** — in the zone, close to it, or bold and away from
    /// every one of them. See `standing.rs`.
    pub standing: Standing,

    /// How big the shape is, in normal candles.
    ///
    /// **On the card either way.** At a zone it says how plainly the thing
    /// happened; away from one it is the whole reason the message was sent.
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
/// ## The three answers, in the order they are tried
///
/// ```text
///     at one of his bands           Inside, or Close
///     no band near it, but big      Bold
///     no band, and ordinary         nothing
/// ```
///
/// **A level beats size, always.** A shape at a zone is a setup whatever its
/// reach; a shape away from every zone is only ever a remark. Testing size
/// first would let a big candle in open water outrank a modest one sitting
/// exactly where he was watching.
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
    let reach = shape.reach(bars, normal)?;

    // **The shape says which candle reached the level, not this function.** A
    // harami reaches on its big first candle and a march on the one it started
    // from, so the whole slice goes over rather than the last bar.
    let touching = shape.touching(bars)?;

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

        // Nowhere near a zone. Worth saying only if it is plainly bigger than
        // an ordinary candle — and `nsc-ta` has already refused anything under
        // one, so this asks for *bolder than ordinary*, not merely real.
        None if reach >= rules.bold_reach => Standing::Bold,
        None => return None,
    };

    Some(Signal {
        shape,
        standing,
        reach,
    })
}
