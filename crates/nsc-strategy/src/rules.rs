//! The settings, out of `config/strategy.toml`.

use std::path::Path;

use rust_decimal::Decimal;
use serde::Deserialize;
use thiserror::Error;

/// Everything rung 3 can be tuned by.
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Rules {
    /// How far outside a band still counts as at it, **as a share of that
    /// band's own thickness**.
    ///
    /// Half, settled with him on 25 August 2026.
    ///
    /// Never a distance. A band on gold is about 78 points and on the euro
    /// about 0.004 — one number in points would work on the pair it was set
    /// on and stop working on every other.
    #[serde(with = "rust_decimal::serde::str")]
    pub reach_of_band: Decimal,
}

/// What can go wrong reading them.
#[derive(Debug, Error)]
pub enum StrategyError {
    #[error("could not read {path}: {detail}")]
    CannotRead { path: String, detail: String },

    #[error("{path} is not a set of strategy rules: {detail}")]
    NotRules { path: String, detail: String },

    /// **A reach of nought would only ever fire on a shape whose tail tip
    /// landed exactly on the edge.** It parses, it looks deliberate, and it
    /// produces silence — which is indistinguishable from a quiet week.
    #[error("{path} sets reach_of_band to {reach}, which can almost never match")]
    NoReach { path: String, reach: Decimal },
}

/// Read them from a file. **Gives up rather than guessing.**
pub fn load(path: &Path) -> Result<Rules, StrategyError> {
    let text = std::fs::read_to_string(path).map_err(|trouble| StrategyError::CannotRead {
        path: path.display().to_string(),
        detail: trouble.to_string(),
    })?;

    let rules: Rules = toml::from_str(&text).map_err(|trouble| StrategyError::NotRules {
        path: path.display().to_string(),
        detail: trouble.to_string(),
    })?;

    if rules.reach_of_band < Decimal::ZERO {
        return Err(StrategyError::NoReach {
            path: path.display().to_string(),
            reach: rules.reach_of_band,
        });
    }

    Ok(rules)
}
