//! **What the bot saw, and what it refused**, against a real Postgres.
//!
//! These are the tests that would have caught `placing` — a reserved word in
//! Postgres, from `OVERLAY(... PLACING ...)`, which made the whole migration a
//! syntax error. It was written on an afternoon when Docker was down, so it
//! could not be run, and it shipped broken.
//!
//! **The queries here are checked at RUNTIME, not by the compiler.** That was
//! the trade for not making `cargo check` need a database. It means a typo in
//! SQL — or a column named after a keyword — is found on the first call, and
//! these are the first call.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde_json::json;

use super::super::{Seen, Turned, refused, sent};
use super::support::calendar_store;

fn moment(text: &str) -> DateTime<Utc> {
    text.parse().expect("a real moment")
}

fn d(text: &str) -> Decimal {
    text.parse().expect("a number")
}

fn a_signal(symbol: &str, at: &str) -> Seen {
    Seen {
        at: moment(at),
        spans_from: moment(at),
        candle_opened_at: moment(at),
        symbol: symbol.into(),
        interval: "1h".into(),
        shape: "bullish engulfing".into(),
        direction: "up".into(),
        band_timeframe: "daily".into(),
        band_price: d("0.71500"),
        sits: "inside".into(),
        broke_out: false,
        reach: d("2.3"),
        sentence: "bullish engulfing on AUD/USD 1h, in your daily 0.71500 zone".into(),
        features: json!({ "normal": "0.0004", "close": "0.71510" }),
        features_version: 1,
        rules_version: "abc123def456".into(),
        sent_at: Some(moment(at)),
    }
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn a_signal_goes_in() {
    let db = calendar_store().await;

    let row = a_signal("TST/SIGNAL", "2026-09-02T10:00:00Z");
    assert!(sent(&db, &row).await.expect("should write"), "written");
}

/// **The same signal twice is one row.** The look runs again on every poll
/// until the next candle closes, so this is the ordinary case — and a restart
/// must not re-send or re-record.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn the_same_signal_twice_is_one_row() {
    let db = calendar_store().await;

    let row = a_signal("TST/TWICE", "2026-09-02T11:00:00Z");

    assert!(sent(&db, &row).await.expect("should write"), "the first");
    assert!(
        !sent(&db, &row).await.expect("should not fail"),
        "the second says it was already there rather than failing"
    );
}

/// **A signal Telegram refused is still recorded.** The bot saw it, and a row
/// missing because a message failed would make the history disagree with what
/// the rules actually did.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn a_signal_that_did_not_send_is_still_kept() {
    let db = calendar_store().await;

    let mut row = a_signal("TST/UNSENT", "2026-09-02T12:00:00Z");
    row.sent_at = None;

    assert!(sent(&db, &row).await.expect("should write"));
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn a_refusal_goes_in_and_repeats_are_one_row() {
    let db = calendar_store().await;

    let row = Turned {
        at: moment("2026-09-02T13:00:00Z"),
        candle_opened_at: moment("2026-09-02T13:00:00Z"),
        symbol: "TST/REFUSED".into(),
        interval: "4h".into(),
        layer: "place".into(),
        why: "bullish engulfing printed at 1.2345, with no level near it".into(),
        features: json!({ "normal": "0.0004" }),
        features_version: 1,
        rules_version: "abc123def456".into(),
    };

    assert!(refused(&db, &row).await.expect("should write"), "the first");
    assert!(
        !refused(&db, &row).await.expect("should not fail"),
        "and the look runs again on every poll, so repeats must be harmless"
    );
}

/// **Every layer the code can produce must be one the table accepts.**
///
/// `Refused::layer` and the CHECK constraint are two lists of the same words
/// in two languages. A word in one and not the other fails at runtime, on the
/// first candle that hits it — which could be weeks after the change.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs Postgres — docker compose up -d"]
async fn every_layer_the_code_uses_is_one_the_table_allows() {
    let db = calendar_store().await;

    for (which, layer) in ["shape", "place", "measure"].iter().enumerate() {
        let row = Turned {
            at: moment("2026-09-02T14:00:00Z"),
            candle_opened_at: moment("2026-09-02T14:00:00Z"),
            symbol: format!("TST/LAYER{which}"),
            interval: "1d".into(),
            layer: (*layer).into(),
            why: "checking the constraint agrees with the code".into(),
            features: json!({}),
            features_version: 1,
            rules_version: "abc123def456".into(),
        };

        refused(&db, &row)
            .await
            .unwrap_or_else(|trouble| panic!("the table refused layer `{layer}`: {trouble}"));
    }
}
