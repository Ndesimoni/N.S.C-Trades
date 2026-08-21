//! Runs of two real gold candles.
//!
//! **Every one of these printed.** The two engulfings come from the gallery
//! drawn in August; the rest were pulled out of live IBKR data on 20 August
//! 2026 by running `--bin read` over gold.

use nsc_core::candle::Bar;

use super::making::bar;

/// 29 February 2024 — the clearest bullish engulfing in three years.
pub(in crate::pattern::tests) fn engulf_up() -> Vec<Bar> {
    vec![
        bar(
            "2024-02-29 19:00:00",
            "2037.39",
            "2037.55",
            "2028.91",
            "2030.36",
        ),
        bar(
            "2024-02-29 23:00:00",
            "2030.36",
            "2050.77",
            "2028.17",
            "2047.9",
        ),
    ]
}

/// 7 June 2024 — the clearest bearish engulfing. **And the clearest tweezer
/// top.** One run, two true statements.
pub(in crate::pattern::tests) fn engulf_down() -> Vec<Bar> {
    vec![
        bar(
            "2024-06-07 11:00:00",
            "2378.86",
            "2387.55",
            "2369.55",
            "2386.73",
        ),
        bar(
            "2024-06-07 15:00:00",
            "2386.77",
            "2387.77",
            "2342.65",
            "2347.4",
        ),
    ]
}

/// 9 July 2026 — two candles resting on the same low.
pub(in crate::pattern::tests) fn tweezer_bottom() -> Vec<Bar> {
    vec![
        bar(
            "2026-07-09 16:00:00",
            "4126.91",
            "4138.2",
            "4118.9",
            "4122.58",
        ),
        bar(
            "2026-07-09 20:00:00",
            "4122.58",
            "4125.78",
            "4120.6",
            "4123.72",
        ),
    ]
}

/// 7 July 2026.
pub(in crate::pattern::tests) fn dark_cloud() -> Vec<Bar> {
    vec![
        bar(
            "2026-07-07 08:00:00",
            "4124.89",
            "4158.49",
            "4119.92",
            "4157.21",
        ),
        bar(
            "2026-07-07 12:00:00",
            "4157.21",
            "4180.48",
            "4135.97",
            "4139.16",
        ),
    ]
}
