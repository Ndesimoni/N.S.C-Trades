//! The bits every test here needs.

use chrono::{DateTime, TimeZone, Utc};

use crate::news::{Event, Impact, Rules};

/// Nine in the morning, a Tuesday, and nothing special about it.
pub(super) fn nine() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 8, 25, 9, 0, 0)
        .single()
        .expect("25 August 2026 09:00 is a real moment")
}

pub(super) fn event(title: &str, at: DateTime<Utc>, impact: Impact) -> Event {
    Event {
        title: title.into(),
        currency: "USD".into(),
        at,
        impact,
        forecast: "90.3".into(),
        previous: "90.8".into(),
    }
}

/// The settings as `config/news.toml` ships them.
pub(super) fn rules() -> Rules {
    Rules {
        url: "https://example.invalid/calendar.json".into(),
        refresh_hours: 6,
        impacts: vec!["High".into(), "Medium".into()],
        warn_at_minutes: vec![30, 5],
        stale_minutes: 5,
    }
}
