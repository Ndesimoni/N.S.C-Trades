//! Today, and the rest of the week.

use chrono::Duration;

use super::support::{event, nine, rules};
use crate::news::{Impact, Span, printed, within};

/// **Today means the whole day, not what is left of it.**
///
/// Showing only what is ahead makes a morning that has already had its rate
/// decision look like a quiet one.
#[test]
fn today_includes_what_has_already_printed() {
    let gone = event("Core PCE", nine() - Duration::hours(2), Impact::High);
    let coming = event("Fed Speaks", nine() + Duration::hours(3), Impact::High);
    let events = vec![gone, coming];

    let shown = within(&events, nine(), Span::Today, &rules());

    assert_eq!(shown.len(), 2);
}

#[test]
fn today_leaves_out_tomorrow() {
    let today = event("Core PCE", nine(), Impact::High);
    let tomorrow = event("GDP", nine() + Duration::days(1), Impact::High);
    let events = vec![today, tomorrow];

    let shown = within(&events, nine(), Span::Today, &rules());

    assert_eq!(shown.len(), 1);
    assert_eq!(shown[0].title, "Core PCE");
}

#[test]
fn today_leaves_out_yesterday() {
    let yesterday = event("GDP", nine() - Duration::days(1), Impact::High);
    let events = vec![yesterday];

    assert!(within(&events, nine(), Span::Today, &rules()).is_empty());
}

/// **The week keeps what has gone, and marks it.**
///
/// A week with its first three days silently missing does not read as a week
/// — it reads as a quiet one. Every row says which side of now it is on, so
/// nothing has to be guessed from where it sits in the list.
#[test]
fn the_week_keeps_what_has_already_gone() {
    let gone = event("Core PCE", nine() - Duration::days(2), Impact::High);
    let coming = event("Fed Speaks", nine() + Duration::days(2), Impact::High);
    let events = vec![gone, coming];

    let shown = within(&events, nine(), Span::Week, &rules());

    assert_eq!(shown.len(), 2);
    assert!(printed(shown[0], nine()), "the one behind us is marked");
    assert!(!printed(shown[1], nine()), "the one ahead is not");
}

/// One setting decides what counts, so the list he pulls up and the cards that
/// arrive on their own can never disagree.
#[test]
fn both_spans_filter_by_the_same_impacts() {
    let quiet = event(
        "Loan Officer Survey",
        nine() + Duration::hours(1),
        Impact::Low,
    );
    let events = vec![quiet];

    assert!(within(&events, nine(), Span::Today, &rules()).is_empty());
    assert!(within(&events, nine(), Span::Week, &rules()).is_empty());
}

#[test]
fn both_come_back_soonest_first() {
    let late = event("Fed Speaks", nine() + Duration::hours(5), Impact::High);
    let early = event("Core PCE", nine() + Duration::hours(1), Impact::High);
    let events = vec![late, early];

    let shown = within(&events, nine(), Span::Week, &rules());

    assert_eq!(shown[0].title, "Core PCE");
    assert_eq!(shown[1].title, "Fed Speaks");
}

#[test]
fn what_has_printed_is_marked_not_dropped() {
    let gone = event("Core PCE", nine() - Duration::minutes(1), Impact::High);
    let coming = event("Fed Speaks", nine() + Duration::minutes(1), Impact::High);

    assert!(printed(&gone, nine()));
    assert!(!printed(&coming, nine()));
}

#[test]
fn nothing_on_the_calendar_is_an_empty_list_not_a_panic() {
    assert!(within(&[], nine(), Span::Today, &rules()).is_empty());
    assert!(within(&[], nine(), Span::Week, &rules()).is_empty());
}
