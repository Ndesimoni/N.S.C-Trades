//! Which pattern, and which way it points.

use serde::{Deserialize, Serialize};

/// The candlestick patterns this project looks for.
///
/// The whole list. If a shape is not here, the bot does not look for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CandleShape {
    /// A long wick with a small body at the far end of it.
    ///
    /// A hammer is a bullish pin bar and a shooting star is a bearish one.
    /// Textbook separates them by what came before — a hammer follows a
    /// downtrend — and that is context, not shape, so it is one pattern here
    /// with a direction.
    PinBar,

    /// The second candle's body completely covers the first candle's body,
    /// the two being opposite colours.
    Engulfing,

    /// Open and close in nearly the same place.
    Doji(DojiKind),

    /// A long candle that opens at one extreme with no wick there.
    BeltHold,

    /// Two neighbouring candles reaching the same high, or the same low.
    Tweezers,

    /// A candle whose whole range sits inside the one before it.
    ///
    /// Points nowhere on its own. It is the market pausing, and which way it
    /// leaves the pause is the part that matters — which is a rule, not a
    /// shape.
    InsideBar,

    /// Three candles: a strong push, a small stalled one, then a strong
    /// candle back the other way.
    ///
    /// Bullish is a morning star at a bottom, bearish an evening star at a
    /// top. The only three-candle shape here.
    Star,
}

/// Which doji.
///
/// Same rule — almost no body — with the wicks in different places, and they
/// are not the same event. A dragonfly is a rejection of lower prices; a
/// gravestone is a rejection of higher ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DojiKind {
    /// Long wicks both sides.
    LongLegged,

    /// A long wick below and almost none above.
    Dragonfly,

    /// A long wick above and almost none below.
    Gravestone,

    /// Barely any wick either side — a candle that went nowhere at all.
    Plain,
}

/// Which way a shape points, if it points anywhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Bias {
    Bullish,
    Bearish,

    /// Points nowhere on its own. A doji is indecision — it says the market
    /// could not pick a side, which is exactly why it needs context to mean
    /// anything.
    Neutral,
}

impl Bias {
    pub fn is_bullish(self) -> bool {
        matches!(self, Bias::Bullish)
    }

    pub fn is_bearish(self) -> bool {
        matches!(self, Bias::Bearish)
    }
}
