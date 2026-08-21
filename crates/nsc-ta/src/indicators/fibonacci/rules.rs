//! The four numbers, and what each one is for.

use std::path::Path;

use rust_decimal::Decimal;
use serde::Deserialize;
use thiserror::Error;

/// Which shares matter, and what job each one has.
///
/// **A level with no job attached is a line the bot draws and nothing reads.**
/// That is why these are named rather than a list of ratios: `0.786` is not
/// "the fourth level", it is where a stop gets looked at.
#[derive(Debug, Clone)]
pub struct Rules {
    /// **The most important two.** Where to look to get in — price sitting
    /// between them is the thing to pay attention to.
    ///
    /// Kept as a pair, low then high, whichever order he wrote them in.
    pub zone: (Decimal, Decimal),

    /// **Not an entry level — a reading.** A pullback this shallow means the
    /// market barely paused, which is what a strong trend looks like.
    pub strong_trend: Decimal,

    /// **Where stops get LOOKED AT.** Not always, and never on its own — this
    /// crate draws the level and `nsc-strategy` decides whether the stop
    /// actually goes there. A stop placed by one line every time is a stop
    /// everybody can see.
    pub stop_level: Decimal,

    /// For targets. **Not confirmed** — the standard numbers, not his.
    pub extensions: Vec<Decimal>,
}

/// What can go wrong reading them.
#[derive(Debug, Error)]
pub enum RulesError {
    #[error("could not read {path}: {detail}")]
    CannotRead { path: String, detail: String },

    #[error("{path} is not a set of Fibonacci rules: {detail}")]
    NotRules { path: String, detail: String },

    #[error("the golden zone needs exactly two numbers, low then high")]
    ZoneNeedsTwo,
}

/// Read them from a file. **Gives up rather than guessing.**
pub fn load(path: &Path) -> Result<Rules, RulesError> {
    let text = std::fs::read_to_string(path).map_err(|trouble| RulesError::CannotRead {
        path: path.display().to_string(),
        detail: trouble.to_string(),
    })?;

    let raw: Wire = toml::from_str(&text).map_err(|trouble| RulesError::NotRules {
        path: path.display().to_string(),
        detail: trouble.to_string(),
    })?;

    let [low, high] = raw.golden_zone[..] else {
        return Err(RulesError::ZoneNeedsTwo);
    };

    Ok(Rules {
        zone: (low.min(high), low.max(high)),
        strong_trend: raw.strong_trend,
        stop_level: raw.stop_level,
        extensions: raw.extensions,
    })
}

/// The file as written, before the zone is turned into a pair.
#[derive(Deserialize)]
struct Wire {
    golden_zone: Vec<Decimal>,
    #[serde(with = "rust_decimal::serde::str")]
    strong_trend: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    stop_level: Decimal,
    extensions: Vec<Decimal>,
}
