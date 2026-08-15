//! Turning candles into the numbers a template can read.
//!
//! Rust hands over facts; the template decides how they look. Nothing here
//! knows about colours, fonts or layout, and nothing in the template works out
//! a price.

use super::CardError;
use chrono::TimeDelta;
use rust_decimal::Decimal;
use serde_json::{Value, json};

use nsc_core::candle::Bar;
use nsc_core::levels::{self, Band, Nearness, Pair};
use nsc_core::settings::{INTERVAL_MINUTES, timeframe_name, unit_for};

/// The one candle the card is about.
pub fn one(bar: &Bar, symbol: &str, interval: &str, digits: u32) -> Result<Value, CardError> {
    // When it FINISHED, not when it opened. "20:00" is the moment this became
    // true, and that is the number a trader looks for.
    //
    // A weekly or daily candle is stamped with a bare DATE, not a time — so
    // `opened_at` refuses it rather than guessing, and there is nothing to add
    // an hour to. The date is then the whole truth and gets shown as it is.
    let stamp = match bar.opened_at() {
        Ok(opened) => {
            let one_step = TimeDelta::try_minutes(INTERVAL_MINUTES)
                .unwrap_or_else(|| TimeDelta::try_minutes(60).unwrap_or_default());

            (opened + one_step).format("%-d %b · %H:%M UTC").to_string()
        }
        Err(_) => bar.datetime.clone(),
    };

    Ok(json!({
        "symbol":   symbol,
        "interval": timeframe_name(interval),
        "stamp":    stamp,
        "unit":     unit_for(symbol),
        "digits":   digits,
        "open":     rounded(bar.open, digits),
        "high":     rounded(bar.high, digits),
        "low":      rounded(bar.low, digits),
        "close":    rounded(bar.close, digits),
    }))
}

/// Every candle, for the chart. Oldest first, because that is how a chart reads.
pub fn all(bars: &[&Bar], digits: u32) -> Value {
    let rows: Vec<Value> = bars
        .iter()
        .map(|bar| {
            json!({
                "at":    bar.datetime.get(5..16).unwrap_or(&bar.datetime),
                "open":  rounded(bar.open, digits),
                "high":  rounded(bar.high, digits),
                "low":   rounded(bar.low, digits),
                "close": rounded(bar.close, digits),
            })
        })
        .collect();

    json!(rows)
}

/// Everything the alert card says out loud.
///
/// **Every distance is worked out here.** The template places things on the
/// card and reads no prices of its own — the same rule the chart cards follow,
/// for the same reason: a number worked out in two places drifts in one.
pub fn alert(
    pair: &Pair,
    band: &Band,
    near: Nearness,
    price: Decimal,
    reach: Decimal,
    stamp: &str,
) -> Value {
    let digits = pair.digits;

    json!({
        "symbol":    pair.symbol,
        "state":     if near == Nearness::Inside { "inside" } else { "approaching" },
        "timeframe": band.timeframe.name(),
        "colour":    band.timeframe.colour(),
        "note":      levels::note(near),
        "stamp":     stamp,
        "unit":      unit_for(&pair.symbol),
        "digits":    digits,
        "price":     rounded(price, digits),
        "level":     rounded(band.price, digits),
        "top":       rounded(band.top, digits),
        "bottom":    rounded(band.bottom, digits),
        "gap":       rounded(levels::gap(band, price), digits),
        "from_line": rounded((price - band.price).abs(), digits),
        "reach":     rounded(reach, digits),
    })
}

/// Rounds to the instrument's own precision, then hands it over as a number.
///
/// **Rounding happens here, on the way out, and nowhere else.** Gold is quoted
/// to two decimals and the feed sends five. Let all five through and a signal
/// reads like a debug dump.
///
/// It goes to a float only at this last step, because JSON has no other kind
/// of number. Every calculation before this point was Decimal.
fn rounded(value: Decimal, digits: u32) -> f64 {
    value
        .round_dp(digits)
        .to_string()
        .parse::<f64>()
        .unwrap_or_default()
}

/// The levels he drew, as bands the template can draw.
///
/// **His colours are a specification, not a design choice** — black weekly,
/// blue daily, yellow 4-hour. Drawing them all one colour was done once
/// already and the chart looked nothing like his.
pub fn levels(bands: &[Band], digits: u32) -> Value {
    let rows: Vec<Value> = bands
        .iter()
        .map(|band| {
            json!({
                "timeframe": band.timeframe.name(),
                "colour":    band.timeframe.colour(),
                "price":     rounded(band.price, digits),
                "top":       rounded(band.top, digits),
                "bottom":    rounded(band.bottom, digits),
            })
        })
        .collect();

    json!(rows)
}
