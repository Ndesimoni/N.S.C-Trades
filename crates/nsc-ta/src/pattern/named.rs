//! What a run of candles is called.

/// A pattern that finishes on the candle being judged.
///
/// **One per candle, same as a shape.** Real runs sit in more than one family
/// at once — a bearish engulfing can also be a tweezer top, and your own June
/// note found the 7 June 2024 candle being both. Answering with a list would
/// be honest and useless: every caller would have to pick, and every caller
/// would pick differently.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pattern {
    /// A body that swallows the one before it whole.
    Engulfing { up: bool },

    /// A big candle, then a small one hiding inside its body.
    Harami { up: bool },

    /// Two candles reaching the same extreme.
    Tweezer { top: bool },

    /// A down candle, then an up candle closing past its middle — but not
    /// past its open, or it would be an engulfing.
    PiercingLine,

    /// The same, the other way up.
    DarkCloudCover,

    /// A long move, a moment where nothing happened, then a long move back.
    ///
    /// **`abandoned` is the strict Japanese case** — the middle candle is a
    /// doji whose whole range cleared both neighbours. On spot forex that
    /// needs a gap, and spot forex only gaps at the Sunday open.
    Star { up: bool, abandoned: bool },

    /// **His own.** A push, then a pin whose tail opposes it.
    ///
    /// The only pattern here that is not from a textbook. `up` is the
    /// direction of the PUSH, so an `nsc-bull` is a push up met by a tail
    /// down.
    Push { up: bool },

    /// Three candles marching the same way, each closing beyond the last.
    ///
    /// **Three white soldiers going up, three black crows going down.** One of
    /// only two three-candle patterns that need neither a gap nor volume,
    /// which is why they survive on spot forex when the kicker and the three
    /// methods do not.
    Marching { up: bool },
}

impl Pattern {
    /// What to call it to a person.
    pub fn spoken(self) -> &'static str {
        match self {
            Self::Engulfing { up: true } => "nsc-bullish-engulfing",
            Self::Engulfing { up: false } => "nsc-bearish-engulfing",
            Self::Harami { up: true } => "bullish harami",
            Self::Harami { up: false } => "bearish harami",
            Self::Tweezer { top: true } => "tweezer top",
            Self::Tweezer { top: false } => "tweezer bottom",
            Self::PiercingLine => "piercing line",
            Self::DarkCloudCover => "dark cloud cover",
            Self::Star {
                up: true,
                abandoned: true,
            } => "abandoned baby (bullish)",
            Self::Star {
                up: false,
                abandoned: true,
            } => "abandoned baby (bearish)",
            Self::Star {
                up: true,
                abandoned: false,
            } => "morning star",
            Self::Star {
                up: false,
                abandoned: false,
            } => "evening star",
            Self::Push { up: true } => "nsc-bull",
            Self::Push { up: false } => "nsc-bear",
            Self::Marching { up: true } => "three white soldiers",
            Self::Marching { up: false } => "three black crows",
        }
    }

    /// How many candles it takes to see it.
    ///
    /// **Worth having on the type**, because it says how far back a caller had
    /// to hand over — and a pattern read from fewer candles than it needs is
    /// not a near miss, it is nothing at all.
    pub fn candles(self) -> usize {
        match self {
            Self::Star { .. } | Self::Marching { .. } => 3,
            _ => 2,
        }
    }
}
