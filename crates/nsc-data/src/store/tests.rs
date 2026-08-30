//! Tests that need a real Postgres.
//!
//! ```text
//!     docker compose up -d
//!     cargo test -p nsc-data -- --ignored
//! ```
//!
//! ## Why every one of them is `#[ignore]`
//!
//! The queries here are checked at RUNTIME, not by the compiler — that was the
//! trade for not making `cargo check` need a database. So a typo in SQL is
//! found on the first call, and these are the first call.
//!
//! They cannot run in the ordinary suite, because the ordinary suite has to
//! pass on a machine with no container.
//!
//! **And they must not quietly pass when there is no database.** A test that
//! skips itself and reports green pins nothing at all — which is the thing
//! `CLAUDE.md` says to check for. `#[ignore]` says "not run" out loud;
//! skipping inside the body would say "passed".

use chrono::{TimeZone, Utc};
use nsc_core::candle::Bar;
use rust_decimal::Decimal;
use std::str::FromStr;

use super::*;
use crate::source::Interval;

fn d(text: &str) -> Decimal {
    Decimal::from_str(text).unwrap()
}

fn bar(at: &str, open: &str, high: &str, low: &str, close: &str) -> Bar {
    Bar {
        datetime: at.into(),
        open: d(open),
        high: d(high),
        low: d(low),
        close: d(close),
    }
}

/// The schema the tests write to. **Never `public`, where the record lives.**
const TEST_SCHEMA: &str = "testing";

/// Opens the record in a schema of the tests' own, and clears one test's rows.
///
/// **A SEPARATE SCHEMA, AND THAT IS NOT FUSSINESS.** The first version wrote
/// to `public` and only cleared on the way IN, so `TST/ROUNDTRIP` and friends
/// piled up in the record beside his candles. The record is meant to be the
/// truth; a fake pair in it gets counted by something eventually.
///
/// **A schema rather than a second database**, because the bot's own role owns
/// this database and can make one — where `CREATE DATABASE` needs a privilege
/// nothing else here needs. The bot connects with the least it can do the job
/// with, and the tests must not be the reason that stops being true.
///
/// **A POOL PER TEST, NOT ONE SHARED.** A shared `static` pool was tried and
/// it timed out at random: `#[tokio::test]` gives every test its own runtime,
/// and a pool belongs to the runtime that made it. Borrowed across runtimes it
/// waits forever for a connection nobody is driving.
///
/// **EVERY TEST ALSO GETS ITS OWN SYMBOL.** They shared one at first and each
/// cleared it on the way in, so in parallel they wiped each other — green
/// alone and red together, which is the worst way round for a test to fail.
async fn store(symbol: &str) -> Store {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL — see .env.example");

    // The schema has to exist before anything can be pointed at it. Racing
    // here is fine: `IF NOT EXISTS` is the whole point, and `sqlx` takes its
    // own lock while migrating.
    let first = sqlx::PgPool::connect(&url)
        .await
        .expect("is the database running?");

    sqlx::query(&format!("CREATE SCHEMA IF NOT EXISTS {TEST_SCHEMA}"))
        .execute(&first)
        .await
        .expect("could not make the test schema");

    first.close().await;

    // **`options=-c search_path=...` puts every statement in that schema**,
    // migrations included — so the tables the tests use are the tests' own.
    let sep = if url.contains('?') { '&' } else { '?' };

    let db = open(&format!("{url}{sep}options=-c%20search_path%3D{TEST_SCHEMA}"))
        .await
        .expect("could not open the test schema");

    sqlx::query("DELETE FROM candles WHERE symbol = $1")
        .bind(symbol)
        .execute(&db)
        .await
        .expect("could not clear the test rows");

    db
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn candles_go_in_and_come_back_the_same() {
    const PAIR: &str = "TST/ROUNDTRIP";
    let db = store(PAIR).await;

    let written = [
        bar("2026-01-01 00:00:00", "1.1", "1.3", "1.0", "1.2"),
        bar("2026-01-01 01:00:00", "1.2", "1.4", "1.15", "1.35"),
    ];

    write(&db, PAIR, Interval::H1, &written).await.unwrap();

    let back = read(
        &db,
        PAIR,
        Interval::H1,
        Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2026, 1, 2, 0, 0, 0).unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(back.len(), 2);
    assert_eq!(back[0].datetime, "2026-01-01 00:00:00");
    assert_eq!(back[0].high, d("1.3"));

    // **NUMERIC, not float.** 1.15 stored as 1.1499999 would answer "did price
    // touch it" with no while his eye says yes.
    assert_eq!(back[1].low, d("1.15"));
}

/// **Running a backfill twice repairs it rather than duplicating it**, and a
/// backfill that dies halfway is the normal case rather than the exception.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn writing_twice_repairs_rather_than_duplicates() {
    const PAIR: &str = "TST/REPAIR";
    let db = store(PAIR).await;

    let first = [bar("2026-02-01 00:00:00", "2.0", "2.5", "1.9", "2.4")];
    write(&db, PAIR, Interval::H4, &first).await.unwrap();

    // The same candle, corrected — as a re-download would hand it over.
    let again = [bar("2026-02-01 00:00:00", "2.0", "2.6", "1.9", "2.45")];
    write(&db, PAIR, Interval::H4, &again).await.unwrap();

    assert_eq!(count(&db, PAIR, Interval::H4).await.unwrap(), 1);

    let back = read(
        &db,
        PAIR,
        Interval::H4,
        Utc.with_ymd_and_hms(2026, 2, 1, 0, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2026, 2, 2, 0, 0, 0).unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(back[0].high, d("2.6"), "the second write must correct the first");
}

/// **A timeframe is one key, not two.** The 1-hour and the 4-hour share a
/// symbol and must never share a row.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn timeframes_do_not_collide() {
    const PAIR: &str = "TST/TIMEFRAMES";
    let db = store(PAIR).await;

    let one = [bar("2026-03-01 00:00:00", "3.0", "3.1", "2.9", "3.05")];
    write(&db, PAIR, Interval::H1, &one).await.unwrap();
    write(&db, PAIR, Interval::H4, &one).await.unwrap();

    assert_eq!(count(&db, PAIR, Interval::H1).await.unwrap(), 1);
    assert_eq!(count(&db, PAIR, Interval::H4).await.unwrap(), 1);
}

