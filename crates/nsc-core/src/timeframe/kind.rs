//! Which timeframes exist, and how long each one lasts.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::CoreError;

/// The timeframes this system works in.
///
/// The names match the strings in `config/app.toml` exactly, so they parse
/// straight out of the settings file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Timeframe {
    M15,
    M30,
    H1,
    H4,
    D1,
    W1,
}

impl Timeframe {
    /// How many minutes this candle lasts — but only for the intraday ones.
    ///
    /// Returns `None` for daily and weekly on purpose. A trading day is not
    /// reliably 24 hours: on the two days a year the clocks change it is 23
    /// or 25. A trading week is five days, not seven, because the market is
    /// shut at the weekend.
    ///
    /// Anything that wants to know where a daily or weekly candle starts has
    /// to go through [`super::DayBoundary`], which knows about all of that.
    /// Returning `None` here is what forces that.
    pub fn intraday_minutes(self) -> Option<i64> {
        match self {
            Timeframe::M15 => Some(15),
            Timeframe::M30 => Some(30),
            Timeframe::H1 => Some(60),
            Timeframe::H4 => Some(240),
            Timeframe::D1 | Timeframe::W1 => None,
        }
    }

    pub fn is_intraday(self) -> bool {
        self.intraday_minutes().is_some()
    }
}

impl fmt::Display for Timeframe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Timeframe::M15 => "M15",
            Timeframe::M30 => "M30",
            Timeframe::H1 => "H1",
            Timeframe::H4 => "H4",
            Timeframe::D1 => "D1",
            Timeframe::W1 => "W1",
        };
        f.write_str(text)
    }
}

impl FromStr for Timeframe {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "M15" => Ok(Timeframe::M15),
            "M30" => Ok(Timeframe::M30),
            "H1" => Ok(Timeframe::H1),
            "H4" => Ok(Timeframe::H4),
            "D1" => Ok(Timeframe::D1),
            "W1" => Ok(Timeframe::W1),
            _ => Err(CoreError::UnknownTimeframe {
                text: s.to_string(),
            }),
        }
    }
}
