//! Tests for reading the calendar.
//!
//! **No network in any of them.** The one thing worth guarding here is the
//! refusal that arrives dressed as a success, and that is a string test.

use nsc_core::error::{Answer, Knows};
use nsc_core::news::Impact;

use super::{CalendarError, feed, read};

/// Two real rows, copied from the file on 24 August 2026.
const REAL: &str = r#"[
  {"title":"CB Consumer Confidence","country":"USD","date":"2026-08-25T10:00:00-04:00",
   "impact":"Medium","forecast":"90.3","previous":"90.8"},
  {"title":"Fed Chairman Warsh Speaks","country":"USD","date":"2026-08-28T10:00:00-04:00",
   "impact":"High","forecast":"","previous":""}
]"#;

/// What comes back over the download limit — a web page, under a normal 200.
const REFUSAL: &str = "<!DOCTYPE html><html><head><title>Request Denied</title></head>\
                       <body>You've exceeded the limit for Calendar Export requests.</body></html>";

#[test]
fn reads_the_real_file() {
    let parsed = read(REAL).expect("two real rows should read");

    assert_eq!(parsed.events.len(), 2);
    assert_eq!(parsed.unreadable, 0);
    assert_eq!(parsed.events[0].title, "CB Consumer Confidence");
    assert_eq!(parsed.events[0].impact, Impact::Medium);
    assert_eq!(parsed.events[1].impact, Impact::High);
}

/// **The field is called `country` and it holds a currency.** Reading it as a
/// country would mean matching "United States" against a pair called USD/CAD.
#[test]
fn country_is_really_a_currency() {
    let parsed = read(REAL).expect("should read");

    assert_eq!(parsed.events[0].currency, "USD");
}

/// **They stamp it with a New York offset.** Kept as written it would be an
/// hour out for half the year and nothing would error — the same trap the
/// daily candle boundary set.
#[test]
fn the_new_york_stamp_becomes_utc() {
    let parsed = read(REAL).expect("should read");

    // 10:00 at -04:00 is 14:00 UTC.
    assert_eq!(
        parsed.events[0].at.to_rfc3339(),
        "2026-08-25T14:00:00+00:00"
    );
}

#[test]
fn a_speech_carries_no_numbers() {
    let parsed = read(REAL).expect("should read");

    assert!(!parsed.events[1].has_numbers());
}

/// One bad row must not cost the whole week — but it must not vanish either.
#[test]
fn an_unreadable_time_is_counted_not_thrown_away() {
    let mixed = r#"[
      {"title":"Good","country":"USD","date":"2026-08-25T10:00:00-04:00","impact":"High"},
      {"title":"Bad","country":"USD","date":"next tuesday","impact":"High"}
    ]"#;

    let parsed = read(mixed).expect("the good row should still read");

    assert_eq!(parsed.events.len(), 1);
    assert_eq!(parsed.unreadable, 1);
}

#[test]
fn missing_fields_do_not_lose_the_file() {
    let sparse = r#"[{"title":"Bank Holiday","country":"All",
                      "date":"2026-08-31T00:00:00-04:00","impact":"Holiday"}]"#;

    let parsed = read(sparse).expect("a row with no numbers should read");

    assert_eq!(parsed.events.len(), 1);
    assert_eq!(parsed.events[0].impact, Impact::Holiday);
    assert!(!parsed.events[0].has_numbers());
}

// ── the refusal that looks like a success ──────────────────────────────────

#[test]
fn a_web_page_is_not_a_calendar() {
    assert!(!feed::looks_like_json(REFUSAL));
    assert!(feed::looks_like_json(REAL));
    assert!(feed::looks_like_json("\n\n  [ ]"));
}

/// **The rate limit must be worth another go, and the shape change must not
/// be.** Fold them together and one busy afternoon retires the news watcher
/// for good, or a changed file gets retried forever looking like a dead line.
#[test]
fn a_polite_refusal_is_worth_waiting_out() {
    assert_eq!(CalendarError::NotJson.answer(), Answer::in_a_while());
    assert!(CalendarError::NotJson.answer().worth_trying_again());
}

#[test]
fn a_changed_file_is_not_worth_retrying() {
    let changed = CalendarError::NotEvents("expected an array".into());

    assert_eq!(changed.answer(), Answer::GiveUp);
    assert!(!changed.answer().worth_trying_again());
}

#[test]
fn a_dropped_line_is_worth_another_go_soon() {
    let dropped = CalendarError::Unreachable("connection reset".into());

    assert_eq!(dropped.answer(), Answer::soon());
}

#[test]
fn something_that_is_not_json_at_all_is_a_shape_change() {
    let trouble = read("{\"not\":\"an array\"}").expect_err("an object is not a calendar");

    assert!(matches!(trouble, CalendarError::NotEvents(_)));
}
