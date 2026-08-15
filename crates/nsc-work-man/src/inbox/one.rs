//! One pair, and what he can do to it.
//!
//! ```text
//!   /pairs        ->  every pair he has
//!   tap GBPUSD    ->  what it holds, and three things he can do
//!                     [+ Add levels] [− Take one off] [✗ Stop watching]
//! ```
//!
//! **This is where a level gets taken off.** Undo only ever reached what the
//! last message added — which covers a typo the moment it happens, and does
//! nothing at all for "that 1.15 from last week was wrong".

use std::path::Path;

use anyhow::Result;
use nsc_core::levels::{Pair, TIMEFRAMES_ORDER, known, load_pair, take_off, with_slash};
use rust_decimal::Decimal;
use serde_json::json;

use super::conversation::Adding;
use super::pairs;
use super::talking::say;
use super::{ADD, DROP, STOP, TIMEFRAMES};

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
            return Some(offer_to_take_one_off(client, token, folder, &name, adding).await);
        }

        if text == STOP {
            adding.removing = false;
            return Some(pairs::ask_first(client, token, folder, name, adding).await);
        }

        // A level he tapped off the list of levels.
        if adding.dropping
            && let Some(price) = price_on(text)
        {
            return Some(took_one_off(client, token, folder, &name, price, adding).await);
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

/// What one pair holds, and the three things he can do to it.
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
    let buttons = json!([[ADD, DROP], [STOP]]);

    say(client, token, &words, Some(buttons)).await
}

/// Its levels as buttons, one each, so he can take one off.
pub async fn offer_to_take_one_off(
    client: &reqwest::Client,
    token: &str,
    folder: &Path,
    name: &str,
    adding: &mut Adding,
) -> Result<()> {
    let Ok(pair) = load_pair(&folder.join(format!("{name}.toml"))) else {
        return say(client, token, "That pair's file will not read.", None).await;
    };

    if pair.levels.is_empty() {
        return say(client, token, "It has no levels on it.", None).await;
    }

    adding.dropping = true;

    // The price as it is written in the file, so tapping it hands back exactly
    // what is there and nothing has to be guessed at.
    let buttons: Vec<Vec<String>> = pair
        .levels
        .iter()
        .map(|line| vec![format!("{} {}", line.timeframe.name(), line.price)])
        .collect();

    say(
        client,
        token,
        "Which one should come off?",
        Some(json!(buttons)),
    )
    .await
}

/// Takes it off, and says what is left.
pub async fn took_one_off(
    client: &reqwest::Client,
    token: &str,
    folder: &Path,
    name: &str,
    price: Decimal,
    adding: &mut Adding,
) -> Result<()> {
    adding.dropping = false;

    let left = take_off(folder, name, price)?;
    let words = format!(
        "<b>{}</b> · {price} taken off\n\n{}",
        with_slash(name),
        listed(&left)
    );

    say(client, token, &words, Some(json!([[ADD, DROP], [STOP]]))).await
}

/// The price out of a button like `weekly 1.21279`.
pub fn price_on(button: &str) -> Option<Decimal> {
    button.rsplit_once(' ')?.1.parse().ok()
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
fn listed(pair: &Pair) -> String {
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
