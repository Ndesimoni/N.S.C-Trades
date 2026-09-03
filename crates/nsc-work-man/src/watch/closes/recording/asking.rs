//! **The two buttons under a setup** — took it, skipped it.
//!
//! ## Why the card is sent here and not with the charts
//!
//! **Telegram does not allow buttons on a group of photos.** So the two charts
//! go as a group, and the setup card comes through here as a photo of its own
//! with the tick and the cross on it.
//!
//! They used to arrive as a separate text message underneath, which is what he
//! saw and did not want: *"they are in a different card — feed them in the
//! same card."*
//!
//! The card also cannot go earlier than this: the buttons carry the signal's
//! ROW ID, and the row does not exist until the signal has been written.
//!
//! ## Why the id travels in the button
//!
//! Telegram hands back whatever the button was created with. The signal's row
//! id is the only thing that identifies which card he pressed; a symbol and a
//! time would collide the moment two shapes print on one pair.

use std::path::Path;

use serde_json::json;

use crate::places::OWNER;
use crate::telegram::{self, SendError};

/// Sends the line with the two buttons, straight after the pictures.
///
/// **Nothing here can end the run.** A button that will not send costs the
/// label, not the signal — he still has the setup on his phone.
pub async fn ask(
    client: &reqwest::Client,
    signal_id: i64,
    sentence: &str,
    card: Option<&Path>,
) -> Result<(), SendError> {
    let keyboard = json!({
        "inline_keyboard": [[
            { "text": "✅ took it",    "callback_data": format!("label:took:{signal_id}") },
            { "text": "❌ skipped it", "callback_data": format!("label:skipped:{signal_id}") },
        ]]
    });

    let owner = OWNER.to_string();

    // **Named either way**, so a tap can never land on the wrong card. Two
    // setups on one pair in an hour would otherwise be two identical
    // questions.
    let words = format!("<b>{sentence}</b>");

    match card {
        Some(card) => telegram::send_with_buttons(client, &owner, card, &words, keyboard).await,

        // **No card means the pictures never went.** The buttons still can:
        // he has nothing to look at, but the setup is in the record and a
        // verdict on it is still worth having.
        None => {
            let asking = format!("{words}\n\nDid you take it?");
            telegram::ask_words(client, &owner, &asking, keyboard).await
        }
    }
}
