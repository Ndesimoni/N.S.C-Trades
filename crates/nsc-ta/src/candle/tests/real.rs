//! Eighteen real gold candles — the clearest example of each shape found in
//! 4,165 four-hour candles between October 2023 and August 2026.
//!
//! **Every one of these actually printed.** They come from the taxonomy drawn
//! in August (`docs/diagrams/candle-taxonomy.html`), which is why the counts
//! are here too: a shape found once in three years and a shape found 408 times
//! are different kinds of fact.

use nsc_core::candle::Bar;
use rust_decimal::Decimal;

/// One of them, with the name the taxonomy gave it and how often it turned up.
pub(super) struct Real {
    pub(super) called: &'static str,
    pub(super) stamp: &'static str,
    pub(super) bar: Bar,
    pub(super) found: usize,
}

fn d(text: &str) -> Decimal {
    text.parse().expect("a price")
}

fn bar(open: &str, high: &str, low: &str, close: &str) -> Bar {
    Bar {
        datetime: "2024-01-01 00:00:00".into(),
        open: d(open),
        high: d(high),
        low: d(low),
        close: d(close),
    }
}

/// Every shape the taxonomy found a real example of.
pub(super) fn every_one() -> Vec<Real> {
    vec![
        Real {
            called: "standard_doji",
            stamp: "2024-04-10 23:00",
            bar: bar("2338.47", "2352.43", "2319.44", "2337.27"),
            found: 38,
        },
        Real {
            called: "long_legged",
            stamp: "2025-09-09 23:00",
            bar: bar("3657.91", "3681.7", "3639.09", "3657.71"),
            found: 5,
        },
        Real {
            called: "dragonfly",
            stamp: "2024-04-15 23:00",
            bar: bar("2357.62", "2361.1", "2324.53", "2357.32"),
            found: 19,
        },
        Real {
            called: "gravestone",
            stamp: "2026-01-22 07:00",
            bar: bar("4780.27", "4840.25", "4775.08", "4780.95"),
            found: 10,
        },
        Real {
            called: "rickshaw",
            stamp: "2023-11-30 23:00",
            bar: bar("2037.25", "2044.35", "2031.57", "2037.74"),
            found: 32,
        },
        Real {
            called: "bull_marubozu",
            stamp: "2026-02-18 11:00",
            bar: bar("4858.92", "4921.42", "4858.92", "4921.03"),
            found: 4,
        },
        Real {
            called: "bear_marubozu",
            stamp: "2026-03-19 15:00",
            bar: bar("4851.76", "4853.64", "4733.05", "4733.05"),
            found: 3,
        },
        Real {
            called: "opening_maru",
            stamp: "2024-03-21 03:00",
            bar: bar("2155.91", "2188.85", "2155.69", "2182.9"),
            found: 87,
        },
        Real {
            called: "closing_maru",
            stamp: "2024-03-04 23:00",
            bar: bar("2082.18", "2109.39", "2081.24", "2109.15"),
            found: 43,
        },
        Real {
            called: "long_bull",
            stamp: "2026-01-29 07:00",
            bar: bar("5326.99", "5589.62", "5324.0", "5518.92"),
            found: 77,
        },
        Real {
            called: "long_bear",
            stamp: "2024-02-13 23:00",
            bar: bar("2027.74", "2032.71", "1990.21", "1992.31"),
            found: 89,
        },
        Real {
            called: "spinning_top",
            stamp: "2025-07-16 23:00",
            bar: bar("3334.77", "3377.19", "3319.93", "3347.0"),
            found: 408,
        },
        Real {
            called: "high_wave",
            stamp: "2024-07-11 23:00",
            bar: bar("2406.4", "2424.6", "2395.94", "2411.84"),
            found: 1,
        },
        Real {
            called: "hammer",
            stamp: "2025-10-17 11:00",
            bar: bar("4436.74", "4436.75", "4256.0", "4401.2"),
            found: 179,
        },
        Real {
            called: "takuri",
            stamp: "2025-10-17 11:00",
            bar: bar("4436.74", "4436.75", "4256.0", "4401.2"),
            found: 40,
        },
        Real {
            called: "shooting_star",
            stamp: "2025-08-12 03:00",
            bar: bar("3340.98", "3373.85", "3339.63", "3344.13"),
            found: 152,
        },
        Real {
            called: "bull_belt",
            stamp: "2026-01-29 07:00",
            bar: bar("5326.99", "5589.62", "5324.0", "5518.92"),
            found: 84,
        },
        Real {
            called: "bear_belt",
            stamp: "2024-11-25 11:00",
            bar: bar("2719.48", "2720.18", "2659.31", "2672.31"),
            found: 56,
        },
    ]
}
