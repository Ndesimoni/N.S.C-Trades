//! The opening hours, and the moment they end.
//!
//! Nothing is said while they run. Prices are still watched — that is the
//! watcher's job, not this one's — but the phone stays quiet, and when they
//! end he gets one report of where price actually stands.

use super::super::{allowed, settled};
use super::support::{rules, utc};
use crate::when::Allowed;

// Summer, so 17:00 New York is 21:00 UTC. Tuesday's session opens Monday
// evening, which is the thing that catches people out.
const OPEN: &str = "2026-08-17T21:00:00Z";

#[test]
fn the_moment_it_opens_is_not_settled() {
    assert!(!settled(utc(OPEN), &rules()));
}

#[test]
fn three_hours_fifty_nine_is_still_not_settled() {
    assert!(!settled(utc("2026-08-18T00:59:00Z"), &rules()));
}

// Four hours exactly. It is one 4-hour candle, so the window ends on a
// boundary that exists rather than in the middle of one.
#[test]
fn four_hours_in_is_settled() {
    assert!(settled(utc("2026-08-18T01:00:00Z"), &rules()));
}

#[test]
fn the_middle_of_the_day_is_settled() {
    assert!(settled(utc("2026-08-18T14:00:00Z"), &rules()));
}

/// **The window is measured from the session's own open, not midnight.**
///
/// Read off the calendar day it would end at 04:00 UTC every day and have
/// nothing to do with when the market actually started.
#[test]
fn it_is_measured_from_the_session_not_the_clock() {
    // 02:00 UTC on Tuesday is five hours into the session that opened Monday
    // 21:00 — settled, even though the calendar day is two hours old.
    assert!(settled(utc("2026-08-18T02:00:00Z"), &rules()));
}

/// **Settled is not the same as tradeable**, and they must not be collapsed.
///
/// Friday is settled four hours in like any day, and still opens no trade.
/// Gating the report on "may trade" instead would silence it every Friday.
#[test]
fn friday_settles_even_though_it_opens_no_trade() {
    // Thursday 21:00 UTC opens Friday's session; four hours in is 01:00.
    let friday = utc("2026-08-21T01:00:00Z");

    assert!(settled(friday, &rules()), "the opening hours are over");
    assert_eq!(
        allowed(friday, &rules()),
        Allowed::WatchOnly,
        "but no trade is opened on a Friday"
    );
}
