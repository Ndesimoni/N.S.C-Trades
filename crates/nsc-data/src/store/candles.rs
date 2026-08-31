//! The history everything else is measured against.

use chrono::{DateTime, Utc};
use nsc_core::candle::Bar;
use rust_decimal::Decimal;

use super::{Store, StoreError};
use crate::source::Interval;

/// Where a candle came from. **Two feeds disagree, and you have to be able to
/// tell which one you are holding.**
const SOURCE: &str = "ibkr";

/// Writes candles, **repairing rather than duplicating**.
///
/// `ON CONFLICT DO UPDATE`, and that matters more than it sounds: a backfill
/// that dies halfway is the normal case rather than the exception, and the fix
/// has to be "run it again".
///
/// Gives back how many rows were touched.
///
/// **One statement, not one per candle.** A three-year backfill is 30,000
/// candles; thirty thousand round trips takes minutes where one takes a
/// moment. They go over in batches so a single enormous statement never has to
/// be built either.
pub async fn write(
    store: &Store,
    symbol: &str,
    interval: Interval,
    bars: &[Bar],
) -> Result<u64, StoreError> {
    /// How many candles ride in one statement.
    ///
    /// Postgres takes 65,535 parameters at most and each candle uses eight, so
    /// the hard ceiling is about 8,000. A thousand is well under it and keeps
    /// any one statement small enough to read in a log.
    const BATCH: usize = 1_000;

    let mut touched = 0;

    for chunk in bars.chunks(BATCH) {
        let mut query = sqlx::QueryBuilder::new(
            "INSERT INTO candles \
             (symbol, interval, opened_at, open, high, low, close, source) ",
        );

        // **Anything that cannot be read as a time is skipped, not guessed.**
        // A candle with a broken stamp has nowhere to sit on a chart, and
        // putting it at the epoch would quietly rewrite the start of history.
        let readable: Vec<(&Bar, DateTime<Utc>)> = chunk
            .iter()
            .filter_map(|bar| bar.opened_at().ok().map(|at| (bar, at)))
            .collect();

        if readable.is_empty() {
            continue;
        }

        query.push_values(readable, |mut row, (bar, at)| {
            row.push_bind(symbol)
                .push_bind(interval.stored())
                .push_bind(at)
                .push_bind(bar.open)
                .push_bind(bar.high)
                .push_bind(bar.low)
                .push_bind(bar.close)
                .push_bind(SOURCE);
        });

        query.push(
            " ON CONFLICT (symbol, interval, opened_at) DO UPDATE SET \
               open = EXCLUDED.open, high = EXCLUDED.high, \
               low = EXCLUDED.low, close = EXCLUDED.close, \
               source = EXCLUDED.source",
        );

        touched += query.build().execute(store).await?.rows_affected();
    }

    Ok(touched)
}

/// Candles for one pair and timeframe, **oldest first** — the order a chart is
/// read in and the order the primary key stores.
///
/// `from` and `to` are inclusive of `from` and exclusive of `to`, so days can
/// be asked for back to back without one candle landing in both.
pub async fn read(
    store: &Store,
    symbol: &str,
    interval: Interval,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> Result<Vec<Bar>, StoreError> {
    let rows: Vec<(DateTime<Utc>, Decimal, Decimal, Decimal, Decimal)> = sqlx::query_as(
        "SELECT opened_at, open, high, low, close FROM candles \
         WHERE symbol = $1 AND interval = $2 AND opened_at >= $3 AND opened_at < $4 \
         ORDER BY opened_at",
    )
    .bind(symbol)
    .bind(interval.stored())
    .bind(from)
    .bind(to)
    .fetch_all(store)
    .await?;

    Ok(rows.into_iter().map(as_bar).collect())
}

/// How many candles are held for one pair and timeframe.
pub async fn count(store: &Store, symbol: &str, interval: Interval) -> Result<i64, StoreError> {
    let (many,): (i64,) =
        sqlx::query_as("SELECT count(*) FROM candles WHERE symbol = $1 AND interval = $2")
            .bind(symbol)
            .bind(interval.stored())
            .fetch_one(store)
            .await?;

    Ok(many)
}

/// The oldest candle held, if any.
pub async fn oldest(
    store: &Store,
    symbol: &str,
    interval: Interval,
) -> Result<Option<DateTime<Utc>>, StoreError> {
    edge(store, symbol, interval, "min").await
}

/// The newest candle held, if any. **What a backfill asks before deciding
/// where to start.**
pub async fn newest(
    store: &Store,
    symbol: &str,
    interval: Interval,
) -> Result<Option<DateTime<Utc>>, StoreError> {
    edge(store, symbol, interval, "max").await
}

/// Either end of what is held.
///
/// **`which` is never user input** — the two callers above pass a literal, and
/// nothing else may call this. It is private for that reason.
async fn edge(
    store: &Store,
    symbol: &str,
    interval: Interval,
    which: &str,
) -> Result<Option<DateTime<Utc>>, StoreError> {
    let sql = format!("SELECT {which}(opened_at) FROM candles WHERE symbol = $1 AND interval = $2");

    let (at,): (Option<DateTime<Utc>>,) = sqlx::query_as(&sql)
        .bind(symbol)
        .bind(interval.stored())
        .fetch_one(store)
        .await?;

    Ok(at)
}

/// A row, as the rest of the project already knows a candle.
///
/// **Never a raw row past this folder.** A table change stops here.
fn as_bar(
    (at, open, high, low, close): (DateTime<Utc>, Decimal, Decimal, Decimal, Decimal),
) -> Bar {
    Bar {
        datetime: at.format("%Y-%m-%d %H:%M:%S").to_string(),
        open,
        high,
        low,
        close,
    }
}
