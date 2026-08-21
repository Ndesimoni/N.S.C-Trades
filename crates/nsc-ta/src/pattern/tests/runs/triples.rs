//! Runs of three real gold candles.
//!
//! **Every one of these printed.** The two engulfings come from the gallery
//! drawn in August; the rest were pulled out of live IBKR data on 20 August
//! 2026 by running `--bin read` over gold.

use nsc_core::candle::Bar;

use super::making::bar;

/// 1 July 2026 — down hard, a pause, then back up.
pub(in crate::pattern::tests) fn morning_star() -> Vec<Bar> {
    vec![
        bar(
            "2026-07-01 00:00:00",
            "4005.86",
            "4009.84",
            "3970.43",
            "3980.59",
        ),
        bar(
            "2026-07-01 04:00:00",
            "3980.59",
            "3982.82",
            "3960.35",
            "3976.1",
        ),
        bar(
            "2026-07-01 08:00:00",
            "3976.1",
            "4035.59",
            "3969.56",
            "4030.45",
        ),
    ]
}

/// 8 July 2026.
pub(in crate::pattern::tests) fn evening_star() -> Vec<Bar> {
    vec![
        bar(
            "2026-07-08 00:00:00",
            "4098.76",
            "4131.23",
            "4096.5",
            "4123.6",
        ),
        bar(
            "2026-07-08 04:00:00",
            "4123.6",
            "4134.15",
            "4114.92",
            "4121.35",
        ),
        bar(
            "2026-07-08 08:00:00",
            "4121.35",
            "4121.43",
            "4040.63",
            "4062.23",
        ),
    ]
}

/// 20 January 2026 — three up candles, each closing beyond the last.
pub(in crate::pattern::tests) fn soldiers() -> Vec<Bar> {
    vec![
        bar(
            "2026-01-20 00:00:00",
            "4665.47",
            "4681.54",
            "4659.55",
            "4680.25",
        ),
        bar(
            "2026-01-20 04:00:00",
            "4680.25",
            "4722.81",
            "4679.73",
            "4716.22",
        ),
        bar(
            "2026-01-20 08:00:00",
            "4716.22",
            "4737.52",
            "4715.13",
            "4733.04",
        ),
    ]
}

/// 28 April 2026 — the same march, downhill.
pub(in crate::pattern::tests) fn crows() -> Vec<Bar> {
    vec![
        bar(
            "2026-04-28 00:00:00",
            "4692.1",
            "4701.5",
            "4666.47",
            "4671.11",
        ),
        bar(
            "2026-04-28 04:00:00",
            "4671.11",
            "4671.42",
            "4620.67",
            "4633.19",
        ),
        bar(
            "2026-04-28 08:00:00",
            "4633.19",
            "4635.5",
            "4555.53",
            "4562.51",
        ),
    ]
}

/// **Not a real candle run, and it says so.**
///
/// No abandoned baby exists in the gold data — none was found in 300 candles,
/// and none can be, because spot forex does not gap. This is the morning star
/// above with its middle candle moved clear of both neighbours, which is the
/// only way to exercise the strict case at all.
pub(in crate::pattern::tests) fn abandoned_baby_made_up() -> Vec<Bar> {
    vec![
        bar(
            "2026-07-01 00:00:00",
            "4005.86",
            "4009.84",
            "3970.43",
            "3980.59",
        ),
        // Its whole range sits below both neighbours' lows.
        bar(
            "2026-07-01 04:00:00",
            "3950.10",
            "3951.00",
            "3948.00",
            "3949.90",
        ),
        bar(
            "2026-07-01 08:00:00",
            "3976.1",
            "4035.59",
            "3969.56",
            "4030.45",
        ),
    ]
}
