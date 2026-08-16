//! Saying things to Telegram.

use anyhow::{Context, Result, anyhow};
use serde_json::{Value, json};

use super::OWNER;

/// Sends a message, with buttons or without.
///
/// `keyboard` is a list of rows, each row a list of button words. Passing
/// nothing takes the buttons away and gives him his own keyboard back.
///
/// **Every keyboard gets a Close row added.** Adding it at each call site
/// would mean one of them eventually not having it, and the one without it is
/// the flow he gets stuck in.
pub async fn say(
    client: &reqwest::Client,
    token: &str,
    text: &str,
    keyboard: Option<Value>,
) -> Result<()> {
    let markup = keys(keyboard);

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
        .map_err(|trouble| anyhow!("could not reach Telegram: {}", trouble.without_url()))?
        .json()
        .await
        .context("Telegram answered, but not with JSON")?;

    // **Refused is not sent.** This printed the refusal to a terminal he is
    // not watching and answered Ok, so everything upstream believed he had
    // been replied to. He would have seen nothing at all and had no way to
    // tell that from the bot being dead.
    if reply["ok"] != true {
        return Err(anyhow!(
            "Telegram refused: {}",
            reply["description"].as_str().unwrap_or("no reason given")
        ));
    }

    Ok(())
}

/// Turns a list of button rows into what Telegram wants, and **adds Close**.
///
/// Separate from the sending so it can be checked without a network. The thing
/// worth checking is that no keyboard can go out without a way off it.
fn keys(keyboard: Option<Value>) -> Value {
    let Some(rows) = keyboard else {
        // Takes the buttons away and gives him his own keyboard back.
        return json!({ "remove_keyboard": true });
    };

    let mut rows = match rows {
        Value::Array(rows) => rows,

        // Not a list of rows. Nothing builds one like this, but sending it
        // would put up a keyboard with no way off — so treat it as none.
        _ => return json!({ "remove_keyboard": true }),
    };

    rows.push(json!([super::CLOSE]));

    json!({
        "keyboard": rows,
        "resize_keyboard": true,
        "one_time_keyboard": true,
    })
}

/// Makes text safe to put in a message.
///
/// **Every message here is parsed as HTML**, so a stray `<` in an error is not
/// a stray `<` — it is an unclosed tag, and Telegram refuses the whole
/// message. The one place that carries text nobody wrote on purpose is the
/// reply that says what went wrong, which is exactly the message that must
/// arrive.
pub fn plainly(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The one that matters: **no keyboard goes out without a way off it.**
    #[test]
    fn every_keyboard_can_be_closed() {
        let put_up = keys(Some(json!([["Weekly", "Daily"], ["4-hour"]])));
        let rows = put_up["keyboard"].as_array().expect("rows");

        assert_eq!(rows.len(), 3, "the Close row should have been added");
        assert_eq!(rows[2], json!([super::super::CLOSE]));

        // On its own row, not tacked onto the end of a real one — a mis-tap
        // beside "Weekly" would otherwise back him out of what he was doing.
        assert_eq!(rows[2].as_array().expect("row").len(), 1);
    }

    #[test]
    fn nothing_means_take_the_buttons_away() {
        assert_eq!(keys(None), json!({ "remove_keyboard": true }));
    }

    /// Anything that is not a list of rows must not become a keyboard, or it
    /// goes up with nothing on it and nothing to press.
    #[test]
    fn a_shape_we_do_not_build_puts_up_nothing() {
        assert_eq!(
            keys(Some(json!("Weekly"))),
            json!({ "remove_keyboard": true })
        );
    }
}
