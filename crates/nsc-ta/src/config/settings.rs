//! All the settings, gathered in one place.

use serde::{Deserialize, Serialize};

use super::indicators::IndicatorSettings;
use super::swings::SwingSettings;
use crate::error::TaError;

/// Everything in `config/ta.toml` that has code behind it so far.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaSettings {
    pub swings: SwingSettings,
    pub indicators: IndicatorSettings,
}

impl TaSettings {
    /// Checks every setting. Call this once, when the program starts.
    ///
    /// Stops at the first problem rather than collecting them all. One
    /// wrong setting is enough to make the results meaningless, so there is
    /// nothing to be gained by carrying on.
    pub fn validate(&self) -> Result<(), TaError> {
        self.swings.validate()?;
        self.indicators.validate()?;
        Ok(())
    }
}
