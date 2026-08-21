//! Turning written-down prices back into candles.

use nsc_core::candle::Bar;
use rust_decimal::Decimal;

fn d(text: &str) -> Decimal {
    text.parse().expect("a price")
}

pub(in crate::pattern::tests) fn bar(
    stamp: &str,
    open: &str,
    high: &str,
    low: &str,
    close: &str,
) -> Bar {
    Bar {
        datetime: stamp.into(),
        open: d(open),
        high: d(high),
        low: d(low),
        close: d(close),
    }
}

/// How big a normal gold 4-hour candle was **in early 2024**.
///
/// **There is one of these per era, and that is the point.** Gold was around
/// 2,030 in February 2024 and around 4,120 in July 2026, and its 4-hour range
/// grew with it. One number cannot cover both — a 2026 run judged against a
/// 2024 normal reads as a monster, and a 2024 run against a 2026 normal reads
/// as nothing happening.
///
/// The first version of these tests used a single 20 for everything, and the
/// real July 2026 tweezer failed: its lows are 1.70 apart, which is inside
/// tolerance on a 35-point candle and outside it on a 20-point one. The
/// candles were right and the yardstick was wrong.
pub(in crate::pattern::tests) fn normal_2024() -> Decimal {
    d("18")
}

/// And in mid-2026.
pub(in crate::pattern::tests) fn normal_2026() -> Decimal {
    d("35")
}
