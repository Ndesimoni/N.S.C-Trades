//! Showing him where his levels landed.
//!
//! Reading a price back at him only proves he can read his own typing. A
//! picture shows the PLACE, which is the thing that actually goes wrong — and
//! it is how he reads a chart anyway.

use std::path::Path;

use crate::{review, telegram};
use anyhow::Result;
use nsc_core::levels::{Pair, load_thickness};

use super::OWNER;
use super::talking::say;

/// Draws the pair's levels and sends the picture.
///
/// If it cannot — the feed is down, the pair is not one they carry — that is
/// worth saying, but the levels are already saved and safe. A picture failing
/// must not look like a level being lost.
pub async fn show(client: &reqwest::Client, token: &str, pair: &Pair) -> Result<()> {
    let thickness = load_thickness(Path::new("config/levels.toml"))?;
    let picture = Path::new("preview").join("just-saved.png");

    match review::picture_of(client, pair, thickness, &picture).await {
        Ok(_) => {
            // To the private chat, not the channel. This is him working, not a
            // signal, and mixing the two turns the channel into a scratchpad.
            telegram::send_to(
                client,
                &OWNER.to_string(),
                &[&picture],
                &format!(
                    "📍 <b>{}</b> — here is where your levels landed.",
                    pair.symbol
                ),
            )
            .await
            .map_err(Into::into)
        }
        Err(trouble) => {
            println!("  -> Could not draw it: {trouble:#}");
            say(
                client,
                token,
                "Saved. Could not draw the chart just now — the levels are safe.",
                None,
            )
            .await
        }
    }
}
