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
use nsc_core::candle::Bar;
use nsc_core::levels::{Action, AtZone, Band, Nearness, News, Pair};

/// The templates these cards are drawn from.
const ALERT: &str = "alert.html";
const CLOSE: &str = "close.html";

/// Draws one alert and gives back the picture's absolute path.
///
/// `reach` is how close counted as arriving — the card draws that line, so it
/// answers "why now?" without anybody writing the explanation.
///
/// `stamp` is the moment it happened, already worded. Nothing in `nsc-core`
/// reads a clock, and nothing here works one out twice.
#[allow(clippy::too_many_arguments)]
pub fn alert(
    pair: &Pair,
    band: &Band,
    near: Nearness,
    news: News,
    price: Decimal,
    reach: Decimal,
    stamp: &str,
    out: &Path,
) -> Result<PathBuf, CardError> {
    fill::draw(
        ALERT,
        &[(
            "/*__ALERT__*/",
            facts::alert(pair, band, near, news, price, reach, stamp).to_string(),
        )],
        out,
    )
}

/// Draws one rung 2 card — a finished candle inside the zone it touched.
///
/// The candle is drawn ON the band, because a wick deep in with the body
/// closing back out is a rejection and no arrangement of numbers says that as
/// fast as the shape does.
#[allow(clippy::too_many_arguments)]
pub fn closed(
    pair: &Pair,
    band: &Band,
    bar: &Bar,
    did: AtZone,
    was: Action,
    interval: &str,
    out: &Path,
) -> Result<PathBuf, CardError> {
    fill::draw(
        CLOSE,
        &[(
            "/*__CLOSE__*/",
            facts::closed(pair, band, bar, did, was, interval, &bar.datetime).to_string(),
        )],
        out,
    )
}
