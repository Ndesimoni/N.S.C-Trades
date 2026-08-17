//! One pair, and what he can do to it.
//!
//! ```text
//!   /pairs        ->  every pair he has
//!   tap GBPUSD    ->  what it holds, and four things he can do
//!                     [+ Add levels] [− Take one off]
//!                     [📈 Chart]
//!                     [✗ Stop watching]
//! ```
//!
//! **This is where a level gets taken off.** Undo only ever reached what the
//! last message added — which covers a typo the moment it happens, and does
//! nothing at all for "that 1.15 from last week was wrong".

use std::path::Path;

use anyhow::Result;
use nsc_core::levels::{Pair, TIMEFRAMES_ORDER, known, load_pair};
use serde_json::json;

use super::conversation::Adding;
use super::talking::say;
use super::{ADD, CHART, DROP, STOP, TIMEFRAMES};
use super::{dropping, pairs};

/// Anything to do with one pair's page.
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
    if text == "/pairs" {
        *adding = Adding::default();
        adding.browsing = true;

        return Some(list(client, token, folder).await);
    }

    if let Some(name) = adding.chosen.clone() {
        if text == ADD {
            adding.pair = Some(name.clone());
            adding.timeframe = None;

            let names: Vec<&str> = TIMEFRAMES.iter().map(|(word, _)| *word).collect();
            let words = format!("{name} — which timeframe?");
            return Some(say(client, token, &words, Some(json!([names]))).await);
        }

        if text == DROP {
            return Some(dropping::offer(client, token, folder, &name, adding).await);
        }

        // **Only remembers which pair, and asks.** Drawing takes Chrome the
        // best part of ten seconds, so the chart itself is left to `route`
        // once he has said which one he wants.
        if text == CHART {
            adding.dropping = false;
            adding.chart_of = Some(name.clone());

            let names: Vec<&str> = TIMEFRAMES.iter().map(|(word, _)| *word).collect();
            let words = format!("{name} — which chart?");
            return Some(say(client, token, &words, Some(json!([names]))).await);
        }

        if text == STOP {
            adding.removing = false;
            return Some(pairs::ask_first(client, token, folder, name, adding).await);
        }

        // A level he tapped off the list of levels.
        if adding.dropping
            && let Some(price) = dropping::price_on(text)
        {
            return Some(dropping::took_one_off(client, token, folder, &name, price, adding).await);
        }
    }

    None
}

/// Every pair, as buttons.
pub async fn list(client: &reqwest::Client, token: &str, folder: &Path) -> Result<()> {
    let pairs = known(folder);

    if pairs.is_empty() {
        return say(client, token, "You have no pairs yet. Send /level.", None).await;
    }

    let buttons: Vec<Vec<String>> = pairs.chunks(2).map(<[String]>::to_vec).collect();

    say(client, token, "Your pairs", Some(json!(buttons))).await
}

/// What one pair holds, and the four things he can do to it.
pub async fn show(
    client: &reqwest::Client,
    token: &str,
    folder: &Path,
    name: &str,
    adding: &mut Adding,
) -> Result<()> {
    adding.chosen = Some(name.to_string());

    let Ok(pair) = load_pair(&folder.join(format!("{name}.toml"))) else {
        return say(client, token, "That pair's file will not read.", None).await;
    };

    let words = format!("{}\n\n{}", heading(&pair), listed(&pair));
    let buttons = json!([[ADD, DROP], [CHART], [STOP]]);

    say(client, token, &words, Some(buttons)).await
}

fn heading(pair: &Pair) -> String {
    format!(
        "<b>{}</b> — {} level{}",
        pair.symbol,
        pair.levels.len(),
        if pair.levels.len() == 1 { "" } else { "s" }
    )
}

/// What it holds, grouped by chart, in his own order.
pub(super) fn listed(pair: &Pair) -> String {
    let mut lines = Vec::new();

    for (word, kind) in TIMEFRAMES_ORDER {
        let held: Vec<String> = pair
            .levels
            .iter()
            .filter(|line| line.timeframe == kind)
            .map(|line| line.price.to_string())
            .collect();

        if !held.is_empty() {
            lines.push(format!("<b>{word}</b> — {}", held.join(" · ")));
        }
    }

    if lines.is_empty() {
        return "Nothing on it yet.".into();
    }

    lines.join("\n")
}
