//! From the `[indicators]` section of `config/ta.toml`.

use serde::{Deserialize, Serialize};

use crate::error::TaError;

/// Settings for the handful of indicators this system uses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IndicatorSettings {
    /// How many candles ATR averages over.
    ///
    /// ATR is the size of a normal candle, and it is the yardstick every
    /// other setting in this project is measured against. Change this and
    /// every distance in the system shifts with it.
    pub atr_period: usize,

    pub rsi_period: usize,
}

impl IndicatorSettings {
    pub fn validate(&self) -> Result<(), TaError> {
        if self.atr_period < 2 {
            return Err(TaError::BadSetting {
                setting: "indicators.atr_period".into(),
                value: self.atr_period.to_string(),
                why: "averaging needs at least two candles".into(),
            });
        }

        Ok(())
    }
}
