//! How a swing proves itself.

use std::path::Path;

use rust_decimal::Decimal;
use serde::Deserialize;
use thiserror::Error;

/// The four numbers, all of them shares of a move.
///
/// **Never a distance.** That is what lets the same four work on the 4-hour
/// and the daily, and on gold and EUR/USD.
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Rules {
    /// How much of the run must be given back for a peak to prove itself on
    /// its own. Half of *that* move, not half of a fixed distance.
    #[serde(with = "rust_decimal::serde::str")]
    pub confirm_retracement: Decimal,

    /// The shallower give-back that counts **once price has taken the peak
    /// out**.
    ///
    /// **This matters more than the one above.** A strong move barely pauses,
    /// so insisting on half would read structure fine in chop and go blind in
    /// a clean trend — which is exactly the market worth trading.
    #[serde(with = "rust_decimal::serde::str")]
    pub shallow_retracement: Decimal,

    /// How big a run must be next to recent ones, or it is not a move.
    #[serde(with = "rust_decimal::serde::str")]
    pub min_run_fraction: Decimal,

    /// How far back "recent" reaches, counted in runs.
    pub run_memory_legs: usize,
}

/// What can go wrong reading them.
#[derive(Debug, Error)]
pub enum RulesError {
    #[error("could not read {path}: {detail}")]
    CannotRead { path: String, detail: String },

    #[error("{path} is not a set of swing rules: {detail}")]
    NotRules { path: String, detail: String },
}

/// Read them from a file. **Gives up rather than guessing.**
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
