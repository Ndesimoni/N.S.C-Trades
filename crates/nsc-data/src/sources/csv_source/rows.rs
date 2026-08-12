//! Turning one row into one candle.

use std::path::Path;
use std::str::FromStr;

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use nsc_core::candle::Candle;
use nsc_core::price::Price;
use rust_decimal::Decimal;

use super::columns::Columns;
use crate::error::DataError;

/// Every timestamp shape these exports come in.
///
/// Tried in order. All of them are read as UTC unless the text carries its own
/// offset — see the module docs for why guessing a timezone is not an option.
const SHAPES: [&str; 6] = [
    "%Y-%m-%d %H:%M:%S",
    "%Y-%m-%dT%H:%M:%S",
    "%Y.%m.%d %H:%M:%S",
    "%Y.%m.%d %H:%M",
    "%Y/%m/%d %H:%M:%S",
    "%d/%m/%Y %H:%M:%S",
];

/// Reads one row.
pub(super) fn to_candle(
    row: &[String],
    at: Columns,
    line: usize,
    path: &Path,
) -> Result<Candle, DataError> {
    let field = |which: usize, what: &str| -> Result<&str, DataError> {
        row.get(which)
            .map(|text| text.trim())
            .ok_or_else(|| DataError::BadRow {
                path: path.to_path_buf(),
                line,
                detail: format!("no {what} in this row"),
            })
    };

    let price = |which: usize, what: &str| -> Result<Price, DataError> {
        let text = field(which, what)?;

        Decimal::from_str(text)
            .map(Price::new)
            .map_err(|_| DataError::BadRow {
                path: path.to_path_buf(),
                line,
                detail: format!("'{text}' is not a {what} price"),
            })
    };

    Ok(Candle::new(
        to_time(field(at.time, "time")?, line, path)?,
        price(at.open, "open")?,
        price(at.high, "high")?,
        price(at.low, "low")?,
        price(at.close, "close")?,
        // Cash forex and CFDs have no traded volume, so nothing in this
        // project may depend on it. Any column in the file is ignored rather
        // than half-supported.
        None,
        // A file holds finished candles. The one exception cannot be seen from
        // here — see the module docs.
        true,
    )?)
}

/// Reads a timestamp, as UTC.
fn to_time(text: &str, line: usize, path: &Path) -> Result<DateTime<Utc>, DataError> {
    // An offset in the text is honoured — it is the one case where the file
    // actually says what it means.
    if let Ok(with_offset) = DateTime::parse_from_rfc3339(text) {
        return Ok(with_offset.with_timezone(&Utc));
    }

    for shape in SHAPES {
        if let Ok(naive) = NaiveDateTime::parse_from_str(text, shape) {
            return Ok(naive.and_utc());
        }
    }

    // A daily or weekly export has no time of day. Midnight is what the file
    // means by it, and where the trading day actually starts is applied later
    // by nsc-core::timeframe rather than guessed at here.
    for shape in ["%Y-%m-%d", "%Y.%m.%d", "%d/%m/%Y"] {
        if let Ok(day) = NaiveDate::parse_from_str(text, shape) {
            return Ok(day.and_hms_opt(0, 0, 0).unwrap_or_default().and_utc());
        }
    }

    Err(DataError::BadTimestamp {
        path: path.to_path_buf(),
        line,
        text: text.to_string(),
    })
}
