//! What the greeting remembers.
//!
//! Nothing here sends anything. What it remembers IS the behaviour — both bugs
//! this file exists to stop were bookkeeping, not sending.

use chrono::{DateTime, Utc};

use super::Awake;

fn session(text: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(text)
        .expect("a real moment")
        .with_timezone(&Utc)
}

/// **Bug one: it was once per RUN.**
///
/// A plain "have I greeted?" flag, set once and never cleared. A bot left
/// running from Friday greeted on the Friday and then never again — so the
/// Sunday open, after two days of silence, was exactly when the report was
/// worth most and exactly when it would not come.
#[test]
fn a_new_session_is_owed_a_new_report() {
    let mut awake = Awake::new();
    let friday = session("2026-08-21T21:00:00Z");
    let sunday = session("2026-08-23T21:00:00Z");

    awake.greeted.insert("XAU/USD".to_string(), friday);

    assert_eq!(awake.greeted.get("XAU/USD"), Some(&friday));
    assert_ne!(
        awake.greeted.get("XAU/USD"),
        Some(&sunday),
        "Sunday's session is not Friday's, so it is owed a report"
    );
}

/// **Bug two: it was once per BOT, not per pair.**
///
/// He sends a level mid-session and its bands are built fresh. He usually
/// draws a level BECAUSE price is near it — but the session was already
/// greeted, so nothing said price was sitting in the zone he had just drawn.
/// He got "your levels are live" and then silence.
#[test]
fn a_pair_built_fresh_is_owed_a_report_on_its_own() {
    let mut awake = Awake::new();
    let now = session("2026-08-18T21:00:00Z");

    awake.greeted.insert("XAU/USD".to_string(), now);
    awake.greeted.insert("GBP/USD".to_string(), now);

    awake.forget("GBP/USD");

    assert_eq!(
        awake.greeted.get("XAU/USD"),
        Some(&now),
        "the pair he did not touch is not announced twice"
    );
    assert_eq!(
        awake.greeted.get("GBP/USD"),
        None,
        "the one he sent a level for is owed a fresh report"
    );
}

/// Forgetting a pair that was never greeted is not an error. It happens on
/// every startup, when nothing has been greeted yet.
#[test]
fn forgetting_one_it_never_knew_is_fine() {
    let mut awake = Awake::new();

    awake.forget("EUR/USD");

    assert!(awake.greeted.is_empty());
}
