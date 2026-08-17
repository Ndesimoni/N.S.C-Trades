//! The card that says something has gone wrong with the bot.
//!
//! **Not about the market.** No pair, no price, no chart — it must never be
//! mistaken for a signal, and the colour answers the only question he has:
//! do I need to get up?
//!
//! ```text
//!     amber   the line is down and it is trying        no
//!     green   it is back                               no
//!     red     it has stopped and will not restart      yes
//!     red     the feed will not serve some pairs       yes
//! ```

use std::path::{Path, PathBuf};

use serde_json::json;

use super::{CardError, fill};

const TEMPLATE: &str = "trouble.html";

/// What has gone wrong, and how much it matters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wrong {
    /// The price line keeps failing. It is still trying.
    LineDown,

    /// It is working again.
    LineBack,

    /// It cannot recover and has stopped.
    Stopped,

    /// **The feed will not send prices for some pairs.**
    ///
    /// The socket is open and the other pairs are watched perfectly, which is
    /// exactly what makes this dangerous: a refused pair looks identical to a
    /// pair where nothing is happening. Levels on it will never fire and
    /// nothing else would ever say so.
    PairsDark,

    /// They are being served again.
    PairsBack,
}

/// The line under the picture.
///
/// **Here, beside the card's own words, so there is one of it.** It was
/// written twice — once for the real message and once for the preview — and
/// the preview showed its own text, so all three states arrived captioned
/// identically and none of them said what had happened.
///
/// The sign carries the same thing the colour does: ⚠️ it is trying, ✅ it is
/// fine, 🛑 it needs him.
pub fn caption(wrong: Wrong) -> &'static str {
    match wrong {
        Wrong::LineDown => {
            "⚠️ <b>The price line is down.</b> Nothing is being watched while this lasts."
        }
        Wrong::LineBack => "✅ <b>The price line is back.</b> Your zones are being watched again.",
        Wrong::Stopped => {
            "🛑 <b>The bot has stopped.</b> Nothing is being watched until you start it."
        }
        Wrong::PairsDark => {
            "🛑 <b>Some pairs are not being watched.</b> The feed will not send prices for them."
        }
        Wrong::PairsBack => "✅ <b>Those pairs are being watched again.</b>",
    }
}

/// Draws it.
///
/// `minutes` is how long the trouble has lasted, where that is known — the
/// card leads with it, because how long is the first thing he wants.
pub fn trouble(
    wrong: Wrong,
    minutes: Option<i64>,
    detail: &str,
    stamp: &str,
    out: &Path,
) -> Result<PathBuf, CardError> {
    let (state, head, since, means, next, big) = match wrong {
        Wrong::LineDown => (
            "down",
            "The price line is down",
            "and it is still not opening.",
            "Nothing is being watched. Price could be sitting in a zone and you would not hear.",
            "It keeps trying. You get a message the moment it is back.",
            "Down",
        ),
        Wrong::LineBack => (
            "back",
            "The price line is back",
            "and it is watching again.",
            "Your zones are being watched again, from now.",
            "Anything that happened while it was down was not seen, and cannot be.",
            "Back",
        ),
        Wrong::Stopped => (
            "stopped",
            "The bot has stopped",
            "and it is not trying again.",
            "Nothing is being watched at all. This one does not fix itself.",
            "Start it again once the reason below is dealt with.",
            "Stopped",
        ),

        // **Red, like Stopped, and for the same reason** — it does not fix
        // itself. The bot is running and the other pairs are watched perfectly,
        // which is exactly what makes this one dangerous: a refused pair looks
        // identical to a pair where nothing is happening.
        Wrong::PairsDark => (
            "dark",
            "Some pairs are not being watched",
            "and the feed will not send prices for them.",
            "Levels on the pairs below will never fire. Everything else is watched as normal.",
            "This does not clear on its own. It needs a plan that carries them, or those pairs removed.",
            "Dark",
        ),

        Wrong::PairsBack => (
            "back",
            "Those pairs are being watched again",
            "and the feed is sending prices for them.",
            "Their levels can fire from now.",
            "Anything that happened while they were dark was not seen, and cannot be.",
            "Back",
        ),
    };

    fill::draw(
        TEMPLATE,
        &[(
            "/*__TROUBLE__*/",
            json!({
                "state":   state,
                "head":    head,
                "since":   since,
                "means":   means,
                "next":    next,
                "detail":  detail,
                "stamp":   stamp,
                "minutes": minutes,
                "big":     big,
            })
            .to_string(),
        )],
        out,
    )
}
