//! Reading the word the feed sent, and matching it against his settings.

use super::support::rules;
use crate::news::{Impact, Rules};

#[test]
fn reads_the_ratings_whatever_the_case() {
    assert_eq!(Impact::from_feed("High"), Impact::High);
    assert_eq!(Impact::from_feed("medium"), Impact::Medium);
    assert_eq!(Impact::from_feed("  LOW "), Impact::Low);
    assert_eq!(Impact::from_feed("Holiday"), Impact::Holiday);
}

#[test]
fn a_rating_nobody_taught_it_is_not_guessed_at() {
    let strange = Impact::from_feed("Severe");

    assert_eq!(strange, Impact::Unknown);
    assert!(
        !rules().wants(strange),
        "an unknown rating must never earn a message — it would arrive \
         looking exactly like a rate decision"
    );
}

#[test]
fn high_is_red_because_that_is_what_he_reads_every_day() {
    assert_eq!(Impact::High.colour(), "red");
    assert_eq!(Impact::Medium.colour(), "orange");
    assert_eq!(Impact::Low.colour(), "yellow");
}

#[test]
fn wants_only_what_the_settings_name() {
    assert!(rules().wants(Impact::High));
    assert!(rules().wants(Impact::Medium));
    assert!(!rules().wants(Impact::Low));
    assert!(!rules().wants(Impact::Holiday));
}

/// **An empty filter is silence, and silence looks like a quiet week.**
/// Matching the feed's raw text instead of the impact's own spelling would let
/// a change of case at their end do this without a word.
#[test]
fn a_change_of_case_in_the_settings_does_not_empty_the_filter() {
    let shouting = Rules {
        impacts: vec!["HIGH".into()],
        ..rules()
    };

    assert!(shouting.wants(Impact::High));
}
