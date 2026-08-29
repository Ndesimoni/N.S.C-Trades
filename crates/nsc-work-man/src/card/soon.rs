//! The news card — what is about to print, and how hard it usually hits.
//!
//! **One card per release, not one per line.** Three Australian CPI numbers
//! land in the same second; `nsc_core::news::together` groups them and this
//! draws the group. Sent apart they would buzz his phone three times for what
//! he reads as one event.

use std::path::{Path, PathBuf};

use nsc_core::news::{Event, Impact};
use serde_json::{Value, json};

use super::{CardError, fill};

const TEMPLATE: &str = "news.html";

/// The card's height, worked out from how many events are on it.
///
/// Header, the list's own padding, and the footer. **Every one of these is
/// pinned in `news.css` rather than left to the content**, so the two numbers
/// cannot drift — change one there and change its twin here.
///
/// It drifted once already: rows sized from the heartbeat's constant, which
/// is a single line where these are two. The card went out headed
/// "4 releases" with three on it, and nothing failed.
const CHROME: u32 = 132 + 3 + 8 + 46;

/// One row and its hairline. Pinned as `.row { height: 71px }`.
const PER_EVENT: u32 = 72;

/// Draws the card for one release.
///
/// `minutes` is how long until it prints — already worked out, because the
/// card is told everything it says out loud and reads no clock of its own.
pub fn coming(
    events: &[&Event],
    minutes: i64,
    when: &str,
    stamp: &str,
    out: &Path,
) -> Result<PathBuf, CardError> {
    if events.is_empty() {
        return Err(CardError::NothingToDraw);
    }

    fill::draw(
        TEMPLATE,
        &[
            ("/*__TALL__*/", tall(events.len()).to_string()),
            (
                "/*__NEWS__*/",
                facts(events, minutes, when, stamp).to_string(),
            ),
        ],
        out,
    )
}

/// How tall the card is for this many releases.
///
/// **Its twin is `news.css`**, where every one of these is pinned rather than
/// left to the content. A test reads that file and checks the two still
/// agree — see `card/tests/growing.rs`.
pub(super) fn tall(events: usize) -> u32 {
    CHROME + PER_EVENT * events as u32
}

fn facts(events: &[&Event], minutes: i64, when: &str, stamp: &str) -> Value {
    let rows: Vec<Value> = events
        .iter()
        .map(|event| {
            json!({
                "impact":   event.impact.name(),
                "colour":   event.impact.colour(),
                "currency": event.currency,
                "title":    event.title,
                "forecast": dash(&event.forecast),
                "previous": dash(&event.previous),
                "numbers":  event.has_numbers(),
            })
        })
        .collect();

    json!({
        "minutes": minutes,
        "when":    when,
        "stamp":   stamp,
        "worst":   worst(events).colour(),
        "count":   events.len(),
        "events":  rows,
    })
}

/// The heaviest rating in the group, which is what colours the header.
///
/// **A card carrying one high and two mediums is a high-impact card.** Taking
/// the first event's rating instead would colour it by whatever order the feed
/// happened to list them in.
fn worst(events: &[&Event]) -> Impact {
    if events.iter().any(|event| event.impact == Impact::High) {
        return Impact::High;
    }

    if events.iter().any(|event| event.impact == Impact::Medium) {
        return Impact::Medium;
    }

    events.first().map_or(Impact::Unknown, |event| event.impact)
}

/// An em dash where there is no number.
///
/// A speech has no forecast. An empty cell reads as a card that failed to
/// fill in; a dash reads as "there is no number here", which is the truth.
fn dash(value: &str) -> String {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        "—".to_string()
    } else {
        trimmed.to_string()
    }
}
