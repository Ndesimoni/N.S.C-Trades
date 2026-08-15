//! The card that says a level he just sent is being watched.
//!
//! **A receipt, not a report.** He has sent something and wants one thing
//! back: yes, it is live.
//!
//! It names nothing. The inbox has already sent him a picture of where the
//! bands landed, with the pair on it in his own colours — repeating that is a
//! second message telling him what he already had. The only thing this adds is
//! that they are being WATCHED, and saved and armed were two separate states
//! with nothing to tell them apart.

use std::path::{Path, PathBuf};

use serde_json::json;

use super::{CardError, fill};

const TEMPLATE: &str = "armed.html";

/// Draws it.
///
/// The counts are the whole of it: he can see the number went up without being
/// told which pair, which he knows.
pub fn armed(pairs: usize, zones: usize, out: &Path) -> Result<PathBuf, CardError> {
    fill::draw(
        TEMPLATE,
        &[(
            "/*__ARMED__*/",
            json!({ "pairs": pairs, "zones": zones }).to_string(),
        )],
        out,
    )
}
