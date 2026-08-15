//! Working out what he meant, and what to say back.

use std::path::Path;

use anyhow::Result;
use nsc_core::levels::{Timeframe, digits_for, known, save, undo, with_slash};
use rust_decimal::Decimal;
use serde_json::json;

use super::picture::show;
use super::talking::say;
use super::{NEW_PAIR, PAIRS, TIMEFRAMES, UNDO};

/// Where he is in the flow.
///
/// It stays put once set, so a run of six weekly levels is two taps and six
/// numbers — the pair and the timeframe are never typed twice.
#[derive(Default)]
pub struct Adding {
    pair: Option<String>,
    timeframe: Option<Timeframe>,
    naming: bool,
    /// What the last message added, so Undo knows how much to take back off.
    just_added: Option<(String, usize)>,
}

/// Works out what he meant and answers.
pub async fn handle(
    client: &reqwest::Client,
    token: &str,
    text: &str,
    adding: &mut Adding,
) -> Result<()> {
    let folder = Path::new(PAIRS);

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
        adding.just_added = Some((pair.clone(), prices.len()));

        // Say back what the pair NOW HOLDS, not only what just arrived. A
        // mistyped 1.4000 is then caught by his eye in the reply rather than
        // three weeks later when a signal fires in the wrong place.
        let mut lines = vec![format!("<b>{} · saved</b>", with_slash(&pair))];

        for (word, kind) in TIMEFRAMES {
            let held: Vec<String> = saved
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
        return show(client, token, &saved).await;
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
