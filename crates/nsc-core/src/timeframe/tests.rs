use chrono::{DateTime, Utc, Weekday};
use chrono_tz::America::New_York;

use super::*;
use crate::error::CoreError;

/// The normal forex boundary: 5pm New York, week starts Sunday.
fn forex() -> DayBoundary {
    DayBoundary::new(17, 0, New_York, Weekday::Sun).expect("17:00 is a real time")
}

fn utc(text: &str) -> DateTime<Utc> {
    text.parse::<DateTime<Utc>>().expect("valid timestamp")
}

#[test]
fn timeframes_parse_from_config_strings() {
    assert_eq!("H4".parse::<Timeframe>(), Ok(Timeframe::H4));
    assert_eq!("m15".parse::<Timeframe>(), Ok(Timeframe::M15));
    assert!("H3".parse::<Timeframe>().is_err());
}

#[test]
fn fifteen_minute_candles_round_down() {
    let b = forex();
    let start = Timeframe::M15
        .candle_start(utc("2026-07-15T14:37:00Z"), &b)
        .expect("valid");

    assert_eq!(start, utc("2026-07-15T14:30:00Z"));
}

// ── The two that matter most ──
//
// 5pm New York is a different UTC hour in summer and winter. If either of
// these ever starts failing, every daily level in the system has moved.

#[test]
fn the_day_starts_at_21_utc_in_summer() {
    let b = forex();
    let start = b.day_start(utc("2026-07-15T18:00:00Z")).expect("valid");

    // 17:00 New York in July = 21:00 UTC, on the previous calendar day
    // because 18:00 UTC is only 14:00 in New York.
    assert_eq!(start, utc("2026-07-14T21:00:00Z"));
}

#[test]
fn the_day_starts_at_22_utc_in_winter() {
    let b = forex();
    let start = b.day_start(utc("2026-01-15T18:00:00Z")).expect("valid");

    // Same wall clock, one hour later in UTC. This is the bug a fixed offset
    // would cause, caught by a test.
    assert_eq!(start, utc("2026-01-14T22:00:00Z"));
}

#[test]
fn sunday_evening_belongs_to_mondays_session() {
    let b = forex();

    // Sunday 9 Aug 2026, 22:00 UTC = 6pm in New York. Market already open.
    let start = b.day_start(utc("2026-08-09T22:00:00Z")).expect("valid");

    assert_eq!(start, utc("2026-08-09T21:00:00Z"));
    assert_eq!(
        b.next_day_start(utc("2026-08-09T22:00:00Z"))
            .expect("valid"),
        utc("2026-08-10T21:00:00Z")
    );
}

#[test]
fn the_week_starts_on_sunday_afternoon() {
    let b = forex();

    // Wednesday 12 Aug 2026, middle of the day.
    let start = b.week_start(utc("2026-08-12T12:00:00Z")).expect("valid");

    assert_eq!(start, utc("2026-08-09T21:00:00Z"));
}

#[test]
fn four_hour_candles_are_anchored_to_the_daily_close() {
    let b = forex();

    // Not 20:00 or 00:00 UTC. The day opened at 21:00, so the 4-hour candles
    // run 21:00, 01:00, 05:00 and so on.
    let start = Timeframe::H4
        .candle_start(utc("2026-07-15T23:30:00Z"), &b)
        .expect("valid");

    assert_eq!(start, utc("2026-07-15T21:00:00Z"));
}

#[test]
fn six_four_hour_candles_fill_a_normal_day() {
    let b = forex();
    let at = utc("2026-07-15T23:30:00Z");

    let day_open = b.day_start(at).expect("valid");
    let day_close = b.next_day_start(at).expect("valid");

    assert_eq!((day_close - day_open).num_hours(), 24);
    assert_eq!((day_close - day_open).num_minutes() / 240, 6);
}

#[test]
fn a_candle_ends_where_the_next_one_starts() {
    let b = forex();
    let at = utc("2026-07-15T14:37:00Z");

    let end = Timeframe::M15.candle_end(at, &b).expect("valid");
    let next = Timeframe::M15.candle_start(end, &b).expect("valid");

    assert_eq!(end, next);
}

#[test]
fn a_nonsense_close_time_is_rejected() {
    assert_eq!(
        DayBoundary::new(25, 0, New_York, Weekday::Sun),
        Err(CoreError::InvalidTimeOfDay {
            hour: 25,
            minute: 0
        })
    );
}
