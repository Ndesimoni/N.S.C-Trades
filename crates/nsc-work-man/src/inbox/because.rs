//! **`/why` — his reason, in his own words.**
//!
//! ```text
//!     /why the wick was into news, I stood aside
//! ```
//!
//! It attaches to the verdict on the **most recent signal**, because that is
//! nearly always the one he means. Asking him to quote a row id would be
//! asking him to go and find it, and a table that is a nuisance to fill is a
//! table that stays empty.
//!
//! ## Why this is worth a command at all
//!
//! The buttons say WHAT he did. This says WHY, and the why is the part that
//! cannot be recovered — an outcome can be recomputed from candles forever,
//! but *"I stood aside because it was into news"* exists nowhere else the
//! moment he forgets it.

use anyhow::Result;
use nsc_data::store::{self, Store};

use super::talking::say;

/// The word he types.
pub const WHY: &str = "/why";

/// Attaches his words to the verdict on the newest signal.
pub async fn note(
    client: &reqwest::Client,
    token: &str,
    text: &str,
    record: Option<&Store>,
) -> Result<()> {
    let words = text.trim_start_matches(WHY).trim();

    if words.is_empty() {
        return say(
            client,
            token,
            "Say why after it — <code>/why it was into the news</code>",
            None,
        )
        .await;
    }

    let Some(record) = record else {
        return say(client, token, "No database, so nothing was saved.", None).await;
    };

    let Some(signal_id) = store::newest_signal(record).await? else {
        return say(client, token, "No signals yet to explain.", None).await;
    };

    // **It needs a verdict to attach to.** Inventing one to hang the note on
    // would put a decision in the record that he never made.
    if !store::because(record, signal_id, words).await? {
        return say(
            client,
            token,
            "Tap <b>took it</b> or <b>skipped it</b> under that setup first, \
             then tell me why.",
            None,
        )
        .await;
    }

    let about = store::sentence_of(record, signal_id)
        .await?
        .unwrap_or_else(|| "that setup".into());

    say(client, token, &format!("Noted, on:\n<b>{about}</b>"), None).await
}
