//! What one candle is called.
//!
//! **These are SHAPES, not textbook names**, and the difference is the whole
//! reason this list is twelve rather than twenty-two.
//!
//! Four names on the textbook list are one shape wearing two labels:
//!
//! ```text
//!     Hammer / Hanging Man             identical candles
//!     Shooting Star / Inverted Hammer  identical candles
//!     Paper Umbrella                   IS the hammer shape
//!     Long Bullish / Belt Hold         the same candle
//! ```
//!
//! What separates a hammer from a hanging man is the **trend before it**, and
//! a candle cannot know that. The trend belongs to `nsc-strategy`. Name them
//! here and you get two detectors firing on one candle — so a backtest counts
//! one setup twice, and the number that comes out looks better than the truth.

/// What a candle looks like, in one word.
///
/// **One per candle.** Several of these overlap on real candles — a dragonfly
/// doji is also a long lower wick — and `naming.rs` settles which one wins by
/// testing the tightest rule first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Named {
    // ── the body is almost nothing ──
    /// Almost no body, no long wick either way.
    Doji,

    /// Almost no body, long wicks BOTH ways. Nobody won, hard.
    LongLeggedDoji,

    /// Almost no body, a long tail down and nothing above.
    DragonflyDoji,

    /// Almost no body, a long wick up and nothing below.
    GravestoneDoji,

    // ── the body is nearly everything ──
    /// All body, no wick at either end. **Rare** — seven in 4,165 gold
    /// candles. Almost everything a trader calls a marubozu is one of the
    /// next two.
    Marubozu,

    /// A long body that opened at its extreme with no wick there, and ran.
    /// The textbook calls this a belt-hold, or an opening marubozu.
    BeltHold,

    /// A long body that CLOSED at its extreme with no wick there.
    ClosingMarubozu,

    /// A long body with wicks at both ends. Just a big candle.
    LongBody,

    // ── price went looking one way and was refused ──
    /// A long tail down, small body at the top. Hammer, hanging man, paper
    /// umbrella and takuri are all this shape.
    LongLowerWick,

    /// A long wick up, small body at the bottom. Shooting star and inverted
    /// hammer are both this shape.
    LongUpperWick,

    // ── neither side finished in charge ──
    /// A small body with very long wicks both ways.
    HighWave,

    /// A small body with ordinary wicks both ways. **One candle in ten** on
    /// gold — far too common to mean anything on its own.
    SpinningTop,

    /// None of the above. Most candles are this.
    Plain,
}

impl Named {
    /// What to call it to a person.
    pub fn spoken(self) -> &'static str {
        match self {
            Self::Doji => "doji",
            Self::LongLeggedDoji => "long-legged doji",
            Self::DragonflyDoji => "dragonfly doji",
            Self::GravestoneDoji => "gravestone doji",
            Self::Marubozu => "marubozu",
            Self::BeltHold => "belt-hold",
            Self::ClosingMarubozu => "closing marubozu",
            Self::LongBody => "long body",
            Self::LongLowerWick => "long lower wick",
            Self::LongUpperWick => "long upper wick",
            Self::HighWave => "high wave",
            Self::SpinningTop => "spinning top",
            Self::Plain => "plain",
        }
    }
}
