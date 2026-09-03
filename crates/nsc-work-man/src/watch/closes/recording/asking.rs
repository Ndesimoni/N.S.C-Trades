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
//! ## Nothing is written above them
//!
//! There was a line naming the setup, and he took it out on 4 September 2026:
//! *"we still have the same information inside the main card — there is no
//! need for another section before the buttons."* He is right; the container's
//! caption already says it, and saying it twice made the buttons look like
//! they belonged to a second thing.
//!
//! **Telegram will not send a message with no text at all**, and it is fussy
//! about what counts as none. Asked directly, on 4 September 2026:
//!
//! ```text
//!     U+200B  zero-width space        refused
//!     U+00A0  non-breaking space      refused
//!     U+2800  blank braille pattern   refused
//!     U+00B7  middle dot              accepted, and visible
//!     U+2063  invisible separator     ACCEPTED, and draws nothing
//! ```
//!
//! So the message is one `U+2063`. The bubble is there and empty, and only the
//! two buttons show.
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
pub async fn ask(client: &reqwest::Client, signal_id: i64) -> Result<(), SendError> {
    // **Padded, because Telegram sizes a button from its label.** There is no
    // width to set. Left to themselves the two sit narrow in the middle of the
    // screen and look like an afterthought under a full-width card; the spaces
    // push them out so the row reads as part of the same block.
    let keyboard = json!({
        "inline_keyboard": [[
            {
                "text": "\u{2003}\u{2003}✅ took it\u{2003}\u{2003}",
                "callback_data": format!("label:took:{signal_id}"),
            },
            {
                "text": "\u{2003}\u{2003}❌ skipped it\u{2003}\u{2003}",
                "callback_data": format!("label:skipped:{signal_id}"),
            },
        ]]
    });

    // **One invisible separator, and that is the whole message.** The three
    // obvious blanks are all refused as empty; this one is not. See above.
    //
    // The sentence is not passed in at all any more: the container above
    // carries it as its caption, and a parameter this does not use would be a
    // lie about what it needs.

    telegram::ask_words(client, &OWNER.to_string(), "\u{2063}", keyboard).await
}
