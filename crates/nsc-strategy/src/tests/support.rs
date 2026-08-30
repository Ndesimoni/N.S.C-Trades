//! The bits every test here needs.
//!
//! **The gold candles are the two off his own screenshot** — 19 and 20 August
//! 2026, the pair he circled and asked about. The same run the pattern tests
//! use, copied rather than invented: a made-up run passes whatever rule you
//! wrote for it.

use nsc_core::candle::Bar;
use nsc_core::levels::{Band, Timeframe};
use rust_decimal::Decimal;

use crate::Rules;

pub(super) fn d(text: &str) -> Decimal {
    text.parse().expect("a price")
}

pub(super) fn bar(open: &str, high: &str, low: &str, close: &str) -> Bar {
    Bar {
        datetime: "2026-08-20 00:00:00".into(),
        open: d(open),
        high: d(high),
        low: d(low),
        close: d(close),
    }
}

/// The settings as `config/strategy.toml` ships them.
pub(super) fn rules() -> Rules {
    Rules {
        reach_of_band: d("0.5"),
    }
}

/// A band 100 points thick, sitting on 4500. Reach at half is 50 either side.
pub(super) fn band() -> Band {
    Band {
        timeframe: Timeframe::Weekly,
        price: d("4500"),
        top: d("4550"),
        bottom: d("4450"),
    }
}

/// **His own gold, 19 and 20 August 2026.** A push of 1.9x a normal day with
/// 87% body, then a tail of 65 points under a body of five.
///
/// The pin's low is 4450.71, which is what the place test measures from.
pub(super) fn his_gold() -> (Vec<Bar>, Decimal) {
    (
        vec![
            bar("4344.53", "4524.36", "4324.71", "4517.78"),
            bar("4520.67", "4541.06", "4450.71", "4515.78"),
        ],
        d("104.27462"),
    )
}
