//! Working out what he meant, and what to say back.

use std::path::Path;

use anyhow::Result;
use nsc_core::levels::{Timeframe, digits_for, known, save, undo, with_slash};
use rust_decimal::Decimal;
use serde_json::json;

use super::picture::show;
use super::talking::say;
use super::{CLOSE, NEW_PAIR, PAIRS, TIMEFRAMES, UNDO};
use super::{asked, one, pairs};

/// Where he is in the flow.
///
/// It stays put once set, so a run of six weekly levels is two taps and six
/// numbers — the pair and the timeframe are never typed twice.
#[derive(Default)]
pub struct Adding {
    /// Reachable from `one`, which hands him into this flow when he taps
    /// "add levels" on a pair's page.
    pub(super) pair: Option<String>,
    pub(super) timeframe: Option<Timeframe>,
    naming: bool,
    /// What the last message added, so Undo knows how much to take back off.
    just_added: Option<(String, usize)>,

    /// He has sent /remove and is picking which pair.
    ///
    /// Reachable from `stopping`, which owns that whole conversation.
    pub(super) removing: bool,

    /// The pair he has asked to stop watching, waiting on a second tap.
    pub(super) stopping: Option<String>,

    /// He has sent /restore and is picking which set to bring back.
    pub(super) restoring: bool,

    /// He has sent /pairs and is picking which one to look at.
    pub(super) browsing: bool,

    /// The pair whose page he is on, from /pairs.
    pub(super) chosen: Option<String>,

    /// He is picking which level to take off that pair.
    pub(super) dropping: bool,
}

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

    // Prices.
    let prices = prices_in(text);

    if !prices.is_empty() {
        let (Some(pair), Some(timeframe)) = (adding.pair.clone(), adding.timeframe) else {
            return say(
                client,
                token,
                "Send /level first, so I know what those are",
                None,
            )
            .await;
        };

        let saved = save(folder, &pair, timeframe, &prices, digits_for(&pair))?;

        // **Only what was actually added.** Undo cuts the last N levels off the
        // file, so telling it a number that includes ones already there would
        // have it cut levels he sent weeks ago.
        adding.just_added = Some((pair.clone(), saved.added));

        // Say back what the pair NOW HOLDS, not only what just arrived. A
        // mistyped 1.4000 is then caught by his eye in the reply rather than
        // three weeks later when a signal fires in the wrong place.
        let mut lines = vec![match (saved.added, saved.already.len()) {
            (_, 0) => format!("<b>{} · saved</b>", with_slash(&pair)),
            (0, _) => format!("<b>{} · nothing new</b>", with_slash(&pair)),
            (new, old) => format!(
                "<b>{} · {new} saved</b>, {old} you already had",
                with_slash(&pair)
            ),
        }];

        // **Name the ones he already had, and the chart they are on.** He may
        // have just re-sent a weekly line off his daily chart expecting it to
        // move; saying nothing would leave him thinking it had.
        for (price, timeframe) in &saved.already {
            lines.push(format!(
                "· {price} is already a <b>{}</b> level",
                timeframe.name()
            ));
        }

        for (word, kind) in TIMEFRAMES {
            let held: Vec<String> = saved
                .pair
                .levels
                .iter()
                .filter(|line| line.timeframe == kind)
                .map(|line| line.price.to_string())
                .collect();

            if !held.is_empty() {
                lines.push(format!(
                    "\n<b>{word}</b> — {}\n{}",
                    held.len(),
                    held.join(" · ")
                ));
            }
        }

        let names: Vec<&str> = TIMEFRAMES.iter().map(|(word, _)| *word).collect();
        let buttons = json!([names, [UNDO]]);

        say(client, token, &lines.join("\n"), Some(buttons)).await?;

        // Then show him where they landed.
        //
        // Reading the price back only proves he can read his own typing. The
        // picture shows the PLACE, which is the thing that actually goes wrong
        // — and it is how he reads a chart anyway.
        return show(client, token, &saved.pair).await;
    }

    say(client, token, "Send /level to add a level", None).await
}

/// Every number in the message.
///
/// One per line, several on a line, or one on its own — whatever is there.
///
/// **Nothing asks how many.** A count is one more thing to get wrong: say four
/// and send three and the bot waits forever; say four and send five and one
/// gets dropped.
pub fn prices_in(text: &str) -> Vec<Decimal> {
    text.split_whitespace()
        .filter_map(|word| word.replace(',', "").parse::<Decimal>().ok())
        .collect()
}
