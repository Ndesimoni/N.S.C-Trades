//! Building charts and swings to test with.
//!
//! Whole numbers throughout, so the sums can be checked by hand. Flat candles
//! are 10 tall, which puts ATR near 10 and makes a band of half a normal
//! candle about 5 wide.

use chrono::{DateTime, TimeDelta, Utc};
use nsc_core::candle::Candle;
use nsc_core::price::{Price, PriceDistance};
use nsc_core::swing::{Swing, SwingKind};
use rust_decimal::Decimal;

use crate::config::{LevelSettings, SwingSettings};

/// How far apart the peaks are placed. Comfortably more than the lookback of
/// 3, so each one is judged on its own.
const SPACING: i64 = 8;

/// Where the first peak goes. The candles before it are there to warm ATR up.
const FIRST_PEAK: i64 = 20;

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

pub fn swing_settings() -> SwingSettings {
    SwingSettings {
        lookback: 3,
        require_confirmed: true,
        min_atr_multiple: 0.5,
    }
}

pub fn level_settings(min_touches: usize, max_age_bars: usize) -> LevelSettings {
    LevelSettings {
        band_atr_multiple: 0.5,
        min_touches,
        max_age_bars,
    }
}

/// A swing, made by hand rather than found on a chart. Confirmed three
/// candles after the one it sits on, the way the finder would.
pub fn swing(kind: SwingKind, index: i64, at_price: i64) -> Swing {
    Swing::new(kind, at(index), at(index + 3), price(at_price)).expect("valid swing")
}

/// A chart with a peak at each of the given prices, well spaced out, and
/// enough flat candles after the last one for it to confirm.
///
/// Every other candle runs from 95 to 105, so nothing else is a swing.
pub fn chart_with_peaks(peaks: &[i64]) -> Vec<Candle> {
    let mut candles: Vec<Candle> = (0..FIRST_PEAK).map(|i| candle(i, 105, 95)).collect();
    let mut index = FIRST_PEAK;

    for peak in peaks {
        candles.push(candle(index, *peak, 95));
        index += 1;

        for _ in 1..SPACING {
            candles.push(candle(index, 105, 95));
            index += 1;
        }
    }

    candles
}

/// Where `chart_with_peaks` puts the nth peak.
pub fn peak_index(nth: i64) -> i64 {
    FIRST_PEAK + nth * SPACING
}
