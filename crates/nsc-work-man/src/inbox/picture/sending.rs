//! The two steps both pictures share — drawing it, and getting it to him.

use std::path::Path;

use anyhow::Result;
use nsc_core::levels::{Pair, Timeframe, load_thickness};
use nsc_data::sources::ibkr::IbkrConnection;

use crate::places::{OWNER, THICKNESS};
use crate::review::{self, Drawn};
use crate::telegram;

/// Draws it, so the two callers handle trouble in the same shape.
///
/// **It opens its own line to TWS, and only when he asks.** IBKR allows one
/// connection per client id and the watcher holds the one from `.env` for
/// weeks at a time — coming in on the same id would throw the watcher off the
/// feed to draw a chart.
///
/// Opened per request rather than held. `/chart` happens a few times a day,
/// connecting takes a second or two, and drawing the card takes several — so
/// the cost is invisible, and a line that is never held cannot go stale.
pub(super) async fn draw(pair: &Pair, chart: Timeframe, out: &Path) -> Result<Drawn> {
    let thickness = load_thickness(Path::new(THICKNESS))?;
    let ibkr = IbkrConnection::connect_beside().await?;

    review::picture_of(&ibkr, pair, thickness, chart, out).await
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
