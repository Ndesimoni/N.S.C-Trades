//! One big candle, and the small candle it started on.

use rust_decimal::Decimal;

/// One big candle, and the small candle it started on.
pub struct Lined {
    /// The big candle's own stamp, as the feed wrote it.
    pub big: String,

    /// The small candles that opened at exactly the same price.
    ///
    /// **Usually one. Sometimes none, and sometimes several** — a quiet market
    /// can open two hours at the same number, and then the hour is not proved
    /// by this candle alone. Several agreeing candles settle it; one is a
    /// coincidence waiting to happen.
    pub started: Vec<String>,

    /// How far the nearest small candle's open was, when none matched exactly.
    pub nearest: Option<(String, Decimal)>,
}
