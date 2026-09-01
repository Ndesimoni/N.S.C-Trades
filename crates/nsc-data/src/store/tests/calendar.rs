//! The economic calendar in the record.
//!
//! **Each test works in its own week.** They run in parallel against one
//! schema, so a shared week would have them stepping on each other — green
//! alone and red together, which is how the candle tests learned it.

use chrono::{DateTime, Utc};
use nsc_core::news::{Event, Impact};

use super::super::news;
use super::support::calendar_store;
fn an_event(title: &str, at: &str, impact: Impact) -> Event {
    Event {
        title: title.into(),
        currency: "USD".into(),
        at: at.parse().expect("a real moment"),
        impact,
        forecast: "55.2".into(),
        previous: "55.6".into(),
    }
}

fn moment(text: &str) -> DateTime<Utc> {
    text.parse().expect("a real moment")
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn a_week_goes_in_and_comes_back() {
    let db = calendar_store().await;
    let now = moment("2026-09-01T06:00:00Z");

    let week = vec![
        an_event(
            "ISM Manufacturing PMI",
            "2026-09-01T14:00:00Z",
            Impact::High,
        ),
        an_event("JOLTS Job Openings", "2026-09-01T14:30:00Z", Impact::Medium),
        an_event("Bank Holiday", "2026-09-01T20:00:00Z", Impact::Holiday),
    ];

    news::write(&db, &week, now).await.expect("should write");

    let back = news::between(
        &db,
        moment("2026-09-01T00:00:00Z"),
        moment("2026-09-01T23:00:00Z"),
    )
    .await
    .expect("should read");

    assert_eq!(back.len(), 3, "all three, holiday included");
    assert_eq!(back[0].title, "ISM Manufacturing PMI");
    assert_eq!(back[0].impact, Impact::High);
    assert_eq!(back[0].forecast, "55.2");

    // **The holiday matters.** The whole file is kept, not only the ratings he
    // wants a card for — which of those earn a message is config's job and it
    // must be changeable without a migration.
    assert_eq!(back[2].impact, Impact::Holiday);
}

/// **A revised forecast updates in place.** The feed publishes forecasts as
/// banks release theirs, so the same release arrives many times before it
/// prints.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn a_revision_updates_rather_than_duplicates() {
    let db = calendar_store().await;

    let mut event = an_event("Revised CPI", "2026-09-10T14:00:00Z", Impact::High);
    news::write(
        &db,
        std::slice::from_ref(&event),
        moment("2026-09-10T06:00:00Z"),
    )
    .await
    .expect("should write");

    event.forecast = "99.9".into();
    news::write(
        &db,
        std::slice::from_ref(&event),
        moment("2026-09-10T12:00:00Z"),
    )
    .await
    .expect("should write again");

    let back = news::between(
        &db,
        moment("2026-09-10T00:00:00Z"),
        moment("2026-09-10T23:00:00Z"),
    )
    .await
    .expect("should read");

    assert_eq!(back.len(), 1, "one release, not two");
    assert_eq!(back[0].forecast, "99.9", "the newer forecast");
}

/// **A release that MOVED must not leave a ghost behind.**
///
/// The time is part of the key, so a moved release arrives as a NEW row rather
/// than editing the old one. An upsert on its own would leave the old time
/// sitting there, and the bot would warn him about a release at a time it is
/// no longer happening. The delete inside `write` is what stops that, and this
/// is the test that would have caught its absence.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn a_release_that_moved_leaves_no_ghost() {
    let db = calendar_store().await;

    // Two events, so the span the file covers holds both the old time and the
    // new one — the delete only ever reaches inside that span.
    let anchor = an_event("Anchor", "2026-09-20T08:00:00Z", Impact::Low);
    let was = an_event("Rate Decision", "2026-09-20T14:00:00Z", Impact::High);

    news::write(&db, &[anchor.clone(), was], moment("2026-09-20T06:00:00Z"))
        .await
        .expect("should write");

    // The next download has it an hour later.
    let moved = an_event("Rate Decision", "2026-09-20T15:00:00Z", Impact::High);
    news::write(&db, &[anchor, moved], moment("2026-09-20T12:00:00Z"))
        .await
        .expect("should write again");

    let back = news::between(
        &db,
        moment("2026-09-20T00:00:00Z"),
        moment("2026-09-20T23:00:00Z"),
    )
    .await
    .expect("should read");

    let decisions: Vec<&Event> = back
        .iter()
        .filter(|one| one.title == "Rate Decision")
        .collect();

    assert_eq!(decisions.len(), 1, "the old time must be gone");
    assert_eq!(
        decisions[0].at,
        moment("2026-09-20T15:00:00Z"),
        "and it is the new one that survived"
    );
}

/// **An empty file deletes nothing.** A download that comes back with no
/// events is far likelier to be a bad reply than a week with no news in it,
/// and treating it as truth would wipe a week he is relying on.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn nothing_in_the_file_deletes_nothing() {
    let db = calendar_store().await;
    let now = moment("2026-10-01T06:00:00Z");

    let event = an_event("Still Here", "2026-10-01T14:00:00Z", Impact::High);
    news::write(&db, std::slice::from_ref(&event), now)
        .await
        .expect("should write");

    let touched = news::write(&db, &[], now).await.expect("should not fail");
    assert_eq!(touched, 0, "nothing written");

    let back = news::between(
        &db,
        moment("2026-10-01T00:00:00Z"),
        moment("2026-10-01T23:00:00Z"),
    )
    .await
    .expect("should read");

    assert_eq!(back.len(), 1, "the week survived");
}

/// **Last week is never touched.** The delete only reaches inside the span the
/// file covers, which is what lets the record build up week over week instead
/// of only ever holding the current one.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn an_older_week_is_left_alone() {
    let db = calendar_store().await;

    let old = an_event("Last Week", "2026-11-02T14:00:00Z", Impact::High);
    news::write(
        &db,
        std::slice::from_ref(&old),
        moment("2026-11-02T06:00:00Z"),
    )
    .await
    .expect("should write");

    let new = an_event("This Week", "2026-11-09T14:00:00Z", Impact::High);
    news::write(
        &db,
        std::slice::from_ref(&new),
        moment("2026-11-09T06:00:00Z"),
    )
    .await
    .expect("should write");

    let back = news::between(
        &db,
        moment("2026-11-01T00:00:00Z"),
        moment("2026-11-10T00:00:00Z"),
    )
    .await
    .expect("should read");

    assert_eq!(back.len(), 2, "both weeks are still there");
}
