//! **What he thought of a signal** — the two buttons, and his words.
//!
//! The table that cannot be recreated. Candles can be downloaded again and
//! outcomes recomputed from them; what he thought of a setup on the afternoon
//! it printed exists nowhere else once he forgets it.

use chrono::{DateTime, Utc};

use super::super::{Noted, Store, because, sent, thought};
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

    assert_eq!(
        because(&db, id, "no verdict yet").await.expect("no error"),
        Noted::NoVerdict,
        "it says so rather than making a verdict up"
    );
}

/// **A note survives him changing his mind.** He may tap, explain, then change
/// the tap — and the explanation is the only part that cannot be recovered.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn his_words_survive_a_change_of_mind() {
    let db = deciding_store("TST/KEEPNOTE").await;
    let id = a_recorded_signal(&db, "TST/KEEPNOTE", "2026-09-03T14:00:00Z").await;

    thought(&db, id, "skipped it", moment("2026-09-03T14:05:00Z"))
        .await
        .expect("a verdict");

    assert_eq!(
        because(&db, id, "it was right on the weekly")
            .await
            .expect("a note"),
        Noted::Down,
        "the note lands"
    );

    // Still a skip, so the note has somewhere to stay.
    thought(
        &db,
        id,
        "would have skipped",
        moment("2026-09-03T14:30:00Z"),
    )
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

/// **Two signals on the same candle must not confuse `/why`.**
///
/// `at` is the moment the candle CLOSED, so two pairs finishing an hourly at
/// 15:00 carry exactly the same `at`. Ordered by `at` alone, Postgres picks
/// between them however it likes — and his note lands on whichever it chose.
///
/// This only became possible when `at` was fixed to the close. It used to be
/// `Utc::now()`, which differed by microseconds and hid it.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn the_newest_of_two_at_the_same_moment_is_the_one_written_last() {
    let db = deciding_store("TST/SAMETIME_A").await;
    let _ = deciding_store("TST/SAMETIME_B").await;

    // Both close at the same moment, which is the whole point.
    let same = "2026-09-04T15:00:00Z";

    let first = a_recorded_signal(&db, "TST/SAMETIME_A", same).await;
    let second = a_recorded_signal(&db, "TST/SAMETIME_B", same).await;

    assert!(second > first, "ids are handed out in the order written");

    // Ask ten times. Ordered by `at` alone this would wander between them.
    for _ in 0..10 {
        let newest = super::super::newest_signal(&db)
            .await
            .expect("should read")
            .expect("there are signals");

        assert!(
            newest >= second,
            "the tie must break the way he would — the one that arrived last"
        );
    }
}

/// **A reason is only for the ones he turned down.**
///
/// His call, 3 September 2026: *"if we take a setup there should be no why."*
/// Taking one means the rules were right and the sentence on the card already
/// says why; skipping means they produced something he did not want, and that
/// is the part no measurement can supply.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn a_setup_he_took_takes_no_reason() {
    let db = deciding_store("TST/NOWHY").await;
    let id = a_recorded_signal(&db, "TST/NOWHY", "2026-09-04T10:00:00Z").await;

    thought(&db, id, "took it", moment("2026-09-04T10:05:00Z"))
        .await
        .expect("a verdict");

    assert_eq!(
        because(&db, id, "no reason needed")
            .await
            .expect("no error"),
        Noted::HeTookIt,
        "it says so rather than writing a row nobody could interpret"
    );

    let note: Option<String> =
        sqlx::query_scalar("SELECT note FROM signal_labels WHERE signal_id = $1")
            .bind(id)
            .fetch_one(&db)
            .await
            .expect("the row");

    assert!(note.is_none(), "and nothing was written");
}

/// **Changing a skip to a take clears the reason.**
///
/// A reason for skipping something he then took is a reason for nothing. The
/// database refuses to hold one, so the write has to clear it rather than
/// leave a row that cannot be saved.
///
/// **This is the case that would have failed at runtime**, weeks later, on the
/// one afternoon he changed his mind after explaining himself.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn taking_one_he_had_explained_clears_the_reason() {
    let db = deciding_store("TST/CLEARED").await;
    let id = a_recorded_signal(&db, "TST/CLEARED", "2026-09-04T11:00:00Z").await;

    thought(&db, id, "skipped it", moment("2026-09-04T11:05:00Z"))
        .await
        .expect("a verdict");

    because(&db, id, "it ran into news")
        .await
        .expect("the reason");

    // He changes his mind and takes it after all.
    thought(&db, id, "took it", moment("2026-09-04T11:30:00Z"))
        .await
        .expect("must not be refused by the constraint");

    let (verdict, note): (String, Option<String>) =
        sqlx::query_as("SELECT verdict, note FROM signal_labels WHERE signal_id = $1")
            .bind(id)
            .fetch_one(&db)
            .await
            .expect("the row");

    assert_eq!(verdict, "took it");
    assert!(note.is_none(), "the reason went with the skip");
}

/// **The database refuses it outright**, not only the code.
///
/// It is a rule about what the data MEANS. A note on a setup he took is not a
/// mistake to tidy up later — it is a row nobody could interpret — so it must
/// be impossible however the row is written.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn the_table_itself_will_not_hold_one() {
    let db = deciding_store("TST/CONSTRAINT").await;
    let id = a_recorded_signal(&db, "TST/CONSTRAINT", "2026-09-04T12:00:00Z").await;

    thought(&db, id, "took it", moment("2026-09-04T12:05:00Z"))
        .await
        .expect("a verdict");

    let straight_in = sqlx::query("UPDATE signal_labels SET note = $2 WHERE signal_id = $1")
        .bind(id)
        .bind("going round the code")
        .execute(&db)
        .await;

    assert!(
        straight_in.is_err(),
        "the constraint has to stop this, not only `because`"
    );
}
