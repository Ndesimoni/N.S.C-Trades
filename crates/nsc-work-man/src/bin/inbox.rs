//! Listen to what he sends the bot, and save it.
//!
//! The other side of Telegram. `telegram.rs` talks; this listens.
//!
//! Buttons are not set up anywhere — the bot sends them with a message, and
//! tapping one sends that word back as an ordinary message. A button is a
//! shortcut for typing, nothing more.
//!
//! ```text
//!   /level      ->  which pair?      [XAUUSD] [GBPUSD] [+ new pair]
//!   XAUUSD      ->  which timeframe? [Weekly] [Daily] [4-hour]
//!   Weekly      ->  send prices
//!   4520 4000   ->  saved, and it says back what the pair now holds
//! ```
//!
//! **The buttons are the files in `config/pairs/`.** Not a list in this file —
//! that was the mistake `settings.rs` made, and two lists always disagree in
//! the end.

use std::path::Path;

use anyhow::{Context, Result};
use nsc_work_man::levels::{Timeframe, digits_for, known, save, with_slash};
use rust_decimal::Decimal;
use serde_json::{Value, json};

/// Only he may write levels.
///
/// **Channel posts carry no sender at all** — Telegram strips it, because a
/// post is from the channel rather than from a person. So the private chat is
/// the only place the bot can tell who is talking.
const OWNER: i64 = 6089491075;

const PAIRS: &str = "config/pairs";
const TIMEFRAMES: [(&str, Timeframe); 3] = [
    ("Weekly", Timeframe::Weekly),
    ("Daily", Timeframe::Daily),
    ("4-hour", Timeframe::H4),
];

const NEW_PAIR: &str = "+ new pair";

/// Where he is in the flow.
///
/// It stays put once set, so a run of six weekly levels is two taps and six
/// numbers — the pair and the timeframe are never typed twice.
#[derive(Default)]
struct Adding {
    pair: Option<String>,
    timeframe: Option<Timeframe>,
    naming: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let token = std::env::var("TELEGRAM_BOT_TOKEN").context("TELEGRAM_BOT_TOKEN is not set")?;
    let client = reqwest::Client::new();
    let mut adding = Adding::default();
    let mut seen_up_to: i64 = 0;

    println!("listening. Send your bot /level\n");

    loop {
        // `timeout=30` makes Telegram hold the line open rather than answering
        // "nothing" instantly. One request every thirty seconds, not hundreds.
        let url = format!(
            "https://api.telegram.org/bot{token}/getUpdates?offset={}&timeout=30",
            seen_up_to + 1
        );

        let reply: Value = client
            .get(&url)
            .send()
            .await
            .context("could not reach Telegram")?
            .json()
            .await
            .context("Telegram answered, but not with JSON")?;

        for update in reply["result"].as_array().into_iter().flatten() {
            seen_up_to = update["update_id"].as_i64().unwrap_or(seen_up_to);
            let message = &update["message"];

            // Anything not from him is ignored without a word. A bot that
            // argues with strangers is a bot that tells them it exists.
            if message["from"]["id"].as_i64() != Some(OWNER) {
                continue;
            }

            let Some(text) = message["text"].as_str() else {
                continue;
            };

            println!("he said: {text}");

            if let Err(trouble) = handle(&client, &token, text.trim(), &mut adding).await {
                println!("  -> {trouble:#}");
                say(
                    &client,
                    &token,
                    &format!("Could not do that:\n{trouble}"),
                    None,
                )
                .await?;
            }
        }
    }
}

/// Works out what he meant and answers.
async fn handle(
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
        return say(client, token, &lines.join("\n"), Some(json!([names]))).await;
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
fn prices_in(text: &str) -> Vec<Decimal> {
    text.split_whitespace()
        .filter_map(|word| word.replace(',', "").parse::<Decimal>().ok())
        .collect()
}

/// Sends a message, with buttons or without.
///
/// `keyboard` is a list of rows, each row a list of button words. Passing
/// nothing takes the buttons away and gives him his own keyboard back.
async fn say(
    client: &reqwest::Client,
    token: &str,
    text: &str,
    keyboard: Option<Value>,
) -> Result<()> {
    let markup = match keyboard {
        Some(rows) => json!({
            "keyboard": rows,
            "resize_keyboard": true,
            "one_time_keyboard": true,
        }),
        None => json!({ "remove_keyboard": true }),
    };

    let reply: Value = client
        .post(format!("https://api.telegram.org/bot{token}/sendMessage"))
        .json(&json!({
            "chat_id": OWNER,
            "text": text,
            "parse_mode": "HTML",
            "reply_markup": markup,
        }))
        .send()
        .await
        .context("could not reach Telegram")?
        .json()
        .await
        .context("Telegram answered, but not with JSON")?;

    if reply["ok"] != true {
        println!(
            "Telegram refused: {}",
            reply["description"].as_str().unwrap_or("no reason given")
        );
    }

    Ok(())
}
