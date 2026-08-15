//! Stopping a pair — the whole two-tap conversation.
//!
//! **Two taps, not one.** It throws away every level he has drawn for that
//! pair — months of chart work — and the first tap is made on a phone while he
//! is doing something else. The second one tells him how many levels are on it
//! before he confirms.
//!
//! And it is MOVED, not deleted. `retire` puts the file in
//! `config/pairs/removed/`, and moving it back starts the pair again.

use std::path::Path;

use anyhow::Result;
use nsc_core::levels::{known, load_pair, retire, with_slash};
use serde_json::json;

use super::conversation::Adding;
use super::talking::say;
use super::{NO, YES};

/// Handles anything to do with stopping a pair.
///
/// Gives back `None` when he was talking about something else, so the rest of
/// the conversation carries on unbothered.
pub async fn heard(
    client: &reqwest::Client,
    token: &str,
    folder: &Path,
    text: &str,
    adding: &mut Adding,
) -> Option<Result<()>> {
    if text == "/remove" {
        *adding = Adding::default();
        adding.removing = true;

        let pairs = known(folder);
        if pairs.is_empty() {
            return Some(say(client, token, "There are no pairs to remove", None).await);
        }

        let buttons: Vec<Vec<String>> = pairs.chunks(2).map(<[String]>::to_vec).collect();

        return Some(
            say(
                client,
                token,
                "Which pair should I stop watching?",
                Some(json!(buttons)),
            )
            .await,
        );
    }

    // **The second tap.** Stopping a pair throws away every level he drew for
    // it, and this is a button on a phone.
    if text == YES {
        let Some(name) = adding.stopping.take() else {
            return Some(say(client, token, "Nothing waiting to be removed", None).await);
        };

        let landed = match retire(folder, &name) {
            Ok(landed) => landed,
            Err(trouble) => return Some(Err(trouble.into())),
        };

        let words = format!(
            "<b>{}</b> · stopped\n\n\
             Nothing will be watched on it. Its levels are not gone — they are in\n\
             <code>{}</code>\n\n\
             Move that file back and it starts again.",
            with_slash(&name),
            landed.display(),
        );

        return Some(say(client, token, &words, None).await);
    }

    if text == NO {
        let Some(name) = adding.stopping.take() else {
            return Some(say(client, token, "Nothing waiting to be removed", None).await);
        };

        let words = format!("<b>{}</b> · kept", with_slash(&name));
        return Some(say(client, token, &words, None).await);
    }
    None
}

/// Shows what stopping a pair would throw away, and asks again.
///
/// **Two taps, not one.** It is months of chart work, and the first tap is
/// made on a phone while doing something else.
pub async fn ask_first(
    client: &reqwest::Client,
    token: &str,
    folder: &Path,
    name: String,
    adding: &mut Adding,
) -> Result<()> {
    // **How many it holds is the whole point of asking twice**, so a file that
    // cannot be read says so rather than saying nought. "It has 0 levels on
    // it" is the one sentence that would make him tap yes without thinking.
    let words = match load_pair(&folder.join(format!("{name}.toml"))) {
        Ok(pair) => format!(
            "<b>{}</b> — stop watching it?\n\n\
             It has <b>{}</b> level{} on it. Nothing will be watched on this pair \
             until you send it again.",
            with_slash(&name),
            pair.levels.len(),
            if pair.levels.len() == 1 { "" } else { "s" },
        ),

        Err(_) => format!(
            "<b>{}</b> — stop watching it?\n\n\
             <b>Its file will not read</b>, so I cannot tell you how many levels \
             are on it. Stopping moves the file rather than deleting it, so \
             nothing is lost either way.",
            with_slash(&name),
        ),
    };

    adding.stopping = Some(name);

    say(client, token, &words, Some(json!([[YES, NO]]))).await
}
