//! The schema these tests run in, and what they share.

use nsc_core::candle::Bar;
use rust_decimal::Decimal;
use std::str::FromStr;

use super::super::{Store, open};

pub(super) fn d(text: &str) -> Decimal {
    Decimal::from_str(text).unwrap()
}

pub(super) fn bar(at: &str, open: &str, high: &str, low: &str, close: &str) -> Bar {
    Bar {
        datetime: at.into(),
        open: d(open),
        high: d(high),
        low: d(low),
        close: d(close),
    }
}

/// The schema the tests write to. **Never `public`, where the record lives.**
pub(super) const TEST_SCHEMA: &str = "testing";

/// Makes the test schema and migrates it — **exactly once per test run**.
///
/// **This is a first-run race, and it only shows on a fresh machine.** Five
/// tests each creating the schema and running the migrations at the same
/// moment fought over the migration lock and timed out; once the schema
/// existed they were fast enough to never collide. So it was green on the
/// second run and red on a machine that had never run them — the worst way
/// round for a test to fail.
///
/// `Once` cannot hold an async setup, and `block_on` inside a runtime panics,
/// so the work goes to a plain thread with a runtime of its own and everybody
/// waits for it.
static PREPARED: std::sync::Once = std::sync::Once::new();

pub(super) fn prepare(url: &str) {
    PREPARED.call_once(|| {
        let url = url.to_string();

        std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("a runtime for the setup");

            runtime.block_on(async {
                let first = sqlx::PgPool::connect(&url)
                    .await
                    .expect("is the database running?");

                sqlx::query(&format!("CREATE SCHEMA IF NOT EXISTS {TEST_SCHEMA}"))
                    .execute(&first)
                    .await
                    .expect("could not make the test schema");

                first.close().await;

                // Migrate it here, so no test has to.
                open(&in_schema(&url))
                    .await
                    .expect("could not migrate the test schema");
            });
        })
        .join()
        .expect("the setup thread");
    });
}

/// The same url, pointed at the tests' schema.
///
/// **`options=-c search_path=...` puts every statement there**, migrations
/// included — so the tables the tests use are the tests' own.
pub(super) fn in_schema(url: &str) -> String {
    let sep = if url.contains('?') { '&' } else { '?' };

    format!("{url}{sep}options=-c%20search_path%3D{TEST_SCHEMA}")
}

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
/// **A POOL PER TEST, NOT ONE SHARED.** A shared `static` pool was tried and it
/// timed out at random: `#[tokio::test]` gives every test its own runtime, and
/// a pool belongs to the runtime that made it. Borrowed across runtimes it
/// waits forever for a connection nobody is driving.
///
/// **EVERY TEST ALSO GETS ITS OWN SYMBOL.** They shared one at first and each
/// cleared it on the way in, so in parallel they wiped each other — green
/// alone and red together.
pub(super) async fn store(symbol: &str) -> Store {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL — see .env.example");

    prepare(&url);

    let db = open(&in_schema(&url))
        .await
        .expect("could not open the test schema");

    sqlx::query("DELETE FROM candles WHERE symbol = $1")
        .bind(symbol)
        .execute(&db)
        .await
        .expect("could not clear the test rows");

    db
}

/// The test schema, opened, with **nothing cleared**.
///
/// The calendar tests each work in their own week rather than sharing one and
/// wiping it, so there is nothing to clear — and a shared wipe is exactly what
/// made the candle tests green alone and red together.
pub(super) async fn calendar_store() -> Store {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL — see .env.example");

    prepare(&url);

    open(&in_schema(&url))
        .await
        .expect("could not open the test schema")
}

/// The test schema, with **this symbol's decisions cleared**.
///
/// The candle tests learned this the hard way and the calendar tests dodge it
/// by each working in its own week. These could not: `sent` is idempotent by
/// design, so the second run of a test finds its own row from the first run
/// and is told "already there" — green once, red forever after.
///
/// **Labels go with the signal**, by `ON DELETE CASCADE`, so clearing signals
/// is enough.
pub(super) async fn deciding_store(symbol: &str) -> Store {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL — see .env.example");

    prepare(&url);

    let db = open(&in_schema(&url))
        .await
        .expect("could not open the test schema");

    for table in ["signals", "rejections"] {
        sqlx::query(&format!("DELETE FROM {table} WHERE symbol = $1"))
            .bind(symbol)
            .execute(&db)
            .await
            .expect("could not clear the test rows");
    }

    db
}
