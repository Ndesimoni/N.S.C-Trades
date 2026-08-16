//! What one candle looks like, as four numbers.
//!
//! **Measured before it is named.** A doji, a hammer and a spinning top are
//! the same four numbers with different thresholds laid over them — so the
//! measuring happens once, here, and the naming happens after with numbers
//! from `config/`.
//!
//! Nothing in this file has an opinion. Two people measuring the same candle
//! get the same answer.

use nsc_core::candle::Bar;
use rust_decimal::Decimal;

/// What a candle looks like.
///
/// `body`, `upper` and `lower` are **shares of the whole candle** and always
/// add up to one. `reach` is the odd one out — see its note.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shape {
    /// How much of the candle is body. `0.02` is a doji, `0.95` is a
    /// marubozu.
    pub body: Decimal,

    /// The wick above the body.
    pub upper: Decimal,

    /// The wick below it.
    pub lower: Decimal,

    /// The whole candle, in ATR. **The only one that is not a share.**
    ///
    /// The other three say what SHAPE it is. This says whether it is worth
    /// looking at — a perfect hammer in a dead hour is noise. Of 394 gold
    /// dojis measured, only 240 reached half a normal candle; the rest had
    /// tiny bodies because nothing happened.
    ///
    /// In ATR rather than points, because three points is nothing on gold and
    /// a week on the euro. A points threshold works on the pair it was set on
    /// and quietly stops working on every other one.
    pub reach: Decimal,

    /// It closed above where it opened.
    ///
    /// Not a share, and not something the three above can tell you — a doji
    /// closing a hair up and one closing a hair down have the same shape.
    pub up: bool,
}

impl Shape {
    /// Measures a candle.
    ///
    /// **`None` when there is nothing to measure**, which is a real case and
    /// not a fault: the feed sends weekend and holiday candles, and on gold
    /// they are flat. 1,412 of 5,000 hourly candles had a range under 0.02%
    /// of price, and several were exactly zero.
    ///
    /// A flat candle has no shape. Dividing by its range would make every
    /// number here meaningless without saying so, which is worse than
    /// answering nothing.
    pub fn of(bar: &Bar, normal: Decimal) -> Option<Shape> {
        let range = bar.high - bar.low;

        if range <= Decimal::ZERO || normal <= Decimal::ZERO {
            return None;
        }

        let top = bar.open.max(bar.close);
        let bottom = bar.open.min(bar.close);

        Some(Shape {
            body: (bar.close - bar.open).abs() / range,
            upper: (bar.high - top) / range,
            lower: (bottom - bar.low) / range,
            reach: range / normal,
            up: bar.close >= bar.open,
        })
    }
}
