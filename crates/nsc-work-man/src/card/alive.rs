//! The heartbeat card — what the bot is watching on a day nothing happened.
//!
//! **The only card that exists to say nothing is wrong.** Everything else here
//! is sent because something occurred.

use std::path::{Path, PathBuf};

use rust_decimal::Decimal;
use serde_json::{Value, json};

use super::{CardError, fill};
use nsc_core::levels::{Band, Pair};

const TEMPLATE: &str = "heartbeat.html";

/// The card's height, worked out from how many pairs are on it.
///
/// A fixed number would either clip the last row or leave a field of white
/// under four of them, and he may have two pairs one week and eight the next.
/// Header, the list's own top padding, and the footer. Measured, not guessed.
const CHROME: u32 = 107 + 8 + 49;

/// One row: 14 top, 14 bottom, about 24 of content, and a hairline.
const PER_PAIR: u32 = 53;

/// One pair, as the heartbeat sees it.
pub struct Alive<'a> {
    pub pair: &'a Pair,
    pub bands: Vec<Band>,

    /// Where price was when we last heard. `None` before the first price
    /// arrives, and the card shows a dash rather than inventing a distance.
    pub price: Option<Decimal>,
}

/// Draws the heartbeat.
pub fn heartbeat(
    watching: &[Alive<'_>],
    quiet_for: &str,
    stamp: &str,
    out: &Path,
) -> Result<PathBuf, CardError> {
    let tall = CHROME + PER_PAIR * watching.len().max(1) as u32;

    fill::draw(
        TEMPLATE,
        &[
            ("/*__TALL__*/", tall.to_string()),
            (
                "/*__BEAT__*/",
                facts(watching, quiet_for, stamp).to_string(),
            ),
        ],
        out,
    )
}

fn facts(watching: &[Alive<'_>], quiet_for: &str, stamp: &str) -> Value {
    let pairs: Vec<Value> = watching
        .iter()
        .map(|seen| {
            json!({
                "symbol": seen.pair.symbol,
                "digits": seen.pair.digits,
                "zones":  seen.bands.iter().map(|b| b.timeframe.colour()).collect::<Vec<_>>(),
                "near":   nearest(seen),
            })
        })
        .collect();

    json!({ "stamp": stamp, "quiet_for": quiet_for, "pairs": pairs })
}

/// The zone price is closest to, and how far off it is.
///
/// **The thing actually worth knowing on a quiet morning.** "Still running" is
/// the point of the message, but the zone price is nearest is the one likely
/// to speak today.
fn nearest(seen: &Alive<'_>) -> Value {
    let Some(price) = seen.price else {
        return Value::Null;
    };

    let closest = seen
        .bands
        .iter()
        .min_by_key(|band| (band.price - price).abs());

    match closest {
        None => Value::Null,
        Some(band) => json!({
            "timeframe": band.timeframe.name(),
            "price":     as_number(band.price, seen.pair.digits),
            "away":      as_number((band.price - price).abs(), seen.pair.digits),
        }),
    }
}

/// Rounds to the pair's own precision and hands it over as a number.
///
/// **A number, not a string.** The card puts the thousands separator and the
/// trailing zeros on, the same way every other card does — hand it "4094" and
/// gold appears as 4094 next to a euro shown to five decimals.
fn as_number(value: Decimal, digits: u32) -> f64 {
    value
        .round_dp(digits)
        .to_string()
        .parse::<f64>()
        .unwrap_or_default()
}
