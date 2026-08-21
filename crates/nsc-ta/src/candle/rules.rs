//! The numbers that decide what a candle is called.
//!
//! **They live in `config/candles.toml`, not in the code.** Bury a threshold
//! inside a function called `is_doji` and changing your mind means changing
//! code and rebuilding. Keep it in a file and it is a restart.
//!
//! **These numbers are textbook, not his.** Taken as standard defaults so the
//! naming could be built, and they stay borrowed until a pair of charts — one
//! he took, one he passed that looked the same — replaces them.

use std::path::Path;

use rust_decimal::Decimal;
use serde::Deserialize;
use thiserror::Error;

/// Everything the naming needs, as shares of a candle's own height.
///
/// **Shares, not points.** A body that is a fifth of its candle is a fifth on
/// EURUSD and a fifth on gold. A points threshold works on the pair it was set
/// on and quietly stops working on every other one.
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Rules {
    pub body: Body,
    pub wick: Wick,
    pub rejection: Rejection,
}

/// How much of the candle is body.
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Body {
    /// At or under this and there is no body worth the name. A doji.
    pub doji: Decimal,

    /// At or under this the body is small — a spinning top, or the stub at the
    /// end of a rejection.
    pub small: Decimal,

    /// At or over this the body is nearly the whole candle.
    pub strong: Decimal,
}

/// How much of the candle is wick.
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Wick {
    /// At or under this, a wick counts as none at all.
    ///
    /// **It cannot be zero.** In real forex there is nearly always a tick or
    /// two, so "no wick" has to mean "almost none".
    pub missing: Decimal,

    /// The short end of a dragonfly or a gravestone.
    ///
    /// **Looser than `missing`, and it has to be.** "Almost nothing" beside a
    /// tail of 0.90 is not the same as "almost nothing" at the end of a
    /// marubozu — the clearest real dragonfly in three years carries 0.095
    /// above it, and judged by `missing` it came back a plain doji.
    pub stub: Decimal,

    /// At or over this, a wick is long.
    ///
    /// This is what separates a long-legged doji from a plain one, and a high
    /// wave from a spinning top.
    pub long: Decimal,
}

/// What makes a candle a rejection rather than just a small body.
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Rejection {
    /// How many times the body the long wick must be.
    pub tail_to_body: Decimal,

    /// The most the wick on the OTHER side may take.
    ///
    /// **This is what keeps the body at the far end from the tail.** Without
    /// it a candle with long wicks both ways passes — and that shape means
    /// close to the opposite thing: nobody won, rather than one side refused.
    pub nose: Decimal,
}

/// What can go wrong reading them.
#[derive(Debug, Error)]
pub enum RulesError {
    #[error("could not read {path}: {detail}")]
    CannotRead { path: String, detail: String },

    #[error("{path} is not a set of candle rules: {detail}")]
    NotRules { path: String, detail: String },
}

/// Read them from a file.
///
/// **Gives up rather than guessing.** A half-parsed rules file would name
/// candles by numbers nobody chose, and every pattern built on top inherits
/// it.
pub fn load(path: &Path) -> Result<Rules, RulesError> {
    let text = std::fs::read_to_string(path).map_err(|trouble| RulesError::CannotRead {
        path: path.display().to_string(),
        detail: trouble.to_string(),
    })?;

    toml::from_str(&text).map_err(|trouble| RulesError::NotRules {
        path: path.display().to_string(),
        detail: trouble.to_string(),
    })
}
