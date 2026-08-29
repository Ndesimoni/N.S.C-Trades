//! Cards that grow a row per thing, and the height Chrome is asked for.
//!
//! **This is the test that was missing.** The news card went out headed
//! "4 releases" with three of them on it, and nothing failed — Chrome shoots a
//! **window**, not a page, so anything past the height it was given is simply
//! cut off. It reads as a card with fewer rows, not as a bug.
//!
//! The row constant had been copied from the heartbeat, whose rows are one
//! line where these are two.
//!
//! Both cards pin their parts in CSS rather than letting content decide, and
//! these read those files to check Rust still agrees.

use chrono::{DateTime, Duration, TimeZone, Utc};
use nsc_core::news::{Event, Impact, Span};

use super::super::listing::tall as calendar_tall;
use super::super::soon::tall as news_tall;

/// The header's rule and the list's top padding. Shared by both cards.
const RULE: u32 = 3;
const PADDING: u32 = 8;
const HAIRLINE: u32 = 1;

// ── reading a pinned height out of a stylesheet ────────────────────────────

/// The height pinned on one selector.
///
/// **Looks inside that block only.** A matcher that swept the whole file for
/// `height:` also found `line-height`, `min-height` and the little colour chip
/// inside a row — none of which are part of how tall the card is.
fn pinned(css: &str, selector: &str) -> u32 {
    let at = css
        .find(selector)
        .unwrap_or_else(|| panic!("{selector} is not in the stylesheet any more"));

    let block = &css[at..];
    let end = block
        .find('}')
        .unwrap_or_else(|| panic!("{selector} has no closing brace"));

    let block = &block[..end];

    let found = block
        .match_indices("height:")
        .find(|(here, _)| !block[..*here].ends_with('-'))
        .unwrap_or_else(|| panic!("{selector} no longer pins a height"));

    let after = &block[found.0 + "height:".len()..];
    let digits: String = after.chars().take_while(char::is_ascii_digit).collect();

    digits
        .parse()
        .unwrap_or_else(|_| panic!("{selector} pins a height that is not a number"))
}

fn stylesheet(name: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/card")
        .join(name);

    std::fs::read_to_string(&path)
        .unwrap_or_else(|trouble| panic!("could not read {}: {trouble}", path.display()))
}

// ── the news card — one release, a row per line of it ──────────────────────

/// **The one that would have caught it.** Four releases must be given room for
/// four rows.
#[test]
fn every_release_the_news_card_promises_has_room() {
    let css = stylesheet("news.css");
    let chrome = pinned(&css, ".top{") + RULE + PADDING + pinned(&css, ".foot{");
    let row = pinned(&css, ".row{") + HAIRLINE;

    for releases in 1..=6 {
        assert_eq!(
            news_tall(releases as usize),
            chrome + row * releases,
            "a news card of {releases} releases is the wrong height — the last \
             one would be cut off and nothing would fail"
        );
    }
}

#[test]
fn no_releases_is_just_the_news_cards_chrome() {
    let css = stylesheet("news.css");
    let chrome = pinned(&css, ".top{") + RULE + PADDING + pinned(&css, ".foot{");

    assert_eq!(news_tall(0), chrome);
}

// ── the calendar card — a row per release AND a heading per day ────────────

/// **Today has no day headings and the week does.** One number cannot serve
/// both, and the week is the one that would clip: it is always the longer.
#[test]
fn the_week_leaves_room_for_its_day_headings() {
    let css = stylesheet("calendar.css");
    let chrome = pinned(&css, ".top{") + RULE + PADDING + pinned(&css, ".foot{");
    let row = pinned(&css, ".row{") + HAIRLINE;
    let day = pinned(&css, ".day{") + HAIRLINE;

    let events = three_days();
    let listed: Vec<&Event> = events.iter().collect();

    assert_eq!(
        calendar_tall(&listed, Span::Week),
        chrome + row * 3 + day * 3,
        "three releases across three days need three rows AND three headings"
    );

    assert_eq!(
        calendar_tall(&listed, Span::Today),
        chrome + row * 3,
        "today is one day, so it grows no headings at all"
    );
}

/// Several releases on one day share one heading, not one each.
#[test]
fn a_day_gets_one_heading_however_many_land_on_it() {
    let css = stylesheet("calendar.css");
    let chrome = pinned(&css, ".top{") + RULE + PADDING + pinned(&css, ".foot{");
    let row = pinned(&css, ".row{") + HAIRLINE;
    let day = pinned(&css, ".day{") + HAIRLINE;

    let events = same_day();
    let listed: Vec<&Event> = events.iter().collect();

    assert_eq!(calendar_tall(&listed, Span::Week), chrome + row * 3 + day);
}

/// **An empty list still needs a line to say so on.** Nought rows draws a
/// header, a footer and a sliver of nothing between them.
#[test]
fn nothing_on_the_calendar_still_has_room_to_say_so() {
    let css = stylesheet("calendar.css");
    let chrome = pinned(&css, ".top{") + RULE + PADDING + pinned(&css, ".foot{");
    let row = pinned(&css, ".row{") + HAIRLINE;

    assert_eq!(calendar_tall(&[], Span::Today), chrome + row);
    assert_eq!(calendar_tall(&[], Span::Week), chrome + row);
}

// ── events to count ────────────────────────────────────────────────────────

fn at(days: i64, hours: i64) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 8, 25, 0, 0, 0)
        .single()
        .expect("25 August 2026 is a real day")
        + Duration::days(days)
        + Duration::hours(hours)
}

fn one(title: &str, when: DateTime<Utc>) -> Event {
    Event {
        title: title.into(),
        currency: "USD".into(),
        at: when,
        impact: Impact::High,
        forecast: String::new(),
        previous: String::new(),
    }
}

fn three_days() -> Vec<Event> {
    vec![
        one("Core PCE", at(0, 9)),
        one("GDP", at(1, 9)),
        one("Fed Speaks", at(2, 9)),
    ]
}

fn same_day() -> Vec<Event> {
    vec![
        one("CPI m/m", at(0, 9)),
        one("CPI y/y", at(0, 9)),
        one("Retail Sales", at(0, 14)),
    ]
}

