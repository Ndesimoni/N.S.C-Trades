//! The calendar card — everything today, or everything left this week.
//!
//! **A different shape from `soon.rs`, on purpose.** That card is one release
//! with its numbers on it. This is a list, so every row carries its own time
//! and the forecast is left off — eighteen rows of numbers is a spreadsheet,
//! and he is reading it on a phone.

use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use nsc_core::news::{Event, Span, away_words, printed};
use serde_json::{Value, json};

use super::{CardError, fill};

const TEMPLATE: &str = "calendar.html";

/// Header and footer. Pinned in `calendar.css`, checked by a test.
const CHROME: u32 = 118 + 3 + 8 + 46;

/// One release. Pinned as `.row { height: 47px }`.
const PER_EVENT: u32 = 48;

/// A day heading on the week's list. Pinned as `.day { height: 35px }`.
const PER_DAY: u32 = 36;

/// Draws the list he asked for.
///
/// `events` are already filtered and sorted — `nsc_core::news::within` does
/// that, so the same rule serves this and the warnings that arrive on their
/// own.
pub fn calendar(
    events: &[&Event],
    span: Span,
    now: DateTime<Utc>,
    out: &Path,
) -> Result<PathBuf, CardError> {
    fill::draw(
        TEMPLATE,
        &[
            ("/*__TALL__*/", tall(events, span).to_string()),
            ("/*__DAYS__*/", facts(events, span, now).to_string()),
        ],
        out,
    )
}

/// How tall the card is for this list.
///
/// **The week's list carries a heading per day and today's does not**, so the
/// two cannot share one number. Getting this wrong does not fail — Chrome
/// shoots a window and the overflow is simply cut off, which reads as a
/// shorter week. That is exactly how the news card lost its fourth release.
pub(super) fn tall(events: &[&Event], span: Span) -> u32 {
    let days = if span == Span::Today {
        0
    } else {
        how_many_days(events)
    };

    // An empty list still says so, and needs a line's worth of room to do it.
    let rows = events.len().max(1) as u32;

    CHROME + PER_EVENT * rows + PER_DAY * days
}

/// How many day headings the week's list will grow.
fn how_many_days(events: &[&Event]) -> u32 {
    let mut days = 0;
    let mut last = None;

    for event in events {
        let day = event.at.date_naive();

        if last != Some(day) {
            days += 1;
            last = Some(day);
        }
    }

    days
}

fn facts(events: &[&Event], span: Span, now: DateTime<Utc>) -> Value {
    let rows: Vec<Value> = events
        .iter()
        .map(|event| {
            json!({
                "day":      event.at.format("%A %-d %B").to_string(),
                "time":     event.at.format("%H:%M").to_string(),
                "colour":   event.impact.colour(),
                "currency": event.currency,
                "title":    event.title,
                "printed":  printed(event, now),

                // **The card reads no clock.** It is handed the words, so
                // they can be checked without a browser.
                "away":     away_words(event, now),
            })
        })
        .collect();

    let gone = events.iter().filter(|event| printed(event, now)).count();

    json!({
        "heading":  if span == Span::Today { "Today" } else { "This week" },
        "days":     span != Span::Today,
        "count":    events.len(),
        "gone":     gone,
        "upcoming": events.len() - gone,
        "stamp":    now.format("%-d %b · %H:%M UTC").to_string(),
        "events":   rows,
    })
}
