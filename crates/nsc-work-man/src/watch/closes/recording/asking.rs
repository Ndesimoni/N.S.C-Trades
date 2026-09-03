//! **The two buttons under a setup** — took it, skipped it.
//!
//! ## Why they are their own message
//!
//! **Telegram does not allow buttons on a group of photos.** The three cards
//! go out as one container — his choice, 4 September 2026, made with the trade
//! in front of him — so the tick and the cross cannot ride on them.
//!
//! They sit in a slim message directly beneath instead.
//!
//! ## And why they cannot go any earlier
//!
//! The buttons carry the signal's ROW ID, and the row does not exist until the
//! signal has been written. Nothing else identifies which card he tapped: two
//! setups on one pair within an hour would be indistinguishable.
//!
//! ## Why the id travels in the button
//!
//! Telegram hands back whatever the button was created with. The signal's row
//! id is the only thing that identifies which card he pressed; a symbol and a
//! time would collide the moment two shapes print on one pair.

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
) -> Result<(), SendError> {
    let keyboard = json!({
        "inline_keyboard": [[
            { "text": "✅ took it",    "callback_data": format!("label:took:{signal_id}") },
            { "text": "❌ skipped it", "callback_data": format!("label:skipped:{signal_id}") },
        ]]
    });

    // **Named, so a tap can never land on the wrong setup.** The group above
    // already carries the sentence as its caption, so this repeats it rather
    // than adding anything — and that repetition is the point: two setups
    // arriving together would otherwise be two identical questions.
    let words = format!("<b>{sentence}</b>\n\nDid you take it?");

    telegram::ask_words(client, &OWNER.to_string(), &words, keyboard).await
}
