//! **Everything the bot saw, as it saw it.**
//!
//! ## Why it is stored rather than worked out later
//!
//! Recalculated against updated chart-reading code, it trains a model on
//! inputs the live bot never produced — and **nothing detects that.** Both
//! sides keep working; only the scores are wrong. It is the same family of
//! mistake as reading a candle the market had not printed: it does not error,
//! it makes results look better.
//!
//! So: written once, at the moment of the decision, and never touched again.
//!
//! ## The two sides have to match exactly
//!
//! A signal and a refusal carry the same keys. They are the two halves of one
//! dataset — what to take and what not to — and a different shape on each side
//! makes them unusable together, which is the only use either has.
//!
//! Anything a side does not know is simply absent, never guessed.

use nsc_core::candle::Bar;
use nsc_core::levels::Band;
use rust_decimal::Decimal;
use serde_json::{Value, json};

/// **What shape the saved features are in.**
///
/// Goes up when a key is added, removed or means something new. Old rows stay
/// valid at their old shape and the training script filters on it — which is
/// what lets the shape change at all without throwing away the history.
pub(in crate::watch::closes) const FEATURES_VERSION: i16 = 1;

/// What the bot saw about the candle and the market around it.
///
/// **Every row gets this much**, signal or refusal, because it is what both
/// sides have in common: a candle, and how big a normal one was at the time.
pub(in crate::watch::closes) fn of_the_candle(bar: &Bar, normal: Decimal) -> Value {
    json!({
        "open":   bar.open.to_string(),
        "high":   bar.high.to_string(),
        "low":    bar.low.to_string(),
        "close":  bar.close.to_string(),

        // **The normal candle at that moment, not today's.** It is the unit
        // every distance in this project is measured in, so a row without it
        // cannot be compared to any other row.
        "normal": normal.to_string(),

        // Worked out here rather than left to the reader: how far the candle
        // travelled, in normal candles. The one number that says whether
        // anything actually happened.
        "range_in_normals": in_normals(bar.high - bar.low, normal),
    })
}

/// The same, with the level it printed at added.
///
/// **Only when there IS one.** A shape with no level under it is the most
/// interesting refusal in the table, and inventing a band for it would put a
/// number in the record that never existed.
pub(in crate::watch::closes) fn with_the_band(
    bar: &Bar,
    normal: Decimal,
    band: &Band,
    reach: Decimal,
) -> Value {
    let mut seen = of_the_candle(bar, normal);

    if let Value::Object(fields) = &mut seen {
        fields.insert("band_price".into(), json!(band.price.to_string()));
        fields.insert("band_top".into(), json!(band.top.to_string()));
        fields.insert("band_bottom".into(), json!(band.bottom.to_string()));
        fields.insert(
            "band_thickness_in_normals".into(),
            json!(in_normals(band.thickness(), normal)),
        );
        fields.insert("shape_reach".into(), json!(reach.to_string()));
    }

    seen
}

/// A distance as a share of a normal candle, or `null` when there is no
/// normal candle to divide by.
///
/// **Never zero as a stand-in.** Zero is a real answer — a candle that did not
/// move — and using it for "unknown" makes the two impossible to tell apart in
/// a table nobody can re-derive.
fn in_normals(distance: Decimal, normal: Decimal) -> Value {
    if normal <= Decimal::ZERO {
        return Value::Null;
    }

    json!((distance / normal).round_dp(3).to_string())
}
