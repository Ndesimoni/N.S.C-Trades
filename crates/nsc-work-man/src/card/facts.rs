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
use nsc_core::candle::{minutes_for, timeframe_name};
use nsc_core::levels::Band;

/// The one candle the card is about.
pub fn one(bar: &Bar, symbol: &str, interval: &str, digits: u32) -> Result<Value, CardError> {
    // When it FINISHED, not when it opened. "20:00" is the moment this became
    // true, and that is the number a trader looks for.
    //
    // **HOW LONG THE CANDLE IS COMES FROM THE INTERVAL**, never from a
    // constant. It was a hardcoded hour here, which would have stamped every
    // 4-hour card an hour after its open instead of four — a picture with the
    // wrong time on it is believed exactly like a wrong number.
    //
    // A weekly or daily candle is stamped with a bare DATE, not a time, and
    // `minutes_for` refuses both because their length is not a fixed run of
    // minutes. The date is then the whole truth and is shown as it is.
    let stamp = match (bar.opened_at(), minutes_for(interval)) {
        (Ok(opened), Some(minutes)) => {
            let one_step = TimeDelta::try_minutes(minutes).unwrap_or_default();

            (opened + one_step).format("%-d %b · %H:%M UTC").to_string()
        }
        _ => bar.datetime.clone(),
    };

    Ok(json!({
        "symbol":   symbol,
        "interval": timeframe_name(interval),
        "stamp":    stamp,
        "unit":     nsc_core::levels::unit_for(symbol),
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

/// Rounds to the instrument's own precision, then hands it over as a number.
///
/// **Rounding happens here, on the way out, and nowhere else.** Gold is quoted
/// to two decimals and the feed sends five. Let all five through and a signal
/// reads like a debug dump.
///
/// It goes to a float only at this last step, because JSON has no other kind
/// of number. Every calculation before this point was Decimal.
pub(super) fn rounded(value: Decimal, digits: u32) -> f64 {
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
