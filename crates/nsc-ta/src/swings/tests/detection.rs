//! The two ways a peak proves itself.

use nsc_core::swing::SwingKind;

use super::making::{bar, flat, rules};
use crate::swings::Finder;

/// **A peak proves itself when half the run has been given back.**
///
/// Up from 100 to 200, then back to 150. That is half of that move, and the
/// 200 is a swing high — regardless of how many candles it took.
#[test]
fn half_the_run_given_back_proves_the_peak() {
    let bars = [
        bar(1, "100", "100"),
        bar(2, "200", "150"),
        bar(3, "160", "150"),
        bar(4, "155", "148"),
    ];

    let found = Finder::over(rules(), &bars).expect("no lookahead");

    assert!(
        found.iter().any(|s| s.kind() == SwingKind::High),
        "the peak at 200 should have proved itself",
    );
}

/// **The same rule holds whether the turn took four candles or forty.**
///
/// This is why a candle count never worked. The move below is the same shape
/// as the one above, drawn out over many more candles, and it confirms just
/// the same.
#[test]
fn the_same_shape_over_more_candles_still_confirms() {
    let mut bars = vec![bar(1, "100", "100"), bar(2, "200", "150")];

    // A long, slow drift back down to half.
    for (at, high) in (3..12).zip([
        "195", "190", "186", "182", "178", "174", "170", "165", "160",
    ]) {
        bars.push(bar(at, high, "150"));
    }

    let found = Finder::over(rules(), &bars).expect("no lookahead");

    assert!(found.iter().any(|s| s.kind() == SwingKind::High));
}

/// **A swing is never confirmed on the candle it sits on.**
///
/// You need candles after a peak to know it was a peak. `Swing::new` refuses
/// the other case outright, so this is really checking the finder never asks
/// it to.
#[test]
fn nothing_is_ever_confirmed_on_its_own_candle() {
    let bars = [
        bar(1, "100", "100"),
        bar(2, "200", "150"),
        bar(3, "160", "150"),
        bar(4, "300", "150"),
        bar(5, "260", "200"),
        bar(6, "220", "170"),
    ];

    for swing in Finder::over(rules(), &bars).expect("no lookahead") {
        assert!(
            swing.confirmed_at() > swing.bar_time(),
            "{:?} was confirmed on its own candle",
            swing.kind(),
        );
    }
}

/// **Swings alternate.** After a high the finder is looking for a low.
///
/// The old windowed finder could call the same candle both — an outside bar
/// that beats everything around it in both directions. Under this rule that
/// cannot happen.
#[test]
fn swings_come_out_alternating() {
    let bars = [
        bar(1, "100", "100"),
        bar(2, "200", "150"),
        bar(3, "160", "120"),
        bar(4, "300", "150"),
        bar(5, "260", "200"),
        bar(6, "210", "160"),
        bar(7, "400", "200"),
        bar(8, "350", "260"),
        bar(9, "300", "240"),
    ];

    let found = Finder::over(rules(), &bars).expect("no lookahead");

    for pair in found.windows(2) {
        assert_ne!(
            pair[0].kind(),
            pair[1].kind(),
            "two of the same in a row at {}",
            pair[1].bar_time(),
        );
    }
}

/// **Two flat candles must not invent a swing.**
///
/// A run that starts and ends on the same candle is not a run — it is one
/// candle's height. Without that guard the whole of such a "run" is given back
/// inside the next candle, which passes every share test there is.
///
/// It bit hardest at the left edge of every history.
#[test]
fn two_flat_candles_do_not_make_a_swing() {
    let bars = [
        flat(1, "100"),
        flat(2, "100"),
        flat(3, "100"),
        flat(4, "100"),
    ];

    let found = Finder::over(rules(), &bars).expect("no lookahead");

    assert!(found.is_empty(), "a flat history has no swings in it");
}

/// A history of one candle proves nothing, and asking is not an error.
#[test]
fn one_candle_proves_nothing() {
    let found = Finder::over(rules(), &[bar(1, "200", "100")]).expect("no lookahead");

    assert!(found.is_empty());
}
