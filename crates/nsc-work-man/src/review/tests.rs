//! Which of his levels land on the chart that was drawn.
//!
//! **The numbers are real ones.** They are USD/CAD on 17 August 2026 — price
//! at 1.3876, a weekly band at 1.34786–1.35214 and a daily one at
//! 1.41610–1.41790. That is the day the question came up: a 4-hour chart of
//! that pair holds neither, and the picture came out looking broken rather
//! than empty.

use nsc_core::levels::{Band, Timeframe};
use rust_decimal::Decimal;

use super::drawn::on_the_chart;

/// A band between two prices, written in hundred-thousandths.
fn band(bottom: i64, top: i64) -> Band {
    Band {
        timeframe: Timeframe::Weekly,
        price: Decimal::new((bottom + top) / 2, 5),
        top: Decimal::new(top, 5),
        bottom: Decimal::new(bottom, 5),
    }
}

/// The lowest and highest price the drawn candles reach.
fn covering(low: i64, high: i64) -> (Decimal, Decimal) {
    (Decimal::new(low, 5), Decimal::new(high, 5))
}

#[test]
fn a_band_among_the_candles_is_on_the_chart() {
    let (low, high) = covering(134_000, 142_000);

    assert_eq!(on_the_chart(&[band(134_786, 135_214)], low, high), 1);
}

/// A 4-hour chart of USD/CAD is twenty-five days wide. The daily level at
/// 1.4170 is a long way above every candle on it.
#[test]
fn a_band_far_above_the_candles_is_not() {
    let (low, high) = covering(138_500, 139_000);

    assert_eq!(on_the_chart(&[band(141_610, 141_790)], low, high), 0);
}

#[test]
fn a_band_far_below_the_candles_is_not() {
    let (low, high) = covering(138_500, 139_000);

    assert_eq!(on_the_chart(&[band(134_786, 135_214)], low, high), 0);
}

/// **The edge counts, not the line.** A level drawn above the highest candle
/// can still have its lower edge on screen, and that edge is exactly the part
/// he is looking for. Asking whether the line itself was on the chart would
/// report "nothing here" over a band he can plainly see.
#[test]
fn a_band_hanging_over_the_top_still_counts() {
    let (low, high) = covering(138_500, 139_000);

    assert_eq!(on_the_chart(&[band(138_900, 139_400)], low, high), 1);
}

/// The same from below — the top edge dipping into the candles is enough.
#[test]
fn a_band_reaching_up_from_underneath_still_counts() {
    let (low, high) = covering(138_500, 139_000);

    assert_eq!(on_the_chart(&[band(138_100, 138_600)], low, high), 1);
}

/// Touching exactly is on it. A band whose top sits on the lowest candle is
/// a band he can see, and `holds` treats its edges as inside too.
#[test]
fn touching_the_edge_exactly_is_on_it() {
    let (low, high) = covering(138_500, 139_000);

    assert_eq!(on_the_chart(&[band(138_000, 138_500)], low, high), 1);
}

#[test]
fn it_counts_only_the_ones_that_reach() {
    let (low, high) = covering(138_500, 139_000);

    let bands = [
        band(138_600, 138_800), // among the candles
        band(141_610, 141_790), // the daily, far above
        band(134_786, 135_214), // the weekly, far below
    ];

    assert_eq!(on_the_chart(&bands, low, high), 1);
}

/// A pair with no levels draws a plain chart rather than counting nothing
/// wrongly.
#[test]
fn no_bands_at_all_is_none_on_the_chart() {
    let (low, high) = covering(138_500, 139_000);

    assert_eq!(on_the_chart(&[], low, high), 0);
}
