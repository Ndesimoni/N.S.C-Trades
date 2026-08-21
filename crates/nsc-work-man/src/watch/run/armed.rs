//! Reading the levels again, and telling him they are being watched.

use std::collections::HashMap;

use anyhow::Result;
use nsc_core::levels::Thickness;
use nsc_data::sources::ibkr::IbkrConnection;

use crate::watch::{Kit, Watching, pulse, reload};

/// Reads the levels again and tells him which are now being watched.
pub(super) async fn armed(
    client: &reqwest::Client,
    ibkr: &IbkrConnection,
    thickness: Thickness,
    watching: HashMap<String, Watching>,
    kit: &mut Kit,
) -> Result<HashMap<String, Watching>> {
    let fresh = reload::again(ibkr, thickness, watching).await?;

    // Said out loud, because from his side a pair that would not size is
    // silent — the level is in the file, the bot is running, and nothing is
    // watching it.
    if !fresh.not_sized.is_empty() {
        eprintln!(
            "Could not size {} this time. Keeping what they had and trying again.",
            fresh.not_sized.join(", ")
        );
    }

    // **A pair built fresh is owed a report of where price already is.**
    //
    // He usually draws a level BECAUSE price is near it. Its `Watch` starts
    // over, so the first price is only a baseline and produces no arrival —
    // and the session had already been greeted, so nothing said price was
    // sitting in the zone he had just drawn. He got "your levels are live"
    // and then silence.
    //
    // Only the rebuilt pairs are forgotten, so the others are not announced
    // to him a second time.
    for symbol in &fresh.armed {
        kit.awake.forget(symbol);
    }

    // **A card that will not draw is not a reason to stop.** This used `?`,
    // so Chrome refusing to start — because his own browser held the profile
    // — killed the bot at startup, on the one message that only says "your
    // levels are live". The levels ARE live either way. Say what went wrong
    // and carry on watching them.
    if !fresh.armed.is_empty()
        && let Err(trouble) = reload::say_it_is_armed(client, &fresh.watching, &mut kit.pulse).await
    {
        eprintln!("Could not say the levels are armed: {trouble:#}");
    }

    Ok(fresh.watching)
}

/// The line he gets when a level he sent goes live.
///
/// **Public so `--bin cards` can show it.** Everything the bot says should be
/// something he can look at without waiting for it to happen.
///
/// It builds the watching set fresh rather than being handed one, because the
/// caller is a preview program with no bot running behind it.
pub async fn say_it_is_armed(
    client: &reqwest::Client,
    ibkr: &IbkrConnection,
    thickness: Thickness,
) -> Result<()> {
    let fresh = reload::again(ibkr, thickness, HashMap::new()).await?;
    let mut pulse = pulse::Pulse::new();

    reload::say_it_is_armed(client, &fresh.watching, &mut pulse).await
}
