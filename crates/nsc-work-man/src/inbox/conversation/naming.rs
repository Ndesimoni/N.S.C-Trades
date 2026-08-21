//! Turning a name he typed into a pair, or explaining why it is not one.

use anyhow::Result;

use super::super::checking::{self, Verdict};
use super::super::talking::{plainly, say};
use super::adding::Adding;

/// The pair name, if IBKR will serve it — and the reply if it will not.
///
/// **`Ok(None)` means he has been answered and nothing else should happen.**
/// Nothing is written, and he stays in naming mode so his next message is
/// read as another go at the name rather than as a stray.
pub(super) async fn checked(
    client: &reqwest::Client,
    token: &str,
    text: &str,
    adding: &mut Adding,
) -> Result<Option<String>> {
    let typed = text.to_uppercase();

    match checking::pair(&typed).await {
        Verdict::Fine => Ok(Some(typed)),

        Verdict::Never(why) => {
            // **Say nothing was saved, in those words.** The old reply to a
            // pair that could not be drawn was "the levels are safe", which
            // told him it had worked.
            let words = format!(
                "✗ <b>{typed}</b> — IBKR has never heard of it.\n\n\
                 <i>{}</i>\n\n\
                 <b>Nothing was saved.</b> Check the spelling and send it again.",
                plainly(&why)
            );

            adding.naming = true;
            say(client, token, &words, None).await?;

            Ok(None)
        }

        // **A gateway that is down is not a pair that does not exist.**
        Verdict::CouldNotAsk(why) => {
            let words = format!(
                "Could not check <b>{typed}</b> — IBKR is not answering.\n\n\
                 <i>{}</i>\n\n\
                 <b>Nothing was saved.</b> Try again once TWS is logged in.",
                plainly(&why)
            );

            adding.naming = true;
            say(client, token, &words, None).await?;

            Ok(None)
        }
    }
}
