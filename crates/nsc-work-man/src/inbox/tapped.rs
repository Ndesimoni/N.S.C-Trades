//! **A button press coming back.**
//!
//! Telegram sends a `callback_query` rather than a message, so the inbox has
//! to read both. It looks like this:
//!
//! ```text
//!     label:took:41        ✅ took it,    on signal 41
//!     label:skipped:41     ❌ skipped it, on signal 41
//! ```
//!
//! **The id is in the button because nothing else identifies the card.** Two
//! setups on one pair within an hour would otherwise be indistinguishable, and
//! the label would land on whichever the code guessed.
//!
//! ## Answering is not optional
//!
//! Telegram spins the button until `answerCallbackQuery` comes back, and
//! **RESENDS the callback if it does not hear.** So a tap that is recorded but
//! not answered arrives again, and again. The write is an upsert for exactly
//! that reason, and this answers even when the write failed.

use anyhow::{Context, Result, anyhow};
use chrono::Utc;
use nsc_data::store::{self, Store, Verdict};
use serde_json::{Value, json};

/// Handles one tap. **Gives back what to flash on his screen.**
pub async fn pressed(
    client: &reqwest::Client,
    token: &str,
    query: &Value,
    record: Option<&Store>,
) -> Result<()> {
    let id = query["id"]
        .as_str()
        .ok_or_else(|| anyhow!("a callback with no id"))?;

    let data = query["data"].as_str().unwrap_or_default();

    // **Answered whatever happens.** An unanswered callback is resent, so a
    // failure here that skipped the answer would become a loop.
    let said = match record_it(data, record).await {
        Ok(said) => said,
        Err(trouble) => {
            answer(client, token, id, "Could not save that", false).await?;
            return Err(trouble);
        }
    };

    answer(client, token, id, &said.words, said.stop_him).await
}

/// What to put on his screen, and how hard to put it there.
struct Said {
    words: String,

    /// **A real dialogue he has to dismiss, not the little grey strip.**
    ///
    /// Kept for the one case that asks something of him: a skip, which wants a
    /// reason. His words, 3 September 2026 — *"it just shows okay, this has
    /// been skipped, but it does not show anything indicating that you need to
    /// say why."*
    ///
    /// **Only for that.** A popup after every tap would be a box to dismiss
    /// twenty times a week, and he would stop reading the one that mattered.
    stop_him: bool,
}

/// Reads the button and writes the verdict.
async fn record_it(data: &str, record: Option<&Store>) -> Result<Said> {
    let (verdict, signal_id) = read(data)?;

    let Some(record) = record else {
        return Err(anyhow!("no database, so nothing was saved"));
    };

    let changed = store::thought(record, signal_id, verdict.words(), Utc::now())
        .await
        .context("could not write the verdict")?;

    // **A skip stops him and asks for the reason.** That is the row worth
    // having, and the moment he has just tapped is the only moment he is
    // thinking about it — an hour later the reason is gone.
    //
    // Nothing asks after "took it": a reason is only for the ones he turned
    // down, because taking one means the rules were right.
    //
    // **Saying "already" rather than "saved" is the honest answer** to a
    // second tap, and it tells him the first one landed.
    Ok(match (changed, verdict) {
        (_, Verdict::Skipped) => Said {
            words: "Skipped.\n\nNow tell me why — send:\n\n/why it ran into news".into(),
            stop_him: true,
        },

        (true, _) => Said {
            words: format!("Saved — {}", verdict.words()),
            stop_him: false,
        },

        (false, _) => Said {
            words: format!("Already {}", verdict.words()),
            stop_him: false,
        },
    })
}

/// Reads `label:took:41` into a verdict and a signal id.
///
/// **A shape this code does not recognise is refused, never guessed.** A
/// callback from an older version of the buttons must not be recorded as a
/// verdict he did not give.
fn read(data: &str) -> Result<(Verdict, i64)> {
    let mut parts = data.split(':');

    if parts.next() != Some("label") {
        return Err(anyhow!("not a label button: {data}"));
    }

    let word = parts.next().unwrap_or_default();
    let verdict =
        Verdict::from_button(word).ok_or_else(|| anyhow!("unknown verdict button: {word}"))?;

    let signal_id: i64 = parts
        .next()
        .unwrap_or_default()
        .parse()
        .map_err(|_| anyhow!("a label button with no signal on it: {data}"))?;

    Ok((verdict, signal_id))
}

/// Stops the button spinning, with a line along the top of his screen.
async fn answer(
    client: &reqwest::Client,
    token: &str,
    id: &str,
    text: &str,
    alert: bool,
) -> Result<()> {
    let reply: Value = client
        .post(format!(
            "https://api.telegram.org/bot{token}/answerCallbackQuery"
        ))
        .json(&json!({
            "callback_query_id": id,
            "text": text,
            // `false` is the little grey strip along the top; `true` is a
            // dialogue he has to dismiss.
            "show_alert": alert,
        }))
        .send()
        .await
        // **The token is in the URL**, and reqwest puts the URL it was trying
        // into the message. Same rule the rest of this crate already follows.
        .map_err(|trouble| anyhow!("could not answer the button: {}", trouble.without_url()))?
        .json()
        .await
        .context("Telegram answered the button, but not with JSON")?;

    // **Refused is not answered.** Returning Ok here would leave the button
    // spinning on his phone while everything upstream believed it had stopped
    // — and Telegram RESENDS a callback it thinks went unanswered, so the tap
    // would arrive again and again.
    //
    // The commonest refusal is a query older than about fifteen minutes, which
    // nothing can be done about. It is still worth saying out loud rather than
    // swallowing, because a button that silently stops working looks exactly
    // like a button nobody pressed.
    if reply["ok"] != true {
        return Err(anyhow!(
            "Telegram would not take the answer: {}",
            reply["description"].as_str().unwrap_or("no reason given")
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::read;
    use nsc_data::store::Verdict;

    #[test]
    fn it_reads_both_buttons() {
        assert_eq!(read("label:took:41").expect("took"), (Verdict::Took, 41));
        assert_eq!(
            read("label:skipped:7").expect("skipped"),
            (Verdict::Skipped, 7)
        );
    }

    /// **Nothing it does not recognise becomes a verdict.** A callback from an
    /// older set of buttons, or from something else entirely, must not be
    /// recorded as a decision he never made.
    #[test]
    fn anything_else_is_refused_rather_than_guessed() {
        assert!(read("label:maybe:41").is_err(), "no such verdict");
        assert!(read("label:took:").is_err(), "no signal on it");
        assert!(read("label:took:abc").is_err(), "not a number");
        assert!(read("something:else:41").is_err(), "not a label at all");
        assert!(read("").is_err());
    }

    /// **`would have skipped` has no button**, and must not be reachable from
    /// one. It is what he says later in words, once the outcome came in.
    #[test]
    fn the_third_verdict_cannot_be_tapped() {
        assert!(read("label:would have skipped:41").is_err());
        assert!(Verdict::from_button("would have skipped").is_none());
    }
}
