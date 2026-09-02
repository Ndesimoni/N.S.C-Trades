//! **What he thought of a signal** — the two buttons, and his words.
//!
//! The table that cannot be recreated. Candles can be downloaded again and
//! outcomes recomputed from them; what he thought of a setup on the afternoon
//! it printed exists nowhere else once he forgets it.

use chrono::{DateTime, Utc};

use super::super::{Store, because, sent, thought};
use super::deciding::a_signal;
use super::support::deciding_store;

fn moment(text: &str) -> DateTime<Utc> {
    text.parse().expect("a real moment")
}

/// A signal to hang a verdict on, and its id.
async fn a_recorded_signal(db: &Store, symbol: &str, at: &str) -> i64 {
    sent(db, &a_signal(symbol, at))
        .await
        .expect("should write")
        .expect("a fresh signal has an id")
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn a_verdict_goes_in() {
    let db = deciding_store("TST/VERDICT").await;
    let id = a_recorded_signal(&db, "TST/VERDICT", "2026-09-03T10:00:00Z").await;

    let now = moment("2026-09-03T10:05:00Z");
    assert!(
        thought(&db, id, "took it", now)
            .await
            .expect("should write")
    );
}

/// **Tapping twice must be harmless.**
///
/// Telegram RESENDS a callback when it does not hear back quickly enough, so
/// one tap arrives more than once. Without the upsert, a single tap would
/// become three rows saying the same thing.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn the_same_tap_twice_changes_nothing() {
    let db = deciding_store("TST/TWICETAP").await;
    let id = a_recorded_signal(&db, "TST/TWICETAP", "2026-09-03T11:00:00Z").await;

    let now = moment("2026-09-03T11:05:00Z");

    assert!(thought(&db, id, "took it", now).await.expect("first"));
    assert!(
        !thought(&db, id, "took it", now).await.expect("second"),
        "the resend changed nothing"
    );
}

/// **He is allowed to change his mind**, and the change replaces rather than
/// adds — one verdict per signal, with `at` moving to when he settled.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn changing_his_mind_replaces_the_verdict() {
    let db = deciding_store("TST/CHANGED").await;
    let id = a_recorded_signal(&db, "TST/CHANGED", "2026-09-03T12:00:00Z").await;

    thought(&db, id, "took it", moment("2026-09-03T12:05:00Z"))
        .await
        .expect("first");

    assert!(
        thought(&db, id, "skipped it", moment("2026-09-03T12:30:00Z"))
            .await
            .expect("second"),
        "the other button is a real change"
    );

    let (verdict, at): (String, DateTime<Utc>) =
        sqlx::query_as("SELECT verdict, at FROM signal_labels WHERE signal_id = $1")
            .bind(id)
            .fetch_one(&db)
            .await
            .expect("one row, not two");

    assert_eq!(verdict, "skipped it");
    assert_eq!(at, moment("2026-09-03T12:30:00Z"), "when he settled");
}

/// **A note needs a verdict to attach to.** Inventing one to hang it on would
/// put a decision in the record that he never made.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn a_note_with_no_verdict_saves_nothing() {
    let db = deciding_store("TST/NONOTE").await;
    let id = a_recorded_signal(&db, "TST/NONOTE", "2026-09-03T13:00:00Z").await;

    assert!(
        !because(&db, id, "no verdict yet").await.expect("no error"),
        "it says so rather than making one up"
    );
}

/// **A note survives him changing his mind.** He may tap, explain, then change
/// the tap — and the explanation is the only part that cannot be recovered.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn his_words_survive_a_change_of_mind() {
    let db = deciding_store("TST/KEEPNOTE").await;
    let id = a_recorded_signal(&db, "TST/KEEPNOTE", "2026-09-03T14:00:00Z").await;

    thought(&db, id, "took it", moment("2026-09-03T14:05:00Z"))
        .await
        .expect("a verdict");

    assert!(
        because(&db, id, "it was right on the weekly")
            .await
            .expect("a note"),
        "the note lands"
    );

    thought(&db, id, "skipped it", moment("2026-09-03T14:30:00Z"))
        .await
        .expect("changed his mind");

    let note: Option<String> =
        sqlx::query_scalar("SELECT note FROM signal_labels WHERE signal_id = $1")
            .bind(id)
            .fetch_one(&db)
            .await
            .expect("the row");

    assert_eq!(
        note.as_deref(),
        Some("it was right on the weekly"),
        "his words are the part that cannot be recovered"
    );
}
