//! What the bot may say, once the day is known.

use chrono::{Duration, Weekday};

use super::super::{Allowed, Rules, allowed, into_day, trading_day};
use super::support::{rules, utc};

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

#[test]
fn the_settle_window_still_reports_what_is_happening() {
    let rules = rules();
    let early = utc("2026-08-18T22:00:00Z"); // an hour into Wednesday

    let says = allowed(early, &rules);

    assert!(says.says_anything(), "it still tells him");
    assert!(!says.may_trade(), "but suggests nothing");
}

#[test]
fn friday_reports_but_suggests_no_trade() {
    let rules = rules();
    let at = utc("2026-08-21T14:00:00Z"); // Friday morning New York

    assert_eq!(trading_day(at, &rules), Weekday::Fri);
    assert_eq!(allowed(at, &rules), Allowed::WatchOnly);
}

#[test]
fn a_normal_wednesday_afternoon_allows_everything() {
    assert_eq!(
        allowed(utc("2026-08-19T14:00:00Z"), &rules()),
        Allowed::Anything
    );
}

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
