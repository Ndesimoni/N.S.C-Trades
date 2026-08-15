//! The calendar the real file describes, and a shorthand for a moment.

use chrono::{DateTime, Utc};

use super::super::Rules;

/// His calendar as `config/when.toml` has it.
///
/// **Written out rather than read from the file.** A test that loads the real
/// config passes or fails depending on what he changed this morning, which is
/// not what these are checking.
pub fn rules() -> Rules {
    toml::from_str(
        r#"
day_ends = "17:00"
timezone = "America/New_York"
silent_days = ["saturday", "sunday", "monday"]
no_new_trades = ["friday"]
settle_hours = 4
look_in_minutes = 20
heartbeat_at = "07:00"
trouble_after_minutes = 5
"#,
    )
    .expect("that is what the real file says")
}

pub fn utc(text: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(text)
        .expect("a real moment")
        .with_timezone(&Utc)
}
