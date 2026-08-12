//! From the `[structure]` section of `config/ta.toml`.

use serde::{Deserialize, Serialize};

use crate::error::TaError;

/// What it takes to call an old high properly taken out.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StructureSettings {
    /// How far past the old extreme price must carry, as a share of the run
    /// that made it — measured from the extreme, not from where the pullback
    /// began.
    ///
    /// A run from 1900 to 2100 is 200 points. At 0.4, price has to reach 2180
    /// before the high counts as taken.
    ///
    /// It is a floor, not a target. Carrying much further is the same answer
    /// arrived at more convincingly, and how far it went is kept on the break
    /// for the rules to weigh.
    pub min_follow_through: f64,
}

impl StructureSettings {
    pub fn validate(&self) -> Result<(), TaError> {
        if !self.min_follow_through.is_finite() || self.min_follow_through <= 0.0 {
            return Err(TaError::BadSetting {
                setting: "structure.min_follow_through".into(),
                value: self.min_follow_through.to_string(),
                why: "with no follow-through required, a one-point poke past an old \
                      high would count as taking it"
                    .into(),
            });
        }

        Ok(())
    }
}
