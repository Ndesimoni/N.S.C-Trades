//! From the `[levels]` section of `config/ta.toml`.

use serde::{Deserialize, Serialize};

use crate::error::TaError;

/// How support and resistance get drawn.
///
/// All three of these decide what the bot *sees*. None of them decides what
/// to do about it — how many touches makes a level worth trading lives in
/// `config/strategy.toml`, because that is a trading opinion and this is the
/// chart-reading code.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LevelSettings {
    /// How thick a band is, as a fraction of a normal candle.
    ///
    /// Too thin and a wick that overshoots by a hair misses the level. Too
    /// thick and every price is at every level, so the check stops meaning
    /// anything.
    pub band_atr_multiple: f64,

    /// How many swing points must sit inside a band before it counts as a
    /// level at all.
    ///
    /// Two is the lowest that can mean anything — one swing point is just a
    /// swing point.
    pub min_touches: usize,

    /// How far back to look, in candles.
    ///
    /// Swings older than this are not considered. A price that turned the
    /// market three years ago and has not been near it since is history, not
    /// a level.
    pub max_age_bars: usize,
}

impl LevelSettings {
    pub fn validate(&self) -> Result<(), TaError> {
        if self.band_atr_multiple <= 0.0 || !self.band_atr_multiple.is_finite() {
            return Err(TaError::BadSetting {
                setting: "levels.band_atr_multiple".into(),
                value: self.band_atr_multiple.to_string(),
                why: "a band with no thickness catches nothing".into(),
            });
        }

        if self.min_touches < 2 {
            return Err(TaError::BadSetting {
                setting: "levels.min_touches".into(),
                value: self.min_touches.to_string(),
                why: "one swing point on its own is not a level".into(),
            });
        }

        if self.max_age_bars == 0 {
            return Err(TaError::BadSetting {
                setting: "levels.max_age_bars".into(),
                value: "0".into(),
                why: "looking back no candles at all finds nothing".into(),
            });
        }

        Ok(())
    }
}
