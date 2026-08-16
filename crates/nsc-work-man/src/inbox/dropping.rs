//! Taking one level off a pair.
//!
//! **This is the one Undo could never reach.** Undo cuts what the last message
//! added, which covers a typo the moment it happens and does nothing at all
//! for "that 1.15 from last week was wrong".
//!
//! Each level goes up as its own button reading `weekly 1.21279`, so tapping
//! one hands back exactly what is in the file and nothing has to be guessed.

use std::path::Path;

use anyhow::Result;
use nsc_core::levels::{TIMEFRAMES_ORDER, load_pair, take_off, with_slash};
use rust_decimal::Decimal;
use serde_json::json;

use super::conversation::Adding;
use super::one::listed;
use super::talking::say;
use super::{ADD, DROP, STOP};

/// Its levels as buttons, one each, so he can take one off.
pub async fn offer(
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

    let took = take_off(folder, name, price)?;

    // **Says which it was.** Nothing coming off is not an error — he may have
    // tapped a button from an older message, and Telegram keeps those tappable
    // forever. But it used to say "taken off" either way, so a tap that did
    // nothing looked exactly like one that worked.
    let words = if took.was_there {
        format!(
            "<b>{}</b> · {price} taken off\n\n{}",
            with_slash(name),
            listed(&took.pair)
        )
    } else {
        format!(
            "<b>{}</b> · nothing taken off\n\n\
             {price} is not on it — that button is from an older message.\n\n{}",
            with_slash(name),
            listed(&took.pair)
        )
    };

    say(client, token, &words, Some(json!([[ADD, DROP], [STOP]]))).await
}

/// The price out of a button like `weekly 1.21279`.
///
/// **The chart name has to be there.** Reading the last number off any message
/// meant that while the take-one-off list was up, sending "1.28 1.31" — which
/// is how he is told to add two levels — was read as "take 1.31 off", and off
/// whichever pair's page he was last on rather than the one he was adding to.
pub fn price_on(button: &str) -> Option<Decimal> {
    let (chart, price) = button.rsplit_once(' ')?;

    if !TIMEFRAMES_ORDER
        .iter()
        .any(|(_, kind)| kind.name() == chart)
    {
        return None;
    }

    price.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::price_on;
    use rust_decimal::Decimal;

    #[test]
    fn reads_the_price_off_a_level_button() {
        assert_eq!(
            price_on("weekly 1.21279"),
            "1.21279".parse::<Decimal>().ok()
        );
        assert_eq!(price_on("4-hour 4120.5"), "4120.5".parse::<Decimal>().ok());
    }

    /// **Two prices on one line is how he is told to add them.** Read as a
    /// button it becomes "take 1.31 off", and it came off the pair whose page
    /// he was last on rather than the one he was adding to.
    #[test]
    fn a_line_of_prices_is_not_a_button() {
        assert_eq!(price_on("1.28 1.31"), None);
    }

    #[test]
    fn nor_is_anything_else_ending_in_a_number() {
        assert_eq!(price_on("EURUSD 1.09"), None);
        assert_eq!(price_on("some note 5"), None);
    }
}
