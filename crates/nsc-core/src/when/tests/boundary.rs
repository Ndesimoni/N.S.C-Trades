//! Which trading day a moment belongs to.
//!
//! **The only hard thing in this folder.** Sunday evening is already Monday's
//! session, and 17:00 New York is not a fixed UTC time.

use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use chrono_tz::America::New_York;

use super::super::{Allowed, allowed, trading_day};
use super::support::{rules, utc};

#[test]
fn sunday_evening_belongs_to_mondays_session() {
    // Sunday 17 August 2026, 22:00 UTC — 18:00 in New York, past the open.
    let at = utc("2026-08-16T22:00:00Z");

    assert_eq!(at.weekday(), Weekday::Sun, "the calendar says Sunday");
    assert_eq!(
        trading_day(at, &rules()),
        Weekday::Mon,
        "the market says Monday"
    );
    assert_eq!(allowed(at, &rules()), Allowed::Silence);
}

#[test]
fn monday_evening_belongs_to_tuesdays_session() {
    let at = utc("2026-08-17T22:00:00Z");

    assert_eq!(at.weekday(), Weekday::Mon, "the calendar says Monday");
    assert_eq!(trading_day(at, &rules()), Weekday::Tue);
    assert_ne!(allowed(at, &rules()), Allowed::Silence, "Tuesday is his");
}

#[test]
fn monday_daytime_is_still_monday() {
    let at = utc("2026-08-17T14:00:00Z"); // 10:00 New York

    assert_eq!(trading_day(at, &rules()), Weekday::Mon);
    assert_eq!(allowed(at, &rules()), Allowed::Silence);
}

#[test]
fn the_boundary_follows_new_york_through_the_clock_change() {
    let rules = rules();

    // Two Sundays, one in summer, one in winter, both at 21:30 UTC.
    let summer = utc("2026-08-16T21:30:00Z"); // 17:30 New York — open
    let winter = utc("2026-12-13T21:30:00Z"); // 16:30 New York — not yet

    assert_eq!(trading_day(summer, &rules), Weekday::Mon, "summer: open");
    assert_eq!(trading_day(winter, &rules), Weekday::Sun, "winter: not yet");

    // An hour later in winter and it has opened.
    assert_eq!(
        trading_day(utc("2026-12-13T22:30:00Z"), &rules),
        Weekday::Mon
    );
}

#[test]
fn five_oclock_exactly_is_the_new_day() {
    let rules = rules();
    let at = New_York
        .with_ymd_and_hms(2026, 8, 18, 17, 0, 0)
        .single()
        .expect("a real moment")
        .with_timezone(&Utc);

    assert_eq!(trading_day(at, &rules), Weekday::Wed, "Tuesday has ended");
    assert_eq!(
        trading_day(at - Duration::seconds(1), &rules),
        Weekday::Tue,
        "one second earlier it had not"
    );
}
