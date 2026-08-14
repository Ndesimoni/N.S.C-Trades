//! Turning candles into the numbers a template can read.
//!
//! Rust hands over facts; the template decides how they look. Nothing here
//! knows about colours, fonts or layout, and nothing in the template works out
//! a price.

use anyhow::{Context, Result};
use chrono::TimeDelta;
use rust_decimal::Decimal;
use serde_json::{Value, json};

use crate::candle::Bar;
use crate::settings::{INTERVAL, INTERVAL_MINUTES, SYMBOL, timeframe_name, unit_for};

/// The one candle the card is about.
pub fn one(bar: &Bar, digits: u32) -> Result<Value> {
    let one_step = TimeDelta::try_minutes(INTERVAL_MINUTES)
        .context("the interval is not a length of time chrono can hold")?;

    // When it FINISHED, not when it opened. "20:00" is the moment this became
    // true, and that is the number a trader looks for.
    let closed_at = bar.opened_at()? + one_step;

    Ok(json!({
        "symbol":   SYMBOL,
        "interval": timeframe_name(INTERVAL),
        "stamp":    closed_at.format("%-d %b · %H:%M UTC").to_string(),
        "unit":     unit_for(SYMBOL),
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
fn rounded(value: Decimal, digits: u32) -> f64 {
    value
        .round_dp(digits)
        .to_string()
        .parse::<f64>()
        .unwrap_or_default()
}
