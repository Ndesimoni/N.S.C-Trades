//! Building charts and swings to test with.
//!
//! Whole numbers throughout, so every run and every give-back can be checked
//! by hand.

use chrono::{DateTime, TimeDelta, Utc};
use nsc_core::candle::Candle;
use nsc_core::level::Level;
use nsc_core::price::{Price, PriceDistance};
use nsc_core::swing::{Swing, SwingKind};
use rust_decimal::Decimal;

use crate::config::{LevelSettings, SwingSettings};

/// Short enough that a test chart of a dozen candles has an ATR at the end.
/// The real setting is 14; nothing here depends on which it is.
pub const ATR_PERIOD: usize = 3;

pub fn at(index: i64) -> DateTime<Utc> {
    let start = "2026-08-10T00:00:00Z"
        .parse::<DateTime<Utc>>()
        .expect("valid timestamp");

    start + TimeDelta::try_minutes(index * 15).expect("in range")
}

pub fn price(n: i64) -> Price {
    Price::new(Decimal::from(n))
}

pub fn distance(n: i64) -> PriceDistance {
    PriceDistance::new(Decimal::from(n))
}

pub fn candle(index: i64, high: i64, low: i64) -> Candle {
    let middle = (high + low) / 2;

    Candle::new(
        at(index),
        price(middle),
        price(high),
        price(low),
        price(middle),
        None,
        true,
    )
    .expect("valid candle")
}

/// A candle sitting at one price.
pub fn tick(index: i64, at_price: i64) -> Candle {
    candle(index, at_price, at_price)
}

/// A chart that walks from one turn price to the next, with one candle halfway
/// between so each move takes more than a single step.
///
/// Turn `n` lands on candle `2n`, which is what the date tests check against.
///
/// Each full swing gives back the whole of the run before it, so every turn
/// except the last one confirms.
pub fn zigzag(turns: &[i64]) -> Vec<Candle> {
    let mut candles = Vec::new();
    let mut index = 0;

    for (position, turn) in turns.iter().enumerate() {
        if position > 0 {
            let previous = turns[position - 1];
            candles.push(tick(index, (previous + turn) / 2));
            index += 1;
        }

        candles.push(tick(index, *turn));
        index += 1;
    }

    candles
}

/// Where `zigzag` puts the nth turn.
pub fn turn_index(nth: i64) -> i64 {
    nth * 2
}

/// The level covering this price, if one was found.
pub fn level_at(levels: &[Level], at_price: i64) -> Option<Level> {
    levels
        .iter()
        .find(|level| level.contains(price(at_price)))
        .copied()
}

pub fn swing_settings() -> SwingSettings {
    SwingSettings {
        confirm_retracement: 0.5,
        shallow_retracement: 0.382,
        min_run_fraction: 0.5,
        run_memory_legs: 5,
    }
}

pub fn level_settings(min_touches: usize, max_age_bars: usize) -> LevelSettings {
    LevelSettings {
        band_atr_multiple: 0.5,
        min_touches,
        max_age_bars,
        absorb_gap_bands: 1.5,
        min_separation_bands: 3.0,
    }
}

/// A swing made by hand rather than found on a chart.
pub fn swing(kind: SwingKind, index: i64, at_price: i64) -> Swing {
    Swing::new(kind, at(index), at(index + 3), price(at_price)).expect("valid swing")
}
