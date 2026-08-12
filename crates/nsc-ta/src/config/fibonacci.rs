//! From the `[fibonacci]` section of `config/ta.toml`.

use serde::{Deserialize, Serialize};

use crate::error::TaError;

/// Which retracement levels get drawn, and what each one is for.
///
/// Four levels, and **each has a different job** — which is why they are four
/// settings rather than one list. A level with no job attached is a line the
/// bot draws that nothing reads.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FibSettings {
    /// The zone that gets attention, and where to look to get in.
    ///
    /// The most important two. Shallow end first.
    pub golden_zone: [f64; 2],

    /// Not an entry level — a reading. A pullback this shallow means the
    /// market barely paused, which is what a strong trend looks like.
    ///
    /// The same number as `shallow_retracement` in `[swings]`, because it is
    /// the same belief.
    pub strong_trend_level: f64,

    /// Where stops get looked at. Not always, and never on its own: this
    /// crate draws the level and the invalidation layer decides whether the
    /// stop goes there.
    pub stop_level: f64,

    /// Used for targets, beyond the end of the move. **Not confirmed** — these
    /// are the textbook numbers rather than the trader's.
    pub extensions: [f64; 2],
}

impl FibSettings {
    pub fn validate(&self) -> Result<(), TaError> {
        let [shallow, deep] = self.golden_zone;

        for (name, value) in [
            ("golden_zone", shallow),
            ("golden_zone", deep),
            ("strong_trend_level", self.strong_trend_level),
            ("stop_level", self.stop_level),
        ] {
            if !value.is_finite() || value <= 0.0 || value >= 1.0 {
                return Err(TaError::BadSetting {
                    setting: format!("fibonacci.{name}"),
                    value: value.to_string(),
                    why: "a retracement is part of a move, so it sits between 0 and 1".into(),
                });
            }
        }

        if shallow >= deep {
            return Err(TaError::BadSetting {
                setting: "fibonacci.golden_zone".into(),
                value: format!("[{shallow}, {deep}]"),
                why: "the shallow edge of the zone comes first, and they cannot be equal".into(),
            });
        }

        if self.strong_trend_level >= shallow {
            return Err(TaError::BadSetting {
                setting: "fibonacci.strong_trend_level".into(),
                value: self.strong_trend_level.to_string(),
                why: "a strong trend turns back SHALLOWER than the zone — at or past it, \
                      the level says nothing the zone does not"
                    .into(),
            });
        }

        if self.stop_level <= deep {
            return Err(TaError::BadSetting {
                setting: "fibonacci.stop_level".into(),
                value: self.stop_level.to_string(),
                why: "the stop sits BEYOND the zone, or it would be hit by the entry \
                      it is supposed to protect"
                    .into(),
            });
        }

        Ok(())
    }
}
