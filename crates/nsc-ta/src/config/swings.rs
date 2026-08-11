//! From the `[swings]` section of `config/ta.toml`.

use serde::{Deserialize, Serialize};

use crate::error::TaError;

/// How sensitive swing detection is.
///
/// `lookback` is the most influential number in the whole project. Every
/// level, trendline, Fibonacci anchor and trend reading is built from swing
/// points, so changing it moves all of them at once.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SwingSettings {
    /// How many candles either side a swing must beat.
    ///
    /// Bigger means fewer, more meaningful swings. Smaller means noise.
    pub lookback: usize,

    /// Only let confirmed swings be used.
    ///
    /// Leave this true. It is the main protection against using a swing
    /// before you could have known it was one.
    pub require_confirmed: bool,

    /// How far a swing must stand out from its neighbours, as a fraction of
    /// a normal candle.
    ///
    /// Filters chop without a fixed pip number that would break on the next
    /// instrument.
    pub min_atr_multiple: f64,
}

impl SwingSettings {
    pub fn validate(&self) -> Result<(), TaError> {
        if self.lookback == 0 {
            return Err(TaError::BadSetting {
                setting: "swings.lookback".into(),
                value: "0".into(),
                why: "a swing must beat at least one candle on each side".into(),
            });
        }

        if self.min_atr_multiple < 0.0 || !self.min_atr_multiple.is_finite() {
            return Err(TaError::BadSetting {
                setting: "swings.min_atr_multiple".into(),
                value: self.min_atr_multiple.to_string(),
                why: "must be zero or more".into(),
            });
        }

        Ok(())
    }
}
