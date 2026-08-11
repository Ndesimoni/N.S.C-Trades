//! Three-letter currency codes.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::error::CoreError;

/// A three-letter currency code — USD, EUR, JPY.
///
/// A type rather than a bare string because of how the mistake shows up.
/// Write `"USDD"` into a plain string and nothing complains: the news filter
/// simply never matches any event, and the blackout rule silently does
/// nothing forever. No error, no crash — you would only notice months later
/// when the bot traded straight through a jobs number.
///
/// Checking it here means the typo is caught at startup, on the line where it
/// was written.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Currency(String);

impl Currency {
    /// Accepts any case and trims spaces, so `" usd "` from a config file
    /// works. Anything that is not three letters is refused.
    pub fn new(code: &str) -> Result<Self, CoreError> {
        let upper = code.trim().to_ascii_uppercase();

        let looks_right = upper.len() == 3 && upper.chars().all(|c| c.is_ascii_alphabetic());

        if looks_right {
            Ok(Self(upper))
        } else {
            Err(CoreError::InvalidCurrencyCode {
                text: code.to_string(),
            })
        }
    }

    pub fn code(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
