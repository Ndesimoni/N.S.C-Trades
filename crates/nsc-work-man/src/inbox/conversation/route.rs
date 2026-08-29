//! What a message means, given where he is.

use std::path::Path;

use anyhow::Result;
use nsc_core::levels::{known, undo, with_slash};
use serde_json::json;

use super::super::talking::say;
use super::super::words::{CLOSE, NEW_PAIR, TIMEFRAMES, TODAY, UNDO, WEEK};
use super::super::{asked, coming, one, pairs, picture};
use super::adding::Adding;
use super::naming;
use super::saving;
use crate::places::PAIRS;

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

    // **Before the pair flows.** These are two fixed words that belong to
    // nothing else, and answering them early means a half-finished /level
    // cannot swallow one.
    if text == "/news" {
        *adding = Adding::default();
        return coming::ask(client, token).await;
    }

    if text == TODAY {
        *adding = Adding::default();
        return coming::show(client, token, nsc_core::news::Span::Today).await;
    }

    if text == WEEK {
        *adding = Adding::default();
        return coming::show(client, token, nsc_core::news::Span::Week).await;
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

    // **The same page /pairs gives, one tap shorter.** Both land in the same
    // place; this is for when he already knows which pair he wants to look at.
    if text == "/chart" {
        *adding = Adding::default();
        adding.charting = true;

        let pairs = known(folder);
        if pairs.is_empty() {
            return say(client, token, "You have no pairs yet. Send /level.", None).await;
        }

        let buttons: Vec<Vec<String>> = pairs.chunks(2).map(<[String]>::to_vec).collect();
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

    // Tapped off /chart, so he wants to look at it rather than change it.
    if adding.charting
        && let Some(name) = tapped.cloned()
    {
        adding.charting = false;
        adding.chart_of = Some(name.clone());

        let names: Vec<&str> = TIMEFRAMES.iter().map(|(word, _)| *word).collect();
        let words = format!("{name} — which chart?");
        return say(client, token, &words, Some(json!([names]))).await;
    }

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
        // **A name he typed is checked with IBKR before ANYTHING is written.**
        //
        // A pair already on disk is not re-checked — startup sweeps those, and
        // asking again would cost a connection every time he taps one.
        let name = match tapped.cloned() {
            Some(known) => known,
            None => match naming::checked(client, token, text, adding).await? {
                Some(name) => name,
                None => return Ok(()),
            },
        };

        adding.naming = false;
        adding.pair = Some(name.clone());
        adding.timeframe = None;

        // **He has moved to a different pair, so forget the last one's page.**
        // These used to survive, and a level button from an older message then
        // took its price off whichever pair he was last LOOKING at rather than
        // the one he is now adding to. Telegram keeps old keyboards tappable
        // forever, so that button is one thumb away for as long as the chat
        // exists.
        adding.chosen = None;
        adding.dropping = false;

        // Same reason: a chart question left hanging on the old pair would
        // turn his next "Weekly" into a picture of a pair he has moved off.
        adding.chart_of = None;

        let words = if existing.contains(&name) {
            format!("{name} — which timeframe?")
        } else {
            format!("{name} is new. Which timeframe?")
        };

        let names: Vec<&str> = TIMEFRAMES.iter().map(|(word, _)| *word).collect();
        return say(client, token, &words, Some(json!([names]))).await;
    }

    // **A chart he asked for — checked before the adding flow's timeframe.**
    //
    // Both flows put up the same three buttons, and a button only sends its
    // own word back, so "Weekly" arrives identical either way. `chart_of` is
    // the only thing that says which question he is answering, and it is
    // cleared here so the next "Weekly" is a level again.
    if let Some(name) = adding.chart_of.clone()
        && let Some((_, chart)) = TIMEFRAMES
            .iter()
            .find(|(word, _)| word.eq_ignore_ascii_case(text))
    {
        adding.chart_of = None;
        return picture::of_pair(client, token, folder, &name, *chart).await;
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
