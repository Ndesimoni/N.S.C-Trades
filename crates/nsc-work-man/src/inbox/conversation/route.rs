//! What a message means, given where he is.

use std::path::Path;

use anyhow::Result;
use nsc_core::levels::{known, undo, with_slash};
use serde_json::json;

use super::super::talking::say;
use super::super::{CLOSE, NEW_PAIR, PAIRS, TIMEFRAMES, UNDO};
use super::super::{asked, one, pairs};
use super::adding::Adding;
use super::saving;

/// Works out what he meant and answers.
pub async fn handle(
    client: &reqwest::Client,
    token: &str,
    text: &str,
    adding: &mut Adding,
    standing: &tokio::sync::watch::Receiver<crate::watch::Snapshot>,
) -> Result<()> {
    let folder = Path::new(PAIRS);

    // **Backing out.** Forgets where he was and takes the buttons away, so his
    // own keyboard comes back. Nothing is undone — he has not asked for that,
    // he has asked to be left alone.
    if text == CLOSE {
        *adding = Adding::default();
        return say(client, token, "Closed.", None).await;
    }

    if text == "/help" || text == "/start" {
        return asked::help(client, token).await;
    }

    if text == "/status" {
        return asked::status(client, token, standing).await;
    }

    if let Some(answer) = pairs::heard(client, token, folder, text, adding).await {
        return answer;
    }

    if let Some(answer) = one::heard(client, token, folder, text, adding).await {
        return answer;
    }

    if text == "/level" {
        *adding = Adding::default();

        let mut buttons: Vec<Vec<String>> =
            known(folder).chunks(2).map(<[String]>::to_vec).collect();
        buttons.push(vec![NEW_PAIR.to_string()]);

        return say(client, token, "Which pair?", Some(json!(buttons))).await;
    }

    if text == UNDO {
        let Some((pair, count)) = adding.just_added.take() else {
            return say(client, token, "Nothing to undo", None).await;
        };

        let left = undo(folder, &pair, count)?;

        let words = format!(
            "<b>{}</b> · took {} back off\n{} level{} left",
            with_slash(&pair),
            count,
            left.levels.len(),
            if left.levels.len() == 1 { "" } else { "s" }
        );

        return say(client, token, &words, None).await;
    }

    if text == NEW_PAIR {
        adding.naming = true;
        return say(client, token, "Type it — like EURUSD", None).await;
    }

    // A pair: either one he tapped, or one he has just typed the name of.
    let existing = known(folder);
    let tapped = existing.iter().find(|name| name.eq_ignore_ascii_case(text));

    // Tapped off /pairs, so he wants to see it rather than add to it.
    if adding.browsing
        && let Some(name) = tapped.cloned()
    {
        adding.browsing = false;
        return one::show(client, token, folder, &name, adding).await;
    }

    // He is part-way through removing one, and has just named it.
    if adding.removing
        && let Some(name) = tapped.cloned()
    {
        adding.removing = false;
        return pairs::ask_first(client, token, folder, name, adding).await;
    }

    if tapped.is_some() || adding.naming {
        let name = tapped.cloned().unwrap_or_else(|| text.to_uppercase());
        adding.naming = false;
        adding.pair = Some(name.clone());
        adding.timeframe = None;

        let words = if existing.contains(&name) {
            format!("{name} — which timeframe?")
        } else {
            format!("{name} is new. Which timeframe?")
        };

        let names: Vec<&str> = TIMEFRAMES.iter().map(|(word, _)| *word).collect();
        return say(client, token, &words, Some(json!([names]))).await;
    }

    // A timeframe — but only once a pair is chosen.
    if let Some((word, timeframe)) = TIMEFRAMES
        .iter()
        .find(|(word, _)| word.eq_ignore_ascii_case(text))
    {
        let Some(pair) = adding.pair.clone() else {
            return say(client, token, "Pick a pair first — send /level", None).await;
        };

        adding.timeframe = Some(*timeframe);

        let words =
            format!("<b>{pair} · {word}</b>\n\nSend prices — one per line, or all at once.");
        return say(client, token, &words, None).await;
    }

    saving::save_them(client, token, folder, text, adding).await
}
