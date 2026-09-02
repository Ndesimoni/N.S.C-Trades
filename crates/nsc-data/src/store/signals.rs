//! What the bot saw, and what it refused.
//!
//! **Two writers, one dataset.** [`sent`] records a signal; [`refused`]
//! records a candle that had a shape and did not become one. They carry the
//! same `features`, because the only use either has is together — what to
//! take, and what not to.
//!
//! Both are add-only and both are idempotent: the look runs again on every
//! poll until the next candle closes, so the same row arrives many times.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde_json::Value;

use super::{Store, StoreError};

/// A signal, as the bot saw it, ready to be written.
///
/// **Everything here is what it saw at that moment.** Nothing is recomputed
/// later — see the migration for why that would quietly ruin a training set.
#[derive(Debug, Clone)]
pub struct Seen {
    pub at: DateTime<Utc>,
    pub spans_from: DateTime<Utc>,
    pub candle_opened_at: DateTime<Utc>,

    pub symbol: String,
    pub interval: String,

    pub shape: String,
    pub direction: String,

    pub band_timeframe: String,
    pub band_price: Decimal,
    /// Where it sits against the band — inside, just above, just below.
    ///
    /// **Not `placing`.** That is a reserved word in Postgres, from
    /// `OVERLAY(... PLACING ...)`, so the column cannot be called it.
    pub sits: String,
    pub broke_out: bool,
    pub reach: Decimal,

    pub sentence: String,
    pub features: Value,
    pub features_version: i16,
    pub rules_version: String,

    /// Null means Telegram refused it. **The bot still saw it.**
    pub sent_at: Option<DateTime<Utc>>,
}

/// A candle that had a shape on it and did not become a signal.
#[derive(Debug, Clone)]
pub struct Turned {
    pub at: DateTime<Utc>,
    pub candle_opened_at: DateTime<Utc>,

    pub symbol: String,
    pub interval: String,

    /// Which layer said no — `shape`, `place`, `measure`.
    pub layer: String,

    /// The specific test that failed, in one line.
    pub why: String,

    pub features: Value,
    pub features_version: i16,
    pub rules_version: String,
}

/// Writes a signal. **Gives back its id**, which the buttons need.
///
/// `None` means this exact signal was already recorded — one shape, one
/// candle, one zone, one row. That is the ordinary case on the second poll of
/// the same candle and is not a failure.
///
/// **The id is what a button carries.** Two setups on one pair within an hour
/// would otherwise be indistinguishable when he taps, and the label would land
/// on whichever the code guessed.
pub async fn sent(store: &Store, signal: &Seen) -> Result<Option<i64>, StoreError> {
    let id: Option<i64> = sqlx::query_scalar(
        "INSERT INTO signals \
         (at, spans_from, candle_opened_at, symbol, interval, \
          shape, shape_kind, direction, \
          band_timeframe, band_price, sits, broke_out, reach, \
          sentence, features, features_version, rules_version, sent_at) \
         VALUES ($1,$2,$3,$4,$5,$6,'candlestick',$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17) \
         ON CONFLICT ON CONSTRAINT one_signal_per_candle_per_zone DO NOTHING \
         RETURNING id",
    )
    .bind(signal.at)
    .bind(signal.spans_from)
    .bind(signal.candle_opened_at)
    .bind(&signal.symbol)
    .bind(&signal.interval)
    .bind(&signal.shape)
    .bind(&signal.direction)
    .bind(&signal.band_timeframe)
    .bind(signal.band_price)
    .bind(&signal.sits)
    .bind(signal.broke_out)
    .bind(signal.reach)
    .bind(&signal.sentence)
    .bind(&signal.features)
    .bind(signal.features_version)
    .bind(&signal.rules_version)
    .bind(signal.sent_at)
    .fetch_optional(store)
    .await
    .map_err(StoreError::from)?;

    Ok(id)
}

/// Writes a refusal. **Says whether it was new**, for the same reason.
pub async fn refused(store: &Store, turned: &Turned) -> Result<bool, StoreError> {
    let done = sqlx::query(
        "INSERT INTO rejections \
         (at, candle_opened_at, symbol, interval, layer, why, \
          features, features_version, rules_version) \
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) \
         ON CONFLICT ON CONSTRAINT one_rejection_per_candle_per_layer DO NOTHING",
    )
    .bind(turned.at)
    .bind(turned.candle_opened_at)
    .bind(&turned.symbol)
    .bind(&turned.interval)
    .bind(&turned.layer)
    .bind(&turned.why)
    .bind(&turned.features)
    .bind(turned.features_version)
    .bind(&turned.rules_version)
    .execute(store)
    .await
    .map_err(StoreError::from)?;

    Ok(done.rows_affected() > 0)
}

/// How many signals and how many refusals, for `/status` and the tests.
pub async fn tallies(store: &Store) -> Result<(i64, i64), StoreError> {
    let signals: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM signals")
        .fetch_one(store)
        .await
        .map_err(StoreError::from)?;

    let turned: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM rejections")
        .fetch_one(store)
        .await
        .map_err(StoreError::from)?;

    Ok((signals, turned))
}

/// The id of a signal already recorded, by what makes it unique.
///
/// **For asking about one a second time.** `sent` refuses a repeat, which is
/// right for the bot — one shape, one candle, one zone, one card. It is wrong
/// for a previewer, whose whole job is showing him a setup again, and which
/// still needs the id to put in the buttons.
pub async fn already(
    store: &Store,
    symbol: &str,
    interval: &str,
    band_price: Decimal,
    candle_opened_at: DateTime<Utc>,
) -> Result<Option<i64>, StoreError> {
    sqlx::query_scalar(
        "SELECT id FROM signals \
         WHERE symbol = $1 AND interval = $2 \
           AND band_price = $3 AND candle_opened_at = $4",
    )
    .bind(symbol)
    .bind(interval)
    .bind(band_price)
    .bind(candle_opened_at)
    .fetch_optional(store)
    .await
    .map_err(StoreError::from)
}
