//! The card that says something has gone wrong with the bot.
//!
//! **Not about the market.** No pair, no price, no chart — it must never be
//! mistaken for a signal, and the colour answers the only question he has:
//! do I need to get up?
//!
//!     amber   the line is down and it is trying        no
//!     green   it is back                               no
//!     red     it has stopped and will not restart      yes

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
