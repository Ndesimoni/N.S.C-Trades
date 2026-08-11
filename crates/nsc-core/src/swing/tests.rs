use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use super::*;
use crate::error::CoreError;
use crate::price::Price;

fn at(text: &str) -> DateTime<Utc> {
    text.parse::<DateTime<Utc>>().expect("valid timestamp")
}

fn swing_at(bar: &str, confirmed: &str) -> Result<Swing, CoreError> {
    Swing::new(
        SwingKind::High,
        at(bar),
        at(confirmed),
        Price::new(Decimal::from(4320)),
    )
}

#[test]
fn a_normal_swing_is_accepted() {
    let s = swing_at("2026-08-10T14:00:00Z", "2026-08-10T14:45:00Z").expect("valid");

    assert!(s.is_high());
    assert_eq!(s.bar_time(), at("2026-08-10T14:00:00Z"));
    assert_eq!(s.confirmed_at(), at("2026-08-10T14:45:00Z"));
}

#[test]
fn a_swing_known_before_it_happened_is_refused() {
    let result = swing_at("2026-08-10T14:00:00Z", "2026-08-10T13:00:00Z");
    assert!(matches!(result, Err(CoreError::SwingKnownTooEarly { .. })));
}

#[test]
fn a_swing_known_the_moment_it_happened_is_refused() {
    // You cannot tell a peak is a peak while it is printing. Price could
    // still carry on up.
    let result = swing_at("2026-08-10T14:00:00Z", "2026-08-10T14:00:00Z");
    assert!(matches!(result, Err(CoreError::SwingKnownTooEarly { .. })));
}

#[test]
fn a_high_and_a_low_are_opposites() {
    assert_eq!(SwingKind::High.opposite(), SwingKind::Low);
    assert_eq!(SwingKind::Low.opposite(), SwingKind::High);
    assert!(SwingKind::High.is_high());
    assert!(!SwingKind::High.is_low());
}

// ── The one that keeps backtests honest ──

#[test]
fn a_swing_is_invisible_until_it_is_confirmed() {
    let s = swing_at("2026-08-10T14:00:00Z", "2026-08-10T14:45:00Z").expect("valid");

    // The swing exists on the chart, but nobody knows it yet.
    assert!(!s.is_known_at(at("2026-08-10T14:00:00Z")));
    assert!(!s.is_known_at(at("2026-08-10T14:30:00Z")));

    // Now it is knowable.
    assert!(s.is_known_at(at("2026-08-10T14:45:00Z")));
    assert!(s.is_known_at(at("2026-08-10T18:00:00Z")));
}
