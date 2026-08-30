//! The one sentence that explains a signal.
//!
//! **Every signal must be explainable in one sentence.** That is a `CLAUDE.md`
//! rule, and it is a test of the rules rather than of the wording: if the
//! sentence cannot be written, the rules are too loose. Fix the rules.

use super::Signal;
use super::standing::Standing;

/// Says what was seen, in one line.
///
/// Four facts and no more: **the shape, the pair and timeframe, the level, and
/// what it did to the band.**
///
/// ```text
///     nsc-bull on XAU/USD 4h, just below your weekly 4094 zone,
///     and it closed outside the band
///
/// ```
pub fn sentence(signal: &Signal, symbol: &str, timeframe: &str, digits: u32) -> String {
    let name = signal.shape.name();
    let band = signal.standing.band();
    let level = band.price.round_dp(digits);
    let where_it_is = signal.standing.placing().words();

    let broke = if signal.standing.broke_out() {
        ", and it closed outside the band"
    } else {
        ""
    };

    format!(
        "{name} on {symbol} {timeframe}, {where_it_is} your {} {level} zone{broke}",
        band.timeframe.name(),
    )
}

/// The headline a card leads with.
///
/// **It never says buy or sell.** Where the stop goes has not been settled,
/// and a signal with no stop is a reading rather than a trade — so it reports
/// what printed and stops there. Version 1 sends signals and places no trades.
pub fn headline(signal: &Signal) -> String {
    match signal.standing {
        Standing::Inside { .. } => format!("{} in your zone", signal.shape.name()),
        Standing::Close { .. } => format!("{} extremely close to your zone", signal.shape.name()),
    }
}
