//! The calendar's own shape, turned into events the bot understands.

use chrono::{DateTime, Utc};
use nsc_core::news::{Event, Impact};
use serde::Deserialize;

use super::CalendarError;

/// One row exactly as the feed writes it.
///
/// **Every field defaults.** A speech has no forecast and no previous, and
/// they arrive as empty strings rather than as missing keys — but defaulting
/// costs nothing and means a field they drop one day does not take the whole
/// week's calendar with it.
#[derive(Debug, Deserialize)]
struct Row {
    #[serde(default)]
    title: String,

    /// **The feed calls this `country` and it holds a CURRENCY** — `USD`,
    /// `AUD`, and `All` for something belonging to no single one. The name is
    /// theirs; the meaning is what it is used as.
    #[serde(default)]
    country: String,

    #[serde(default)]
    date: String,

    #[serde(default)]
    impact: String,

    #[serde(default)]
    forecast: String,

    #[serde(default)]
    previous: String,
}

/// What came back, and what could not be read.
#[derive(Debug)]
pub struct Parsed {
    pub events: Vec<Event>,

    /// Rows whose time made no sense.
    ///
    /// **Counted rather than thrown away silently.** One unreadable row must
    /// not cost the whole week's calendar — but a row quietly vanishing is how
    /// a feed change goes unnoticed for a month, so the caller is told how
    /// many and can say so.
    pub unreadable: usize,
}

/// Turns the file into events.
pub fn read(body: &str) -> Result<Parsed, CalendarError> {
    let rows: Vec<Row> = serde_json::from_str(body)
        .map_err(|trouble| CalendarError::NotEvents(trouble.to_string()))?;

    let mut events = Vec::with_capacity(rows.len());
    let mut unreadable = 0;

    for row in rows {
        match when(&row.date) {
            Some(at) => events.push(Event {
                title: row.title,
                currency: row.country,
                at,
                impact: Impact::from_feed(&row.impact),
                forecast: row.forecast,
                previous: row.previous,
            }),
            None => unreadable += 1,
        }
    }

    Ok(Parsed { events, unreadable })
}

/// Reads the feed's stamp and converts it once, here, to UTC.
///
/// **They send a New York offset** — `2026-08-25T10:00:00-04:00`. Kept as
/// written it would be an hour out for half the year and nothing would error,
/// which is the same trap the daily candle boundary set. Everything after this
/// line is UTC, exactly like candles.
fn when(stamp: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(stamp.trim())
        .ok()
        .map(|moment| moment.with_timezone(&Utc))
}
