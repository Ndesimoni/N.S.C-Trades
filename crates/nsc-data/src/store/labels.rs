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
/// **A note already written is kept** when only the verdict changes — unless
/// he changes it to *took it*, which the database will not allow a note on.
/// Going that way round clears it, because a reason for skipping something he
/// then took is a reason for nothing.
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
           at      = EXCLUDED.at, \
           note    = CASE WHEN EXCLUDED.verdict = 'took it' \
                          THEN NULL ELSE signal_labels.note END \
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

/// What came of trying to attach his words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Noted {
    /// Written.
    Down,

    /// He has not judged that setup yet, so there is nothing to attach to.
    /// **Inventing a verdict to hang the note on** would put a decision in the
    /// record that he never made.
    NoVerdict,

    /// **He took it, and a reason is only for the ones he turned down.**
    ///
    /// His call, 3 September 2026. Taking a setup means the rules were right
    /// and the sentence on the card already says why; skipping is the part no
    /// measurement can supply.
    HeTookIt,
}

/// Adds his words to the verdict on a signal.
///
/// **Only on a setup he turned down.** The database refuses the other case
/// outright — see `0006_why_is_for_skips.sql` — and this checks first so he
/// gets a sentence back rather than a constraint violation.
pub async fn because(store: &Store, signal_id: i64, note: &str) -> Result<Noted, StoreError> {
    let verdict: Option<String> =
        sqlx::query_scalar("SELECT verdict FROM signal_labels WHERE signal_id = $1")
            .bind(signal_id)
            .fetch_optional(store)
            .await
            .map_err(StoreError::from)?;

    let Some(verdict) = verdict else {
        return Ok(Noted::NoVerdict);
    };

    if verdict == Verdict::Took.words() {
        return Ok(Noted::HeTookIt);
    }

    sqlx::query("UPDATE signal_labels SET note = $2 WHERE signal_id = $1")
        .bind(signal_id)
        .bind(note)
        .execute(store)
        .await
        .map_err(StoreError::from)?;

    Ok(Noted::Down)
}

/// The most recent signal he has been sent, whether or not he has judged it.
///
/// **For `/why` with no number.** He explains the one that just arrived far
/// more often than an old one, and asking him to quote an id would be asking
/// him to go and find it.
///
/// ## Why `id` is in the ordering, and it has to be
///
/// `at` is the moment the candle CLOSED, not the moment the row was written.
/// So two signals on the same candle — two pairs, or two zones on one pair —
/// carry exactly the same `at`, and `ORDER BY at` alone picks between them
/// arbitrarily. His note would land on whichever Postgres felt like.
///
/// The id is handed out in the order rows were written, so it breaks the tie
/// the way he would: the one that arrived last.
///
/// **This only became possible when `at` was fixed.** It used to be
/// `Utc::now()`, which differed by microseconds between two rows and hid it.
pub async fn newest_signal(store: &Store) -> Result<Option<i64>, StoreError> {
    sqlx::query_scalar("SELECT id FROM signals ORDER BY at DESC, id DESC LIMIT 1")
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
