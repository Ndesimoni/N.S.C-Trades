//! How hard an event is expected to hit.

/// The rating the calendar puts on an event.
///
/// **`Unknown` is deliberate and it is the safe direction.** The feed writes
/// this as a word, and a word nobody has seen before must not be guessed at.
/// It never matches anything in `impacts`, so a new rating goes quiet rather
/// than arriving on his phone as though it were a rate decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Impact {
    High,
    Medium,
    Low,

    /// A bank holiday. The calendar carries these and they are not releases —
    /// there is no number, and nothing prints.
    Holiday,

    /// Something this code has not been taught.
    Unknown,
}

impl Impact {
    /// Reads the word the feed sent.
    ///
    /// Case is ignored because it costs nothing and the feed is not ours.
    pub fn from_feed(word: &str) -> Impact {
        match word.trim().to_ascii_lowercase().as_str() {
            "high" => Impact::High,
            "medium" => Impact::Medium,
            "low" => Impact::Low,
            "holiday" => Impact::Holiday,
            _ => Impact::Unknown,
        }
    }

    /// The word, spelled the way `config/news.toml` spells it.
    ///
    /// **The settings match on this**, so the two have to agree. Writing the
    /// config check against the raw feed text instead would mean a change of
    /// case at their end silently emptying his filter.
    pub fn name(self) -> &'static str {
        match self {
            Impact::High => "High",
            Impact::Medium => "Medium",
            Impact::Low => "Low",
            Impact::Holiday => "Holiday",
            Impact::Unknown => "Unknown",
        }
    }

    /// The colour it wears on a card.
    ///
    /// **ForexFactory's spelling, not a design choice.** He reads that
    /// calendar every day; red meaning anything other than "high" here would
    /// make the card disagree with the site it came from.
    pub fn colour(self) -> &'static str {
        match self {
            Impact::High => "red",
            Impact::Medium => "orange",
            Impact::Low => "yellow",
            Impact::Holiday | Impact::Unknown => "grey",
        }
    }
}
