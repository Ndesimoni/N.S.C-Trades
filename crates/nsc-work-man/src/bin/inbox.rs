//! Listen to what he sends the bot, and answer.
//!
//! The other side of Telegram. `telegram.rs` talks; this listens.
//!
//! Buttons are not set up anywhere — the bot sends them with a message, and
//! tapping one sends that word back as an ordinary message. A button is a
//! shortcut for typing, nothing more.
//!
//! ```text
//!   /level      ->  which pair?      [XAUUSD] [EURUSD] ...
//!   XAUUSD      ->  which timeframe? [Weekly] [Daily] [4-hour]
//!   Weekly      ->  send prices
//!   4520        ->  saved
//!   4000        ->  saved            (still XAUUSD weekly)
//! ```

use anyhow::{Context, Result};
use serde_json::{Value, json};

/// Only he may write levels.
///
/// **Channel posts carry no sender at all** — Telegram strips it, because a
/// post is from the channel rather than from a person. So the private chat is
/// the only place the bot can tell who is talking.
const OWNER: i64 = 6089491075;

const PAIRS: [&str; 4] = ["XAUUSD", "EURUSD", "GBPUSD", "USDJPY"];
const TIMEFRAMES: [&str; 3] = ["Weekly", "Daily", "4-hour"];

/// Where he is in the flow.
///
/// It stays put once set, so a run of six weekly levels is two taps and six
/// numbers — the pair and the timeframe are never typed twice.
#[derive(Default)]
struct Adding {
    pair: Option<String>,
    timeframe: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let token = std::env::var("TELEGRAM_BOT_TOKEN").context("TELEGRAM_BOT_TOKEN is not set")?;
    let client = reqwest::Client::new();
    let mut adding = Adding::default();

    println!("listening. Send your bot /level\n");

    // Telegram hands you the same messages over and over until you say how far
    // you have read. This is that marker.
    let mut seen_up_to: i64 = 0;

    loop {
        // `timeout=30` makes Telegram hold the line open for up to thirty
        // seconds rather than answering "nothing" instantly.
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
            handle(&client, &token, text.trim(), &mut adding).await?;
        }
    }
}

/// Works out what he meant and answers.
async fn handle(client: &reqwest::Client, token: &str, text: &str, adding: &mut Adding) -> Result<()> {
    // Start again.
    if text == "/level" {
        *adding = Adding::default();
        let rows: Vec<Vec<&str>> = PAIRS.chunks(2).map(|two| two.to_vec()).collect();

        return say(client, token, "Which pair?", Some(json!(rows))).await;
    }

    // A pair.
    if let Some(pair) = PAIRS.iter().find(|known| known.eq_ignore_ascii_case(text)) {
        adding.pair = Some((*pair).to_string());
        adding.timeframe = None;

        return say(client, token, "Which timeframe?", Some(json!([TIMEFRAMES]))).await;
    }

    // A timeframe — but only once a pair is chosen.
    if let Some(found) = TIMEFRAMES.iter().find(|known| known.eq_ignore_ascii_case(text)) {
        let Some(pair) = adding.pair.clone() else {
            return say(client, token, "Pick a pair first — send /level", None).await;
        };

        adding.timeframe = Some((*found).to_string());

        let words = format!("{pair} · {found}\n\nSend prices, one per message.");
        return say(client, token, &words, None).await;
    }

       // Prices. One, or a whole set on separate lines.
    let prices = prices_in(text);

    if !prices.is_empty() {
        let (Some(pair), Some(timeframe)) = (&adding.pair, &adding.timeframe) else {
            return say(client, token, "Send /level first, so I know what those are", None).await;
        };

        let listed: Vec<String> = prices.iter().map(|price| price.to_string()).collect();

        // Step 4 writes these to a file. For now it says them back, so a typed
        // 45200 is caught by his eyes before it becomes a level.
        let words = format!(
            "<b>{pair} · {timeframe}</b>\n{} level{} — {}\n\nAnother timeframe?",
            prices.len(),
            if prices.len() == 1 { "" } else { "s" },
            listed.join(" · ")
        );

        return say(client, token, &words, Some(json!([TIMEFRAMES]))).await;
    }


    say(client, token, "Send /level to add a level", None).await
}

/// Sends a message, with buttons or without.
///
/// `keyboard` is a list of rows, each row a list of button words. Passing
/// nothing takes the buttons away and gives him his own keyboard back.
async fn say(client: &reqwest::Client, token: &str, text: &str, keyboard: Option<Value>) -> Result<()> {
    let markup = match keyboard {
        Some(rows) => json!({
            "keyboard": rows,
            "resize_keyboard": true,     // fit the buttons, do not fill the screen
            "one_time_keyboard": true,   // hide them once he taps
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


/// Every number in the message.
///
/// One per line, several on a line, or one on its own — it takes whatever is
/// there.
///
/// **Nothing asks how many.** A count is one more thing to get wrong: say four
/// and send three and the bot waits forever; say four and send five and one
/// gets dropped. Reading whatever arrives has no count to be wrong about.
fn prices_in(text: &str) -> Vec<f64> {
    text.split_whitespace()
        .filter_map(|word| word.replace(',', "").parse::<f64>().ok())
        .collect()
}
