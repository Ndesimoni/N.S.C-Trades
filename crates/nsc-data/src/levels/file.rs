//! The shape of a levels file on disk.

use serde::{Deserialize, Serialize};
use toml::value::Datetime;

/// One `config/levels/<PAIR>.toml` file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LevelsFile {
    /// Every level in the file. Named `level` because each one is written as
    /// its own `[[level]]` block.
    #[serde(default)]
    pub level: Vec<DrawnLevel>,
}

/// One level, exactly as it is written down.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawnLevel {
    /// `W1`, `D1` or `H4`. Which chart it belongs to, and it keeps that tag
    /// everywhere — a weekly level is still a weekly level on the 15-minute.
    pub timeframe: String,

    /// The price the line sits at.
    ///
    /// One number, not a top and a bottom. The band around it is worked out
    /// from a normal candle on that timeframe — see `band_bands` in
    /// `config/ta.toml`.
    ///
    /// That split is deliberate. The price is the thing he decided; the
    /// thickness is the same on every chart he draws, so it is a setting
    /// rather than something to record 100 times. It is also far easier to
    /// read one centre line off a screenshot than two edges.
    pub price: f64,

    /// The day it was drawn.
    ///
    /// **The level does not exist before this date.** Not drawn, not checked,
    /// not tradeable.
    ///
    /// A level drawn today knows what price did last year, so letting it act
    /// on last year's candles would make a backtest look better than anything
    /// that could have been traded. Running forward only is what stops that,
    /// and it costs nothing.
    ///
    /// Written the TOML way — `from = 2026-08-14`, unquoted. Kept as TOML's
    /// own date type rather than a chrono one, because chrono expects a quoted
    /// string and a bare date is what anyone editing this file would write.
    pub from: Datetime,

    /// Why it was drawn, in the trader's own words. Optional, and worth more
    /// than it looks — the notes are where a pattern will eventually show up.
    #[serde(default)]
    pub note: Option<String>,
}
