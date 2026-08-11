//! What kind of thing an instrument is.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::CoreError;

/// The kind of instrument. Matches the `class` field in
/// `config/symbols.toml`.
///
/// This is not decoration. Equity indices settle an hour earlier than the
/// forex day, indices and oil genuinely stop trading at night while forex
/// does not, and a gap at the open is not a candle. Code that needs to treat
/// those differently asks this.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetClass {
    Forex,
    Metal,
    Index,
    Energy,
}

impl fmt::Display for AssetClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            AssetClass::Forex => "forex",
            AssetClass::Metal => "metal",
            AssetClass::Index => "index",
            AssetClass::Energy => "energy",
        };
        f.write_str(text)
    }
}

impl FromStr for AssetClass {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "forex" => Ok(AssetClass::Forex),
            "metal" => Ok(AssetClass::Metal),
            "index" => Ok(AssetClass::Index),
            "energy" => Ok(AssetClass::Energy),
            _ => Err(CoreError::UnknownAssetClass {
                text: s.to_string(),
            }),
        }
    }
}
