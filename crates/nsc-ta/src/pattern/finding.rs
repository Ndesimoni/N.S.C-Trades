//! The one way in, and the order the patterns are tested in.

use nsc_core::candle::Bar;
use rust_decimal::Decimal;

use super::{Pattern, Rules, three, two};

/// What pattern, if any, **finishes on the last candle handed in**.
///
/// **The name is the safety.** You hand over the candles up to and including
/// the one being judged, and it looks backwards from the end. There is no
/// argument it could use to see forwards, so the rule that matters most in
/// this project — never use price the market had not printed yet — is not a
/// discipline here, it is the shape of the function.
///
/// `normal` is how big a normal candle was **at that moment**, not today. Hand
/// over today's and a pattern from 2023 gets judged against a market that had
/// not happened yet.
///
/// **One pattern, tested longest first.** Three candles beats two: a star that
/// happens to end in an engulfing is a star, and reporting both would let a
/// backtest count one setup twice.
pub fn ending_at(bars: &[&Bar], normal: Decimal, rules: &Rules) -> Option<Pattern> {
    let last = bars.len();

    if last >= 3
        && let found = three::ending_at(bars[last - 3], bars[last - 2], bars[last - 1], rules)
        && found.is_some()
    {
        return found;
    }

    if last >= 2 {
        return two::ending_at(bars[last - 2], bars[last - 1], normal, rules);
    }

    None
}
