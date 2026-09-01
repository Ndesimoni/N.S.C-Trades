//! The settings, read out of `config/news.toml`.

use std::path::Path;

use serde::Deserialize;
use thiserror::Error;

use super::Impact;

/// Everything the news watcher can be tuned by.
#[derive(Debug, Clone, Deserialize)]
pub struct Rules {
    /// Where the week's file comes from.
    ///
    /// **A string this crate only carries.** `nsc-core` cannot reach anything
    /// — it has no `reqwest` — so holding an address is all it does with it.
    /// `nsc-data` is what fetches. One file of settings, read in one place,
    /// beats two loaders that drift.
    pub url: String,

    /// How often to download it again.
    pub refresh_hours: i64,

    /// Which ratings earn a message, spelled as the calendar spells them.
    pub impacts: Vec<String>,

    /// **How many minutes before an event to say something, and it is a
    /// list.**
    ///
    /// `[5, 1]` is a heads-up five minutes out and a last call one minute
    /// out. His ask, 1 September 2026: *"we are going to have five minutes and
    /// one minute."*
    ///
    /// It was `[30, 5]` for a few hours and thirty was too early — *"thirty
    /// minutes is really fast."* A warning that far ahead gets read, filed and
    /// forgotten before the number prints.
    ///
    /// One mark is live at a time — see `due.rs`. The card each one draws is
    /// the same card; it simply says a different number of minutes, because it
    /// works that out from the clock rather than from the setting.
    ///
    /// Order does not matter, and a repeat is harmless. Both are tidied in
    /// `due.rs` rather than trusted from the file.
    pub warn_at_minutes: Vec<i64>,

    /// How long after one it is still worth mentioning. See `due.rs` — this is
    /// the edge that makes a restart survivable.
    pub stale_minutes: i64,
}

impl Rules {
    /// Is this rating on his list?
    ///
    /// **Compared without case, and against the impact's own spelling.**
    /// Matching the feed's raw text instead would let a change of case at
    /// their end quietly empty the filter — and an empty filter is silence,
    /// which looks exactly like a quiet week.
    pub fn wants(&self, impact: Impact) -> bool {
        self.impacts
            .iter()
            .any(|wanted| wanted.trim().eq_ignore_ascii_case(impact.name()))
    }
}

/// What can go wrong reading them.
#[derive(Debug, Error)]
pub enum NewsError {
    #[error("could not read {path}: {detail}")]
    CannotRead { path: String, detail: String },

    #[error("{path} is not a set of news rules: {detail}")]
    NotRules { path: String, detail: String },

    #[error("{path} lists no impacts, so nothing could ever be sent")]
    NoImpacts { path: String },
}

/// Read them from a file. **Gives up rather than guessing.**
///
/// An empty `impacts` list is refused rather than accepted. It parses
/// perfectly and means "never say anything", which is indistinguishable from
/// a quiet week — so it fails at startup, where it can still be seen.
pub fn load(path: &Path) -> Result<Rules, NewsError> {
    let text = std::fs::read_to_string(path).map_err(|trouble| NewsError::CannotRead {
        path: path.display().to_string(),
        detail: trouble.to_string(),
    })?;

    let rules: Rules = toml::from_str(&text).map_err(|trouble| NewsError::NotRules {
        path: path.display().to_string(),
        detail: trouble.to_string(),
    })?;

    if rules.impacts.is_empty() {
        return Err(NewsError::NoImpacts {
            path: path.display().to_string(),
        });
    }

    Ok(rules)
}
