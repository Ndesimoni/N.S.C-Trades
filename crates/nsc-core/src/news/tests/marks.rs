//! **Two warnings about one release**, and which of them is live.
//!
//! `warn_at_minutes = [5, 1]` — a heads-up five minutes out and a last call
//! one minute out. His ask, 1 September 2026: *"we are going to have five
//! minutes and one minute."*
//!
//! The thing these tests actually protect is that ONE mark is live at a time.
//! Windows that both stayed open would make the second card either impossible
//! to tell from the first or impossible to send at all, depending on which way
//! the caller broke the tie.

use chrono::Duration;

use super::support::{event, nine, rules};
use crate::news::{Impact, due_at};

/// A release at nine, judged from a given number of minutes before it.
fn at_minutes_before(minutes: i64) -> Option<i64> {
    let release = event(
        "Core PCE",
        nine() + Duration::minutes(minutes),
        Impact::High,
    );

    due_at(&release, nine(), &rules())
}

#[test]
fn nothing_is_live_before_the_widest_mark() {
    assert_eq!(at_minutes_before(6), None, "still too far off");
}

#[test]
fn the_five_minute_mark_owns_the_stretch_down_to_one() {
    assert_eq!(at_minutes_before(5), Some(5), "the moment it opens");
    assert_eq!(at_minutes_before(3), Some(5));
    assert_eq!(at_minutes_before(2), Some(5), "the last minute it owns");
}

/// **The handover, and the one that would break first.**
///
/// At exactly one minute out both marks would qualify if the windows simply
/// ran from their own start to the release. The narrower one wins, because it
/// is the newer piece of news.
#[test]
fn the_one_minute_mark_takes_over_at_one() {
    assert_eq!(at_minutes_before(1), Some(1), "not 5 — the narrower wins");
}

#[test]
fn the_last_mark_is_the_one_that_outlives_the_release() {
    let printed = event("Core PCE", nine(), Impact::High);
    assert_eq!(due_at(&printed, nine(), &rules()), Some(1), "as it prints");

    // `stale_minutes` is 5, so five minutes after is the far edge itself.
    let gone = event("Core PCE", nine() - Duration::minutes(5), Impact::High);
    assert_eq!(due_at(&gone, nine(), &rules()), Some(1));

    let older = event("Core PCE", nine() - Duration::minutes(6), Impact::High);
    assert_eq!(due_at(&older, nine(), &rules()), None, "past it now");
}

/// **A restart just after a release gets the LAST call, not the heads-up.**
///
/// A minute to a release is news he can act on. Five minutes ago is history,
/// and a card headed "in 5 minutes" about something that has already printed
/// is worse than silence.
#[test]
fn coming_back_up_after_it_printed_gives_the_last_call() {
    let just_gone = event("Core PCE", nine() - Duration::minutes(2), Impact::High);

    assert_eq!(due_at(&just_gone, nine(), &rules()), Some(1));
}

/// **The file is tidied, not trusted.** A trader writing them small-first, or
/// twice, means the obvious thing — and the windows only line up if the marks
/// run widest first with no repeats.
#[test]
fn the_order_they_are_written_in_does_not_matter() {
    let mut muddled = rules();
    muddled.warn_at_minutes = vec![1, 5, 5];

    let soon = event("Core PCE", nine() + Duration::minutes(3), Impact::High);
    assert_eq!(due_at(&soon, nine(), &muddled), Some(5));

    let closer = event("Core PCE", nine() + Duration::minutes(1), Impact::High);
    assert_eq!(due_at(&closer, nine(), &muddled), Some(1));
}

/// One mark on its own still works, and still owns the far edge.
#[test]
fn a_single_mark_behaves_as_it_always_did() {
    let mut only_one = rules();
    only_one.warn_at_minutes = vec![5];

    let soon = event("Core PCE", nine() + Duration::minutes(3), Impact::High);
    assert_eq!(due_at(&soon, nine(), &only_one), Some(5));

    let gone = event("Core PCE", nine() - Duration::minutes(4), Impact::High);
    assert_eq!(due_at(&gone, nine(), &only_one), Some(5));
}

/// **A minute is comfortably wider than the tick.**
///
/// The watcher wakes once a minute. The last mark's window runs from a minute
/// before the release to `stale_minutes` after it — six minutes on his
/// settings — so no tick can step over it. A mark narrower than a minute
/// COULD be missed, and that is the reason not to add one.
#[test]
fn the_last_window_is_wider_than_the_watchers_tick() {
    let widest = rules().warn_at_minutes.iter().copied().min().unwrap_or(0);
    let window = widest + rules().stale_minutes;

    assert!(window > 1, "a 60-second tick must not be able to skip it");
}

/// An impact he does not want is not live at any mark.
#[test]
fn a_rating_he_ignores_is_never_live() {
    let quiet = event("Some speech", nine() + Duration::minutes(3), Impact::Low);

    assert_eq!(due_at(&quiet, nine(), &rules()), None);
}