/// Nothing held is `None`, **not a zero time**. A backfill asks this to decide
/// where to start, and an epoch would send it to 1970.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn nothing_held_is_nothing_not_a_date() {
    const PAIR: &str = "TST/EMPTY";
    let db = store(PAIR).await;

    assert!(newest(&db, PAIR, Interval::Week).await.unwrap().is_none());
    assert!(oldest(&db, PAIR, Interval::Week).await.unwrap().is_none());
    assert_eq!(count(&db, PAIR, Interval::Week).await.unwrap(), 0);
}

/// **The stored spelling is not the spoken one, and they must never merge.**
/// The day `spoken` gets prettier, every stored key would change and every old
/// row would orphan.
#[test]
fn stored_and_spoken_are_different_words() {
    for interval in [Interval::H1, Interval::H4, Interval::Day, Interval::Week] {
        assert_ne!(interval.stored(), interval.spoken());
        assert_eq!(Interval::from_stored(interval.stored()), Some(interval));
    }

    assert_eq!(Interval::from_stored("1-hour"), None, "the spoken one is not a key");
}

/// **The bot reads UTC, whatever the machine's clock says.**
///
/// **This does not pin a line of ours, and that is worth saying.** `sqlx`
/// sends `TimeZone=UTC` on every connection it opens, so this holds because of
/// the driver rather than because of anything here. It was checked: with our
/// own `SET TIME ZONE` removed and the database default reset, it still
/// passed.
///
/// It stays because it pins the BEHAVIOUR. If `sqlx` ever changes, or someone
/// adds a connection option that overrides it, the bot would start reading the
/// record in local time and nothing else would notice.
///
/// Found on 30 August 2026 the other way round: the first candle showed as
/// `2010-02-12 08:00:00+04` in `psql`, because the Mac is on Asia/Dubai and
/// Postgres.app inherits it. It opened at 04:00 UTC and the file said so —
/// the data was right and the screen was not. Nothing had shifted, because
/// `TIMESTAMPTZ` holds an absolute instant. But the first person to read that
/// screen would have believed a 4-hour candle opened four hours late and gone
/// hunting for a bug in the feed. Migration 0002 is for them.
#[tokio::test]
#[ignore = "needs Postgres — see the README"]
async fn every_connection_is_utc() {
    const PAIR: &str = "TST/CLOCK";
    let db = store(PAIR).await;

    let (zone,): (String,) = sqlx::query_as("SHOW TIME ZONE")
        .fetch_one(&db)
        .await
        .unwrap();

    assert_eq!(zone, "UTC", "the bot must never read the record in local time");

    // And a candle comes back on the instant it was written, not shifted.
    write(&db, PAIR, Interval::H4, &[bar("2010-02-12 04:00:00", "1", "2", "0.5", "1.5")])
        .await
        .unwrap();

    let back = read(
        &db,
        PAIR,
        Interval::H4,
        Utc.with_ymd_and_hms(2010, 2, 12, 0, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2010, 2, 13, 0, 0, 0).unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(back[0].datetime, "2010-02-12 04:00:00");
}
