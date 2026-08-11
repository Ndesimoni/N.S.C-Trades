//! From the `[swings]` section of `config/ta.toml`.

use serde::{Deserialize, Serialize};

use crate::error::TaError;

/// How a swing proves itself.
///
/// Every number here is a **fraction of a move**, not a distance. That is why
/// one set of settings works on the 4-hour and the daily, and on gold and
/// EURUSD — there are no units to get wrong.
///
/// These are the most influential settings in the project. Levels, trendlines,
/// Fibonacci anchors and trend direction are all built from swing points, so
/// changing any of them moves everything at once.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SwingSettings {
    /// How much of the run must be given back for a peak to prove itself on
    /// its own.
    ///
    /// Half. A 200-point rally needs about 100 points back before its top is a
    /// swing high.
    pub confirm_retracement: f64,

    /// The shallower give-back that counts **once price takes the peak out**.
    ///
    /// A strong move barely pauses. Insisting on a deep pullback would read
    /// structure fine in chop and go blind in a clean trend, which is the
    /// market you most want to be reading.
    pub shallow_retracement: f64,

    /// How big a run must be next to recent ones, or it is not a move at all.
    ///
    /// Without this, half of a tiny run is a tinier pullback and a quiet
    /// afternoon fills with swings that are really just noise.
    pub min_run_fraction: f64,

    /// How far back "recent" reaches, counted in runs.
    ///
    /// Compared against only the last run, the test can ratchet downwards —
    /// 200, then 120, then 72, each one passing on its own while the chain
    /// shrinks to nothing. Remembering several stops that.
    pub run_memory_legs: usize,
}

impl SwingSettings {
    pub fn validate(&self) -> Result<(), TaError> {
        self.check_fraction("confirm_retracement", self.confirm_retracement)?;
        self.check_fraction("shallow_retracement", self.shallow_retracement)?;
        self.check_fraction("min_run_fraction", self.min_run_fraction)?;

        if self.shallow_retracement > self.confirm_retracement {
            return Err(TaError::BadSetting {
                setting: "swings.shallow_retracement".into(),
                value: self.shallow_retracement.to_string(),
                why: "the shallow give-back cannot be deeper than the one that \
                      confirms on its own, or the shallow route would never be used"
                    .into(),
            });
        }

        if self.run_memory_legs == 0 {
            return Err(TaError::BadSetting {
                setting: "swings.run_memory_legs".into(),
                value: "0".into(),
                why: "remembering no runs leaves nothing to measure the next one against".into(),
            });
        }

        Ok(())
    }

    /// Every setting here is a share of a move, so it lives between zero and
    /// one. Anything else means somebody typed a price or a pip count.
    fn check_fraction(&self, name: &str, value: f64) -> Result<(), TaError> {
        if !value.is_finite() || value <= 0.0 || value > 1.0 {
            return Err(TaError::BadSetting {
                setting: format!("swings.{name}"),
                value: value.to_string(),
                why: "must be a share of a move, above 0 and no more than 1".into(),
            });
        }

        Ok(())
    }
}
