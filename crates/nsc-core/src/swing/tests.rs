//! The one thing a swing refuses to be.

use chrono::{TimeZone, Utc};
use rust_decimal::Decimal;

use super::{Swing, SwingError, SwingKind};

fn at(day: u32) -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 8, day, 0, 0, 0)
        .single()
        .expect("a real time")
}

fn price(text: &str) -> Decimal {
    text.parse().expect("a price")
}

/// **A swing cannot be known on the candle it sits on.**
///
/// You need candles AFTER a peak to know it was a peak. If this ever passes,
/// whatever built the swing is reading price the market had not printed — and
/// that mistake does not look broken, it makes results look better.
#[test]
fn a_swing_known_on_its_own_candle_is_refused() {
    let same = Swing::new(SwingKind::High, at(10), at(10), price("4500"));

    assert!(matches!(same, Err(SwingError::KnownTooSoon { .. })));
}

/// And known BEFORE it is worse still.
#[test]
fn a_swing_known_before_its_candle_is_refused() {
    let earlier = Swing::new(SwingKind::High, at(10), at(9), price("4500"));

    assert!(matches!(earlier, Err(SwingError::KnownTooSoon { .. })));
}

/// Confirmed later is the only shape that is allowed to exist.
#[test]
fn a_swing_confirmed_later_is_fine() {
    let swing = Swing::new(SwingKind::High, at(10), at(13), price("4500")).expect("valid");

    assert_eq!(swing.bar_time(), at(10));
    assert_eq!(swing.confirmed_at(), at(13));
    assert_eq!(swing.kind(), SwingKind::High);
}

/// **`known_by` is what a backtest must ask**, and it answers on the
/// confirmation, never on where the swing sits.
#[test]
fn a_swing_is_not_usable_until_it_is_confirmed() {
    let swing = Swing::new(SwingKind::High, at(10), at(13), price("4500")).expect("valid");

    assert!(!swing.known_by(at(10)), "on its own candle it is not known");
    assert!(
        !swing.known_by(at(12)),
        "nor while the pullback is still forming"
    );
    assert!(swing.known_by(at(13)), "known at the candle that proved it");
    assert!(swing.known_by(at(20)));
}

/// After a high the finder looks for a low. That is what makes them alternate.
#[test]
fn the_opposite_of_a_high_is_a_low() {
    assert_eq!(SwingKind::High.opposite(), SwingKind::Low);
    assert_eq!(SwingKind::Low.opposite(), SwingKind::High);
    assert_eq!(SwingKind::High.opposite().opposite(), SwingKind::High);
}
