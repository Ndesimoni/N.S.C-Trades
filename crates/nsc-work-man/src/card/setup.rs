//! The signal card — a shape he trades, at a level he drew.
//!
//! **Rung 3, and it never says buy.** Where the stop goes has not been
//! settled, so a signal with no stop is a reading rather than a trade. The
//! card reports what printed and stops there.

use std::path::{Path, PathBuf};

use nsc_core::candle::Bar;
use nsc_core::levels::Pair;
use nsc_strategy::{Signal, reasons};
use serde_json::{Value, json};

use super::{CardError, fill};

const TEMPLATE: &str = "setup.html";

/// Draws the signal.
///
/// `bars` are the two candles the shape is made of, oldest first.
pub fn setup(
    signal: &Signal,
    pair: &Pair,
    bars: &[&Bar],
    timeframe: &str,
    stamp: &str,
    out: &Path,
) -> Result<PathBuf, CardError> {
    if bars.is_empty() {
        return Err(CardError::NothingToDraw);
    }

    fill::draw(
        TEMPLATE,
        &[(
            "/*__SETUP__*/",
            facts(signal, pair, bars, timeframe, stamp).to_string(),
        )],
        out,
    )
}

fn facts(signal: &Signal, pair: &Pair, bars: &[&Bar], timeframe: &str, stamp: &str) -> Value {
    let digits = pair.digits;

    let candles: Vec<Value> = bars
        .iter()
        .map(|bar| {
            json!({
                "open":  as_number(bar.open, digits),
                "high":  as_number(bar.high, digits),
                "low":   as_number(bar.low, digits),
                "close": as_number(bar.close, digits),
                "up":    bar.close >= bar.open,
            })
        })
        .collect();

    json!({
        "shape":     signal.shape.name(),

        // **The one sentence, written by the rules rather than by the card.**
        // If nsc-strategy cannot write it, the rules are too loose — that is a
        // test of the rules, and it belongs beside them.
        "sentence":  reasons::sentence(signal, &pair.symbol, timeframe, digits),

        "symbol":    pair.symbol,
        "timeframe": timeframe,
        "digits":    digits,
        "stamp":     stamp,

        "up":        signal.shape.is_up(),

        // **The tier, and it decides the whole look of the card.** Red and
        // "act" for the one in the zone; amber for the near miss; plain for a
        // shape with no level under it at all.
        "tier":      signal.standing.label(),
        "colour":    signal.standing.colour(),
        "solid":     signal.standing.solid(),
        "placing":   signal.standing.placing().map(|p| p.words()),
        "broke":     signal.standing.broke_out(),

        // How big the shape is, in normal candles. On the card either way —
        // at a zone it says how plainly it happened, away from one it is the
        // entire reason the message was sent.
        "reach":     as_number(signal.reach, 1),

        // **Null when there is no zone, and the template must handle that.**
        // A `Bold` card with an empty band drawn on it would look exactly like
        // a setup at a level of nought.
        "band":      signal.standing.band().map(|band| json!({
            "name":   band.timeframe.name(),
            "colour": band.timeframe.colour(),
            "price":  as_number(band.price, digits),
            "top":    as_number(band.top, digits),
            "bottom": as_number(band.bottom, digits),
        })),

        "candles": candles,
    })
}

/// Rounds to the pair's own precision and hands it over as a number.
///
/// **A number, not a string.** The card puts the separators and trailing zeros
/// on the way every other card does — hand it "4094" and gold appears as 4094
/// beside a euro shown to five decimals.
fn as_number(value: rust_decimal::Decimal, digits: u32) -> f64 {
    value
        .round_dp(digits)
        .to_string()
        .parse::<f64>()
        .unwrap_or_default()
}
