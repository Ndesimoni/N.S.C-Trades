//! The economic calendar, kept.
//!
//! **The file is the truth for the span it covers.** Each download replaces
//! what we hold for that stretch of the week: rows still listed are updated,
//! and rows that have fallen out of it are removed. See [`write`].

use chrono::{DateTime, Utc};
use nsc_core::news::{Event, Impact};

use super::{Store, StoreError};

/// How many events ride in one statement.
///
/// Postgres takes 65,535 parameters at most and each event uses eight, so the
/// hard ceiling is about 8,000. A week is around 150.
const BATCH: usize = 500;

/// Writes the week, and **drops anything that has fallen out of it**.
///
/// ## Why a plain upsert is not enough
///
/// The feed revises the week while it is running. A release that MOVES does
/// not edit its row — the time is part of the key, so it arrives as a new one
/// and the old row is left behind. Upsert alone would leave that ghost in
/// place, and the bot would warn him about a release at a time it is no longer
/// happening.
///
/// So every row inside the span this file covers is either refreshed by it or
/// deleted. `last_seen` is what tells the two apart: everything in the file
/// gets this moment stamped on it, and whatever inside the span still carries
/// an older stamp was not in the file.
///
/// **Only inside the span.** Last week's rows keep their old `last_seen` and
/// are never touched, which is what lets the record build up week over week.
///
/// One transaction, so a failure halfway cannot leave the week half replaced.
pub async fn write(store: &Store, events: &[Event], now: DateTime<Utc>) -> Result<u64, StoreError> {
    let Some(first) = events.iter().map(|event| event.at).min() else {
        // An empty file is not a reason to delete the week. It is a reason to
        // say nothing and keep what we had.
        return Ok(0);
    };

    let last = events.iter().map(|event| event.at).max().unwrap_or(first);

    let mut deal = store.begin().await.map_err(StoreError::from)?;
    let mut touched = 0;

    for chunk in events.chunks(BATCH) {
        let mut query = sqlx::QueryBuilder::new(
            "INSERT INTO news_events \
             (at, currency, title, impact, forecast, previous, first_seen, last_seen) ",
        );

        query.push_values(chunk, |mut row, event| {
            row.push_bind(event.at)
                .push_bind(&event.currency)
                .push_bind(&event.title)
                .push_bind(event.impact.name())
                .push_bind(&event.forecast)
                .push_bind(&event.previous)
                .push_bind(now)
                .push_bind(now);
        });

        // **first_seen is never overwritten.** "This was added on Wednesday"
        // is worth knowing later, and the row it belongs to may be revised
        // many times before it prints.
        query.push(
            " ON CONFLICT (at, currency, title) DO UPDATE SET \
               impact    = EXCLUDED.impact, \
               forecast  = EXCLUDED.forecast, \
               previous  = EXCLUDED.previous, \
               last_seen = EXCLUDED.last_seen",
        );

        touched += query
            .build()
            .execute(&mut *deal)
            .await
            .map_err(StoreError::from)?
            .rows_affected();
    }

    sqlx::query(
        "DELETE FROM news_events \
         WHERE at BETWEEN $1 AND $2 AND last_seen < $3",
    )
    .bind(first)
    .bind(last)
    .bind(now)
    .execute(&mut *deal)
    .await
    .map_err(StoreError::from)?;

    deal.commit().await.map_err(StoreError::from)?;

    Ok(touched)
}

/// Everything on the calendar between two moments, soonest first.
///
/// **The bot reads a window, not the week.** It only ever asks about what is
/// about to happen or has just happened, so a query that reads the lot would
/// carry rows nothing can use.
pub async fn between(
    store: &Store,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> Result<Vec<Event>, StoreError> {
    type Row = (DateTime<Utc>, String, String, String, String, String);

    let rows: Vec<Row> = sqlx::query_as(
        "SELECT at, currency, title, impact, forecast, previous \
         FROM news_events \
         WHERE at BETWEEN $1 AND $2 \
         ORDER BY at, currency, title",
    )
    .bind(from)
    .bind(to)
    .fetch_all(store)
    .await
    .map_err(StoreError::from)?;

    Ok(rows
        .into_iter()
        .map(|(at, currency, title, impact, forecast, previous)| Event {
            title,
            currency,
            at,
            // **Read back through `from_feed`, not matched on the stored
            // text.** One place turns a word into an `Impact`, so a spelling
            // that ever changes is a change in one function.
            impact: Impact::from_feed(&impact),
            forecast,
            previous,
        })
        .collect())
}

/// How many events are held. For `/status` and the tests.
pub async fn count(store: &Store) -> Result<i64, StoreError> {
    sqlx::query_scalar("SELECT COUNT(*) FROM news_events")
        .fetch_one(store)
        .await
        .map_err(StoreError::from)
}
