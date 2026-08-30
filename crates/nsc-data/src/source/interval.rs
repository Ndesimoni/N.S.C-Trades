//! How long one candle lasts.
//!
//! **A type, not a string.** These used to be written out by hand at each call
//! site — `"1week"`, `"4h"`, `"1day"` — in whatever spelling the provider of
//! the day happened to want. A typo in one of those does not fail to compile.
//! It fails at the feed, at runtime, on one timeframe only, and the pair
//! quietly stops reporting while everything else carries on.

/// How long one candle lasts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Interval {
    Min5,
    Min15,
    Min30,
    H1,
    H4,
    Day,
    Week,
}

impl Interval {
    /// How many minutes it covers.
    ///
    /// **This says when the next one is due. It never says where a boundary
    /// falls.** Real boundaries come from the feed's own stamps — working them
    /// out here would mean knowing where IBKR puts its 4-hour candles, and
    /// guessing wrong reports a candle that has not finished.
    ///
    /// `Day` and `Week` are the nominal lengths. The trading day does not end
    /// at midnight and the trading week has five days in it, which is exactly
    /// why nothing may derive a boundary from these numbers.
    pub fn minutes(self) -> i64 {
        match self {
            Self::Min5 => 5,
            Self::Min15 => 15,
            Self::Min30 => 30,
            Self::H1 => 60,
            Self::H4 => 240,
            Self::Day => 1_440,
            Self::Week => 10_080,
        }
    }

    /// What to call it to a person — on a card, or in the terminal.
    /// **How it is written in the database, and only there.**
    ///
    /// Short, stable, and never the feed's own spelling. This project already
    /// paid for the alternative once: the timeframe travelled around as a
    /// `&'static str`, two spellings of the same thing became two different
    /// keys, and the same candle was reported twice.
    ///
    /// **`spoken` is for a person and this is for a row.** They must never be
    /// the same function — the day one gets prettier, every stored key would
    /// change and every old row would orphan.
    pub fn stored(self) -> &'static str {
        match self {
            Self::Min5 => "5m",
            Self::Min15 => "15m",
            Self::Min30 => "30m",
            Self::H1 => "1h",
            Self::H4 => "4h",
            Self::Day => "1d",
            Self::Week => "1w",
        }
    }

    /// Back from what the database holds.
    ///
    /// **Gives up rather than guessing.** An unknown spelling means a row this
    /// build does not understand, and inventing a timeframe for it would put
    /// candles on the wrong chart.
    pub fn from_stored(text: &str) -> Option<Self> {
        Some(match text {
            "5m" => Self::Min5,
            "15m" => Self::Min15,
            "30m" => Self::Min30,
            "1h" => Self::H1,
            "4h" => Self::H4,
            "1d" => Self::Day,
            "1w" => Self::Week,
            _ => return None,
        })
    }

    pub fn spoken(self) -> &'static str {
        match self {
            Self::Min5 => "5-minute",
            Self::Min15 => "15-minute",
            Self::Min30 => "30-minute",
            Self::H1 => "1-hour",
            Self::H4 => "4-hour",
            Self::Day => "daily",
            Self::Week => "weekly",
        }
    }
}
