//! Which shapes he trades, and where each one is measured from.

use nsc_core::candle::Bar;
use nsc_ta::pattern::Pattern;
use rust_decimal::Decimal;

/// A shape he trades, and nothing else.
///
/// **`pattern/` names eight shapes and he trades four.** Tweezers, piercing,
/// dark cloud and the star exist because they are on every candlestick page,
/// not because they are on his chart.
///
/// Harami and marching were added on 29 August 2026, at his word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Traded {
    /// His own: a push, then a pin whose tail opposes it.
    Push { up: bool },

    /// A body that swallows the one before it whole.
    Engulfing { up: bool },

    /// A big candle, then a small one hiding inside its body. The move did not
    /// reverse — it ran out.
    Harami { up: bool },

    /// Three candles the same way, each closing beyond the last. Three white
    /// soldiers going up, three black crows going down.
    Marching { up: bool },
}

/// Is this a shape he trades?
pub fn traded(pattern: Pattern) -> Option<Traded> {
    match pattern {
        Pattern::Push { up } => Some(Traded::Push { up }),
        Pattern::Engulfing { up } => Some(Traded::Engulfing { up }),
        Pattern::Harami { up } => Some(Traded::Harami { up }),
        Pattern::Marching { up } => Some(Traded::Marching { up }),

        // Named, and not his. Left out rather than quietly included — a
        // detector that fires on eight shapes when he trades four is twice the
        // messages and half the meaning.
        _ => None,
    }
}

impl Traded {
    /// How many candles the shape is made of.
    ///
    /// **Marching is the only three-candle one he trades**, and `touching`
    /// below needs to know how far back to reach.
    pub fn candles(self) -> usize {
        match self {
            Traded::Marching { .. } => 3,
            _ => 2,
        }
    }

    /// The price the level has to be near.
    ///
    /// **MEASURE FROM THE PRICE THAT ACTUALLY REACHED THE LEVEL.** That is the
    /// one rule here, and each shape answers it differently because each one
    /// reaches with a different candle.
    ///
    /// `bars` are the candles up to and including the one being judged, oldest
    /// first — the same slice the detector was handed.
    ///
    /// - **Push** — the pin's tail tip. The tail is a pullback that failed; if
    ///   it reached into the level, the level is what stopped it, and that is
    ///   the whole story of the setup. `up` pushes up so its tail points down:
    ///   the pin's low.
    /// - **Engulfing** — where the second candle closed. It has no tail to
    ///   speak of, and the close is what "engulfing at the level" means to the
    ///   eye.
    /// - **Harami** — the **first, big** candle's far extreme, NOT the small
    ///   one. Price travelled into the zone on the big candle; the small one is
    ///   only the proof it stopped there. A bullish harami falls into the zone,
    ///   so its low is the reach.
    /// - **Marching** — the **first** candle's far extreme. A run of three
    ///   launches from somewhere, and if a zone is there that is the zone it
    ///   broke out of. Measuring from the end would put the setup three candles
    ///   away from the level that caused it.
    ///
    /// Gives back nothing when it was handed fewer candles than the shape
    /// needs. **That is not a near miss, it is nothing at all.**
    pub fn touching(self, bars: &[&Bar]) -> Option<Decimal> {
        let last = bars.len().checked_sub(1)?;
        let first = bars.len().checked_sub(self.candles())?;

        Some(match self {
            Traded::Push { up: true } => bars[last].low,
            Traded::Push { up: false } => bars[last].high,

            Traded::Engulfing { .. } => bars[last].close,

            // The big candle is the one before the small one.
            Traded::Harami { up: true } => bars[first].low,
            Traded::Harami { up: false } => bars[first].high,

            // Where the run started.
            Traded::Marching { up: true } => bars[first].low,
            Traded::Marching { up: false } => bars[first].high,
        })
    }

    /// What the card calls it.
    pub fn name(self) -> &'static str {
        match self {
            Traded::Push { up: true } => "nsc-bull",
            Traded::Push { up: false } => "nsc-bear",
            Traded::Engulfing { up: true } => "bullish engulfing",
            Traded::Engulfing { up: false } => "bearish engulfing",
            Traded::Harami { up: true } => "bullish harami",
            Traded::Harami { up: false } => "bearish harami",
            Traded::Marching { up: true } => "three white soldiers",
            Traded::Marching { up: false } => "three black crows",
        }
    }

    /// Which way the shape itself points.
    pub fn is_up(self) -> bool {
        match self {
            Traded::Push { up }
            | Traded::Engulfing { up }
            | Traded::Harami { up }
            | Traded::Marching { up } => up,
        }
    }
}
