//! Where the levels he just sent landed.

use std::path::Path;

use anyhow::Result;
use nsc_core::levels::{Pair, Timeframe};

use super::super::talking::say;
use super::sending::{draw, sent};
use crate::places::PREVIEW;

/// Draws the pair's levels and sends the picture.
///
/// **Always the weekly**, because this is the reply to having just saved
/// something and the question is "did that land where I drew it". Only the
/// weekly is wide enough to hold levels drawn years apart.
///
/// If it cannot draw — the feed is down, the pair is not one they carry — that
/// is worth saying, but the levels are already saved and safe. A picture
/// failing must not look like a level being lost.
pub async fn show(client: &reqwest::Client, token: &str, pair: &Pair) -> Result<()> {
    let out = Path::new(PREVIEW).join("just-saved.png");
    let caption = format!(
        "📍 <b>{}</b> — here is where your levels landed.",
        pair.symbol
    );

    match draw(pair, Timeframe::Weekly, &out).await {
        Ok(drawn) => sent(client, &drawn, &caption).await,
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
