use chrono::{DateTime, TimeDelta, Utc};
use rust_decimal::Decimal;

use super::*;
use crate::error::CoreError;
use crate::price::{Price, PriceDistance};
use crate::swing::SwingKind;

fn price(n: i64) -> Price {
    Price::new(Decimal::from(n))
}

fn distance(n: i64) -> PriceDistance {
    PriceDistance::new(Decimal::from(n))
}

fn at(day: i64) -> DateTime<Utc> {
    let start = "2026-08-10T00:00:00Z"
        .parse::<DateTime<Utc>>()
        .expect("valid timestamp");

    start + TimeDelta::try_days(day).expect("in range")
}

/// The worked example: a run of 200 up to 2100, then price carries 90 past it.
fn a_break() -> StructureBreak {
    StructureBreak::new(
        SwingKind::High,
        price(2100),
        at(1),
        distance(200),
        distance(90),
        at(5),
    )
    .expect("valid break")
}

// ── Trend ──

#[test]
fn an_old_high_taken_out_means_up() {
    assert_eq!(Trend::from_break(SwingKind::High), Trend::Up);
    assert_eq!(Trend::from_break(SwingKind::Low), Trend::Down);
}

// Unclear is a real answer, not a weak Up. Anything reading a trend has to be
// made to notice the difference.
#[test]
fn unclear_is_not_a_direction() {
    assert!(!Trend::Unclear.is_clear());
    assert!(!Trend::Unclear.is_up());
    assert!(!Trend::Unclear.is_down());
    assert!(Trend::Up.is_clear());
}

// ── Breaks ──

#[test]
fn a_break_reports_how_far_past_it_carried() {
    let broken = a_break();

    assert_eq!(broken.kind(), SwingKind::High);
    assert_eq!(broken.broken(), price(2100));
    assert_eq!(broken.carried(), distance(90));

    // 90 of a 200 run.
    assert_eq!(broken.share_of_run(), Some(0.45));
}

// ── Failed attempts ──

#[test]
fn a_failed_attempt_reports_how_far_it_got() {
    let attempt = FailedAttempt::new(
        SwingKind::High,
        price(2100),
        at(1),
        distance(200),
        distance(40),
        at(4),
        at(6),
    )
    .expect("valid attempt");

    assert_eq!(attempt.best(), distance(40));
    assert_eq!(attempt.share_of_run(), Some(0.2));
    assert!(!StructureEvent::Failed(attempt).is_taken());
}

// A push that never went past the extreme is not an attempt at all — nothing
// happened, and a row saying otherwise would be noise in the training data.
#[test]
fn an_attempt_that_never_crossed_is_refused() {
    let refused = FailedAttempt::new(
        SwingKind::High,
        price(2100),
        at(1),
        distance(200),
        distance(0),
        at(4),
        at(6),
    );

    assert!(matches!(
        refused,
        Err(CoreError::ImpossibleStructureBreak { .. })
    ));
}

#[test]
fn an_attempt_that_ends_before_it_starts_is_refused() {
    let refused = FailedAttempt::new(
        SwingKind::High,
        price(2100),
        at(1),
        distance(200),
        distance(40),
        at(6),
        at(4),
    );

    assert!(matches!(
        refused,
        Err(CoreError::ImpossibleStructureBreak { .. })
    ));
}

#[test]
fn a_break_that_happens_before_the_extreme_it_breaks_is_refused() {
    let refused = StructureBreak::new(
        SwingKind::High,
        price(2100),
        at(5),
        distance(200),
        distance(90),
        at(1),
    );

    assert!(matches!(
        refused,
        Err(CoreError::ImpossibleStructureBreak { .. })
    ));
}

#[test]
fn a_break_that_never_carried_past_is_refused() {
    let refused = StructureBreak::new(
        SwingKind::High,
        price(2100),
        at(1),
        distance(200),
        distance(0),
        at(5),
    );

    assert!(matches!(
        refused,
        Err(CoreError::ImpossibleStructureBreak { .. })
    ));
}

// Without a run behind it there is nothing to measure the follow-through
// against, so the threshold would mean nothing.
#[test]
fn a_break_of_an_extreme_with_no_run_behind_it_is_refused() {
    let refused = StructureBreak::new(
        SwingKind::High,
        price(2100),
        at(1),
        distance(0),
        distance(90),
        at(5),
    );

    assert!(matches!(
        refused,
        Err(CoreError::ImpossibleStructureBreak { .. })
    ));
}
