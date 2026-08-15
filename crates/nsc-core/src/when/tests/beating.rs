//! The heartbeat — one line, only on a day that said nothing else.

use super::super::{Allowed, allowed, beat_due, beat_words};
use super::support::{rules, utc};

#[test]
fn the_heartbeat_is_due_at_the_first_seven_after_the_session_opened() {
    let rules = rules();

    assert!(
        !beat_due(utc("2026-08-18T22:00:00Z"), None, None, &rules),
        "an hour into the session, far too early"
    );
    assert!(
        !beat_due(utc("2026-08-19T06:59:00Z"), None, None, &rules),
        "a minute short"
    );
    assert!(
        beat_due(utc("2026-08-19T07:00:00Z"), None, None, &rules),
        "07:00 the next morning"
    );
}

#[test]
fn a_day_that_already_said_something_gets_no_heartbeat() {
    let rules = rules();
    let now = utc("2026-08-19T07:30:00Z");

    // An alert at 02:00, well inside this session.
    assert!(!beat_due(
        now,
        Some(utc("2026-08-19T02:00:00Z")),
        None,
        &rules
    ));

    // But one from BEFORE the session opened does not count — that was
    // yesterday's news, and today has been silent.
    assert!(beat_due(
        now,
        Some(utc("2026-08-18T14:00:00Z")),
        None,
        &rules
    ));
}

#[test]
fn the_heartbeat_goes_out_once_a_session() {
    let rules = rules();
    let sent = utc("2026-08-19T07:00:00Z");

    assert!(!beat_due(
        utc("2026-08-19T07:10:00Z"),
        None,
        Some(sent),
        &rules
    ));
    assert!(!beat_due(
        utc("2026-08-19T16:00:00Z"),
        None,
        Some(sent),
        &rules
    ));

    // The next session is a different day, and it is due again.
    assert!(beat_due(
        utc("2026-08-20T07:00:00Z"),
        None,
        Some(sent),
        &rules
    ));
}

#[test]
fn a_silent_monday_still_gets_its_heartbeat() {
    let rules = rules();
    let monday = utc("2026-08-17T07:00:00Z");

    assert_eq!(allowed(monday, &rules), Allowed::Silence);
    assert!(beat_due(monday, None, None, &rules));
}

#[test]
fn the_heartbeat_has_no_stray_indentation_in_it() {
    let words = beat_words(4, 16);

    for line in words.lines() {
        assert_eq!(line, line.trim_start(), "line begins with space: {line:?}");
    }

    assert!(words.contains("4 pairs · 16 zones"));
    assert!(
        beat_words(1, 1).contains("1 pair · 1 zone"),
        "no stray plurals"
    );
}
