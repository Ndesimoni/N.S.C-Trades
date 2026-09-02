//! **`/why` — his reason, in his own words.**
//!
//! ```text
//!     /why the wick was into news, I stood aside
//! ```
//!
//! **Only on a setup he turned down.** His call, 3 September 2026: *"we should
//! only explain the why when we reject. If we take a setup there should be no
//! why."*
//!
//! It is the right way round. Taking a setup means the rules were right, and
//! the sentence on the card already says why. Skipping means they produced
//! something he did not want — and that reason is the one thing no measurement
//! can supply.
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
//! moment he forgets it. Those are the "don't take this" examples the Phase 4
//! model needs, and they are worthless without the reason.

use anyhow::Result;
use nsc_data::store::{self, Noted, Store};

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

    let about = store::sentence_of(record, signal_id)
        .await?
        .unwrap_or_else(|| "that setup".into());

    match store::because(record, signal_id, words).await? {
        Noted::Down => say(client, token, &format!("Noted, on:\n<b>{about}</b>"), None).await,

        // **It needs a verdict to attach to.** Inventing one to hang the note
        // on would put a decision in the record that he never made.
        Noted::NoVerdict => {
            say(
                client,
                token,
                "Tap <b>skipped it</b> under that setup first, then tell me why.",
                None,
            )
            .await
        }

        // **A reason is only for the ones he turned down.** His call,
        // 3 September 2026 — taking a setup means the rules were right, and
        // the sentence on the card already says why.
        Noted::HeTookIt => {
            say(
                client,
                token,
                &format!(
                    "You took that one, so there is nothing to explain:\n<b>{about}</b>\n\n\
                     A why is for the setups you turn down."
                ),
                None,
            )
            .await
        }
    }
}
