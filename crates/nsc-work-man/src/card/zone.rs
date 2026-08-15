//! The alert card — price arriving at one of his zones, drawn.
//!
//! **Why a picture and not a line of text.** Telegram gives no colour, no font
//! size and no layout, so every message looks like every other message. On a
//! card the state can actually look different, and the zone can be drawn on
//! the same shape he drew it on — he sees how close price is instead of
//! working it out from three numbers.

use std::path::{Path, PathBuf};

use rust_decimal::Decimal;

use super::{CardError, facts, fill};
use nsc_core::levels::{Band, Nearness, Pair};

/// The template this card is drawn from.
const TEMPLATE: &str = "alert.html";

/// Draws one alert and gives back the picture's absolute path.
///
/// `reach` is how close counted as arriving — the card draws that line, so it
/// answers "why now?" without anybody writing the explanation.
///
/// `stamp` is the moment it happened, already worded. Nothing in `nsc-core`
/// reads a clock, and nothing here works one out twice.
pub fn alert(
    pair: &Pair,
    band: &Band,
    near: Nearness,
    price: Decimal,
    reach: Decimal,
    stamp: &str,
    out: &Path,
) -> Result<PathBuf, CardError> {
    fill::draw(
        TEMPLATE,
        &[(
            "/*__ALERT__*/",
            facts::alert(pair, band, near, price, reach, stamp).to_string(),
        )],
        out,
    )
}
