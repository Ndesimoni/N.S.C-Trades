//! The two steps both pictures share — drawing it, and getting it to him.

use std::path::Path;

use anyhow::Result;
use nsc_core::levels::{Pair, Timeframe, load_thickness};

use super::super::OWNER;
use crate::review::{self, Drawn};
use crate::telegram;

/// Draws it, so the two callers handle trouble in the same shape.
pub(super) async fn draw(
    client: &reqwest::Client,
    pair: &Pair,
    chart: Timeframe,
    out: &Path,
) -> Result<Drawn> {
    let thickness = load_thickness(Path::new("config/levels.toml"))?;

    review::picture_of(client, pair, thickness, chart, out).await
}

/// To the private chat, not the channel.
///
/// This is him working, not a signal, and mixing the two turns the signal
/// channel into a scratchpad.
pub(super) async fn sent(client: &reqwest::Client, drawn: &Drawn, caption: &str) -> Result<()> {
    telegram::send_to(client, &OWNER.to_string(), &[&drawn.picture], caption)
        .await
        .map_err(Into::into)
}
