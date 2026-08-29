//! Naming an event so it is only ever said once, and grouping a release.

use chrono::Duration;

use super::support::{event, nine};
use crate::news::{Event, Impact, together};

/// **Three Australian CPI numbers print in the same second.** One card each
/// buzzes his phone three times for what he reads as a single release — and
/// the whole design rests on messages being rare enough to open.
#[test]
fn events_at_the_same_moment_share_a_card() {
    let at = nine();
    let (a, b, c) = (
        event("CPI m/m", at, Impact::High),
        event("CPI y/y", at, Impact::High),
        event("Trimmed Mean CPI m/m", at, Impact::High),
    );

    let grouped = together(&[&a, &b, &c]);

    assert_eq!(grouped.len(), 1);
    assert_eq!(grouped[0].len(), 3);
}

#[test]
fn events_at_different_moments_do_not() {
    let a = event("Core PCE", nine(), Impact::High);
    let b = event("Prelim GDP", nine() + Duration::minutes(1), Impact::High);

    assert_eq!(together(&[&a, &b]).len(), 2);
}

#[test]
fn groups_come_back_in_the_order_they_will_happen() {
    let late = event(
        "Fed Chair Speaks",
        nine() + Duration::hours(2),
        Impact::High,
    );
    let early = event("Core PCE", nine(), Impact::High);

    let grouped = together(&[&late, &early]);

    assert_eq!(grouped[0][0].title, "Core PCE");
    assert_eq!(grouped[1][0].title, "Fed Chair Speaks");
}

#[test]
fn nothing_in_makes_nothing_out() {
    assert!(together(&[]).is_empty());
}

#[test]
fn three_releases_in_one_second_are_three_different_things() {
    let at = nine();
    let a = event("CPI m/m", at, Impact::High);
    let b = event("CPI y/y", at, Impact::High);

    assert_ne!(
        a.key(),
        b.key(),
        "the time alone cannot name an event — three CPI numbers share it"
    );
}

#[test]
fn the_same_release_next_month_is_a_different_thing() {
    let this = event("Core PCE", nine(), Impact::High);
    let next = event("Core PCE", nine() + Duration::days(30), Impact::High);

    assert_ne!(this.key(), next.key());
}

/// The file is downloaded every few hours and the same event is in every copy.
/// If its name changed with the download he would hear about one release all
/// day long.
#[test]
fn the_same_event_read_again_keeps_its_name() {
    let once = event("Core PCE", nine(), Impact::High);
    let read_again = event("Core PCE", nine(), Impact::High);

    assert_eq!(once.key(), read_again.key());
}

#[test]
fn a_speech_has_no_numbers_to_show() {
    let speech = Event {
        forecast: String::new(),
        previous: String::new(),
        ..event("Fed Chairman Warsh Speaks", nine(), Impact::High)
    };

    assert!(!speech.has_numbers());
    assert!(event("Core PCE", nine(), Impact::High).has_numbers());
}
