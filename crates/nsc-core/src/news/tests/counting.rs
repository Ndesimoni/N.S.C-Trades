//! How long until it prints, in words.

use chrono::Duration;

use super::support::{event, nine};
use crate::news::{Impact, away_words};

fn words(minutes: i64) -> String {
    let when = event(
        "Core PCE",
        nine() + Duration::minutes(minutes),
        Impact::High,
    );

    away_words(&when, nine())
}

#[test]
fn one_that_has_gone_says_so() {
    assert_eq!(words(-1), "passed");
    assert_eq!(words(-600), "passed");
}

#[test]
fn one_printing_this_minute_says_now() {
    assert_eq!(words(0), "now");
}

#[test]
fn under_an_hour_counts_in_minutes() {
    assert_eq!(words(1), "in 1m");
    assert_eq!(words(45), "in 45m");
    assert_eq!(words(59), "in 59m");
}

/// **The units shrink as it gets closer.** Forty minutes out, the minutes are
/// the only thing that matters — and "in 0h" reads as a card that failed to
/// fill in.
#[test]
fn an_hour_brings_in_the_hours() {
    assert_eq!(words(60), "in 1h");
    assert_eq!(words(80), "in 1h 20m");
    assert_eq!(words(200), "in 3h 20m");
}

#[test]
fn a_whole_number_of_hours_does_not_say_zero_minutes() {
    assert_eq!(words(120), "in 2h");
    assert_eq!(words(300), "in 5h");
}

/// Two days out the hours stop mattering, so they are only shown when they
/// are not nought.
#[test]
fn past_a_day_it_counts_in_days() {
    assert_eq!(words(24 * 60), "in 1d");
    assert_eq!(words(24 * 60 + 4 * 60), "in 1d 4h");
    assert_eq!(words(3 * 24 * 60), "in 3d");
}

/// The boundary either side, because an off-by-one here is the kind of thing
/// nobody notices: "in 60m" and "in 1h" are both readable, so only a test
/// says which one it does.
#[test]
fn the_hour_boundary_falls_the_right_way() {
    assert_eq!(words(59), "in 59m");
    assert_eq!(words(60), "in 1h");
}

#[test]
fn the_day_boundary_falls_the_right_way() {
    assert_eq!(words(24 * 60 - 1), "in 23h 59m");
    assert_eq!(words(24 * 60), "in 1d");
}
