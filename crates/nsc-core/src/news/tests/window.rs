//! The two edges of the warning window.
//!
//! These are the tests that matter most in this folder. The near edge is
//! obvious. The far edge is what stops a restart dumping a morning's releases
//! on him at once.

use chrono::Duration;

use super::support::{event, nine, rules};
use crate::news::{Impact, due, minutes_until};

#[test]
fn says_nothing_while_it_is_still_far_off() {
    let later = event("Core PCE", nine() + Duration::minutes(6), Impact::High);

    assert!(!due(&later, nine(), &rules()));
}

#[test]
fn speaks_once_it_is_inside_the_warning() {
    let soon = event("Core PCE", nine() + Duration::minutes(5), Impact::High);

    assert!(due(&soon, nine(), &rules()));
}

#[test]
fn still_speaks_the_moment_it_prints() {
    let now = event("Core PCE", nine(), Impact::High);

    assert!(due(&now, nine(), &rules()));
}

#[test]
fn still_speaks_just_after_it_printed() {
    let just_gone = event("Core PCE", nine() - Duration::minutes(5), Impact::High);

    assert!(due(&just_gone, nine(), &rules()));
}

/// **The restart case, and the whole reason `stale_minutes` exists.**
///
/// The bot comes back at nine having been down all night. The week's file is
/// full of yesterday. Without the far edge every one of those reads as
/// "coming up" and they all arrive together.
#[test]
fn a_release_from_this_morning_is_history_not_news() {
    let earlier = event("Core PCE", nine() - Duration::minutes(6), Impact::High);
    let yesterday = event("GDP", nine() - Duration::hours(20), Impact::High);

    assert!(!due(&earlier, nine(), &rules()));
    assert!(!due(&yesterday, nine(), &rules()));
}

#[test]
fn a_rating_he_did_not_ask_for_stays_quiet_inside_the_window() {
    let quiet = event("Loan Officer Survey", nine(), Impact::Low);

    assert!(!due(&quiet, nine(), &rules()));
}

#[test]
fn counts_the_minutes_and_goes_negative_after() {
    let soon = event("Core PCE", nine() + Duration::minutes(30), Impact::High);
    let gone = event("Core PCE", nine() - Duration::minutes(2), Impact::High);

    assert_eq!(minutes_until(&soon, nine()), 30);
    assert_eq!(minutes_until(&gone, nine()), -2);
}
