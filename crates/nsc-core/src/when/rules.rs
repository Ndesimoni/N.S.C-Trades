//! Reading `config/when.toml`.

use std::path::Path;

use chrono::{NaiveTime, Weekday};
use chrono_tz::Tz;
use serde::Deserialize;
use thiserror::Error;

/// His trading calendar, as the file describes it.
#[derive(Debug, Clone, Deserialize)]
pub struct Rules {
    /// When a trading day ends, **in `timezone`** — never in UTC.
    pub day_ends: NaiveTime,

    /// The clock that boundary is on. `America/New_York`.
    ///
    /// Not a fixed offset, on purpose. 17:00 New York is 21:00 UTC in summer
    /// and 22:00 in winter, and an offset would be an hour out for half the
    /// year without ever saying so.
    #[serde(rename = "timezone")]
    pub zone: Tz,

    /// Days nothing is watched, fetched or sent.
    #[serde(default)]
    pub silent_days: Vec<Weekday>,

    /// Days that report what is happening but suggest no trade.
    #[serde(default)]
    pub no_new_trades: Vec<Weekday>,

    /// How long after a day opens before a trade may be suggested.
    #[serde(default = "four")]
    pub settle_hours: i64,

    /// How far into a forming candle before it may say what it is doing.
    #[serde(default = "twenty")]
    pub look_in_minutes: i64,

    /// When the heartbeat goes out, **in UTC**.
    ///
    /// Not on the New York clock like [`Rules::day_ends`]. That one is a market
    /// boundary; this is about when he looks at his phone, and it does not
    /// move with the seasons.
    #[serde(default = "seven")]
    pub heartbeat_at: NaiveTime,

    /// How long the price line must stay down before he is told.
    ///
    /// **Quiet about hiccups, loud about outages.** Most drops fix themselves
    /// in seconds, and a buzz for each one teaches him the buzz means nothing.
    #[serde(default = "five")]
    pub trouble_after_minutes: i64,
}

/// What `trouble_after_minutes` is when a file predates it.
fn five() -> i64 {
    5
}

/// What `heartbeat_at` is when a file predates it. Before London opens.
fn seven() -> NaiveTime {
    NaiveTime::from_hms_opt(7, 0, 0).unwrap_or_default()
}

fn four() -> i64 {
    4
}

fn twenty() -> i64 {
    20
}

/// What went wrong reading the calendar.
///
/// **Both are settled troubles.** A missing or malformed calendar does not
/// clear on its own, and retrying is the bot spinning while every boundary it
/// owns goes unenforced.
#[derive(Debug, Error)]
pub enum WhenError {
    #[error("cannot read the calendar at {path}: {detail}")]
    CannotRead { path: String, detail: String },

    #[error("{path} is not a calendar: {detail}")]
    NotReadable { path: String, detail: String },
}

pub fn load(path: &Path) -> Result<Rules, WhenError> {
    let text = std::fs::read_to_string(path).map_err(|trouble| WhenError::CannotRead {
        path: path.display().to_string(),
        detail: trouble.to_string(),
    })?;

    toml::from_str(&text).map_err(|trouble| WhenError::NotReadable {
        path: path.display().to_string(),
        detail: trouble.to_string(),
    })
}
