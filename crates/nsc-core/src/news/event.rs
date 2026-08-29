//! One thing the calendar says is coming.

use chrono::{DateTime, Utc};

use super::Impact;

/// A single entry on the economic calendar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    /// What it is called — "Core PCE Price Index m/m".
    pub title: String,

    /// **A currency, not a country**, however the feed labels the field. It
    /// arrives as `USD`, `EUR`, `AUD`, and once in a while `All` for
    /// something like Jackson Hole that belongs to no single one.
    ///
    /// Kept as the feed's own text. Turning it into a known list here would
    /// mean a currency he has just started trading arriving as "unknown".
    pub currency: String,

    /// When it prints, **in UTC**.
    ///
    /// The feed stamps these with a New York offset. It is converted once, on
    /// the way in, and is UTC everywhere after that — same rule as candles.
    pub at: DateTime<Utc>,

    pub impact: Impact,

    /// What the market expects, and what it was last time. Either can be
    /// empty — a speech has no number — and the card shows a dash rather than
    /// inventing one.
    pub forecast: String,
    pub previous: String,
}

impl Event {
    /// A name for this event that does not change between downloads.
    ///
    /// **This is what stops him being told twice.** The file is fetched every
    /// few hours and the same event is in every copy of it, so "have I already
    /// said this" has to survive a re-read.
    ///
    /// Time, currency and title together. The time alone is not enough —
    /// three Australian CPI numbers print in the same second — and the title
    /// alone is not either, because the same release comes round every month.
    pub fn key(&self) -> String {
        format!("{}|{}|{}", self.at.timestamp(), self.currency, self.title)
    }

    /// Does it have a number worth showing?
    ///
    /// A speech or a holiday has neither forecast nor previous, and a card
    /// that ruled a column for them looks broken rather than empty.
    pub fn has_numbers(&self) -> bool {
        !self.forecast.trim().is_empty() || !self.previous.trim().is_empty()
    }
}
