//! The boundary, which is the only thing in this folder that is hard.

use chrono::{DateTime, Datelike, Duration, TimeZone, Utc, Weekday};
use chrono_tz::America::New_York;

use super::{Allowed, Rules, allowed, into_day, trading_day};

/// His calendar as `config/when.toml` has it.
fn rules() -> Rules {
    toml::from_str(
        r#"
day_ends = "17:00"
timezone = "America/New_York"
silent_days = ["monday"]
no_new_trades = ["friday"]
settle_hours = 4
look_in_minutes = 20
"#,
    )
    .expect("that is what the real file says")
}

fn utc(text: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(text)
        .expect("a real moment")
        .with_timezone(&Utc)
}

// ── Sunday evening is Monday ──

// THE ONE THAT MATTERS. The forex week opens Sunday 17:00 New York, so Sunday
// evening is already Monday's session.
//
// Read off the UTC calendar this is "Sunday", Monday's silence never covers
// it, and the bot talks through the exact hours he is not trading.
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

// And the other half of the same mistake: Monday evening is TUESDAY, which he
// does trade. Silence here would cost him the session.
#[test]
fn monday_evening_belongs_to_tuesdays_session() {
    let at = utc("2026-08-17T22:00:00Z");

    assert_eq!(at.weekday(), Weekday::Mon, "the calendar says Monday");
    assert_eq!(trading_day(at, &rules()), Weekday::Tue);
    assert_ne!(allowed(at, &rules()), Allowed::Silence, "Tuesday is his");
}

// Monday daytime is still Monday's session, and still silent. Between the two
// tests above it would be easy to overshoot and silence nothing.
#[test]
fn monday_daytime_is_still_monday() {
    let at = utc("2026-08-17T14:00:00Z"); // 10:00 New York

    assert_eq!(trading_day(at, &rules()), Weekday::Mon);
    assert_eq!(allowed(at, &rules()), Allowed::Silence);
}

// ── The clocks moving ──

// 17:00 New York is 21:00 UTC in summer and 22:00 UTC in winter. A fixed UTC
// boundary would be an hour out for half the year and never say so.
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

// The exact second counts as the new day, not the last one. `>=` rather than
// `>`, and the difference is one candle a day.
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

// ── The settle window ──

// The first hours of a day are where a move gets faked and taken back. What is
// happening still gets reported; only the trade is held back.
#[test]
fn a_trade_waits_four_hours_after_the_day_opens() {
    let rules = rules();
    let opened = utc("2026-08-18T21:00:00Z"); // Tuesday 17:00 New York

    assert_eq!(
        into_day(opened, &rules).num_minutes(),
        0,
        "right on the open"
    );

    assert_eq!(allowed(opened, &rules), Allowed::WatchOnly);
    assert_eq!(
        allowed(opened + Duration::hours(3), &rules),
        Allowed::WatchOnly,
        "still settling"
    );
    assert_eq!(
        allowed(opened + Duration::hours(4), &rules),
        Allowed::Anything,
        "four hours in, it may speak"
    );
}

// Watching is not trading. The settle window must not silence the thing that
// tells him price is at his level.
#[test]
fn the_settle_window_still_reports_what_is_happening() {
    let rules = rules();
    let early = utc("2026-08-18T22:00:00Z"); // an hour into Wednesday

    let says = allowed(early, &rules);

    assert!(says.says_anything(), "it still tells him");
    assert!(!says.may_trade(), "but suggests nothing");
}

// ── Friday ──

// A setup that needs the weekend to work out is one nobody can manage, and
// Sunday's gap is not something a stop protects against.
#[test]
fn friday_reports_but_suggests_no_trade() {
    let rules = rules();
    let at = utc("2026-08-21T14:00:00Z"); // Friday morning New York

    assert_eq!(trading_day(at, &rules), Weekday::Fri);
    assert_eq!(allowed(at, &rules), Allowed::WatchOnly);
}

// ── The middle of a normal week ──

#[test]
fn a_normal_wednesday_afternoon_allows_everything() {
    assert_eq!(
        allowed(utc("2026-08-19T14:00:00Z"), &rules()),
        Allowed::Anything
    );
}

// A calendar with nothing in it must not silence the week by accident.
#[test]
fn a_calendar_with_no_quiet_days_never_goes_silent() {
    let open: Rules = toml::from_str(
        r#"
day_ends = "17:00"
timezone = "America/New_York"
settle_hours = 0
"#,
    )
    .expect("a valid calendar");

    for moment in [
        "2026-08-16T22:00:00Z",
        "2026-08-17T14:00:00Z",
        "2026-08-21T14:00:00Z",
    ] {
        assert_eq!(allowed(utc(moment), &open), Allowed::Anything, "{moment}");
    }
}
