//! Building candles to feed the finder.

use std::path::Path;

use nsc_core::candle::Bar;
use rust_decimal::Decimal;

use crate::swings::Rules;

pub(super) fn rules() -> Rules {
    crate::swings::load(Path::new("../../config/swings.toml"))
        .expect("config/swings.toml should read")
}

fn d(text: &str) -> Decimal {
    text.parse().expect("a price")
}

/// One candle on day `day` of August 2026, from its high and low.
///
/// **Open and close are the middle**, because none of the swing rules read
/// them — a swing sits at the extreme of the candle, wick included.
pub(super) fn bar(day: u32, high: &str, low: &str) -> Bar {
    let mid = (d(high) + d(low)) / Decimal::TWO;

    Bar {
        datetime: format!("2026-08-{day:02} 00:00:00"),
        open: mid,
        high: d(high),
        low: d(low),
        close: mid,
    }
}

/// A candle that is one price all the way through — a holiday.
pub(super) fn flat(day: u32, price: &str) -> Bar {
    bar(day, price, price)
}
