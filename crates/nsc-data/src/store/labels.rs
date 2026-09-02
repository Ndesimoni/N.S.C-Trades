//! **What he thought of a signal.** The half no measurement can supply.
//!
//! Candles can be downloaded again and outcomes recomputed from them. What he
//! thought of a setup on the afternoon it printed exists nowhere else the
//! moment he forgets it — which is why this is the one table here that cannot
//! be recreated, and why it fills itself from buttons rather than a sitting
//! nobody schedules.

use chrono::{DateTime, Utc};

use super::{Store, StoreError};

/// What the buttons say.
///
/// **Two, and there is no third.** *"Would have skipped"* is what he says
/// later, in words, once the outcome came in — a button for it would invite
/// him to answer before the market had.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Took,
    Skipped,
    /// Said afterwards, never tapped.
    WouldHaveSkipped,
}

impl Verdict {
    /// The words the table stores, and the ones the card shows.
    pub fn words(self) -> &'static str {
        match self {
            Verdict::Took => "took it",
            Verdict::Skipped => "skipped it",
            Verdict::WouldHaveSkipped => "would have skipped",
        }
    }

    /// Reads what a button sent back. **Unknown is `None`, never a guess** —
    /// a callback this code does not recognise must not be recorded as a
    /// verdict he never gave.
    pub fn from_button(word: &str) -> Option<Verdict> {
        match word {
            "took" => Some(Verdict::Took),
            "skipped" => Some(Verdict::Skipped),
            _ => None,
        }
    }
}

/// Records what he thought. **Says whether anything changed.**
///
/// ## Tapping twice must be harmless
///
/// Telegram RESENDS a callback when it does not hear back quickly enough, so
/// one tap arrives more than once. Without the upsert, a single tap would
/// become three rows saying the same thing.
///
/// ## And he is allowed to change his mind
///
/// Tapping the other button replaces the verdict rather than adding a row, and
/// `at` moves with it — so the record says when he settled rather than when he
/// first wavered.
///
/// **A note already written is kept** when only the verdict changes. He may
/// tap, explain in words, then change the tap; losing the explanation there
/// would be losing the only part that cannot be recovered.
pub async fn thought(
    store: &Store,
    signal_id: i64,
    verdict: &str,
    at: DateTime<Utc>,
) -> Result<bool, StoreError> {
    let done = sqlx::query(
        "INSERT INTO signal_labels (signal_id, at, verdict) \
         VALUES ($1, $2, $3) \
         ON CONFLICT ON CONSTRAINT one_verdict_per_signal DO UPDATE SET \
           verdict = EXCLUDED.verdict, \
           at      = EXCLUDED.at \
         WHERE signal_labels.verdict IS DISTINCT FROM EXCLUDED.verdict",
    )
    .bind(signal_id)
    .bind(at)
    .bind(verdict)
    .execute(store)
    .await
    .map_err(StoreError::from)?;

    Ok(done.rows_affected() > 0)
}

/// Adds his words to the verdict on a signal.
///
/// **It needs a verdict to attach to.** Says `false` when there is none — he
/// explained a setup he never judged, and inventing a verdict to hang the note
/// on would put a decision in the record that he did not make.
pub async fn because(store: &Store, signal_id: i64, note: &str) -> Result<bool, StoreError> {
    let done = sqlx::query("UPDATE signal_labels SET note = $2 WHERE signal_id = $1")
        .bind(signal_id)
        .bind(note)
        .execute(store)
        .await
        .map_err(StoreError::from)?;

    Ok(done.rows_affected() > 0)
}

/// The most recent signal he has been sent, whether or not he has judged it.
///
/// **For `/why` with no number.** He explains the one that just arrived far
/// more often than an old one, and asking him to quote an id would be asking
/// him to go and find it.
pub async fn newest_signal(store: &Store) -> Result<Option<i64>, StoreError> {
    sqlx::query_scalar("SELECT id FROM signals ORDER BY at DESC LIMIT 1")
        .fetch_optional(store)
        .await
        .map_err(StoreError::from)
}

/// What one signal was, in a line — for confirming a tap without making him
/// scroll back up to see which card he pressed.
pub async fn sentence_of(store: &Store, signal_id: i64) -> Result<Option<String>, StoreError> {
    sqlx::query_scalar("SELECT sentence FROM signals WHERE id = $1")
        .bind(signal_id)
        .fetch_optional(store)
        .await
        .map_err(StoreError::from)
}
