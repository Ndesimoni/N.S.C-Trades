//! **Reading the levels again**, keeping what has not changed.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Result;
use nsc_core::levels::{Thickness, Watch, known, load_pair};
use nsc_data::sources::ibkr::IbkrConnection;

use super::super::{Watching, bands, pulse};
use crate::places::{OWNER, PAIRS, PREVIEW};
use crate::{card, telegram};

/// What reading the levels again produced.
pub struct Reloaded {
    pub watching: HashMap<String, Watching>,

    /// The pairs whose bands were built fresh, so he can be told — and so the
    /// greeting knows which ones it owes a "price is already here" report.
    pub armed: Vec<String>,

    /// Pairs whose bands could not be sized this time, because the feed would
    /// not answer. **Not an error.** See the note on `again`.
    pub not_sized: Vec<String>,
}

/// **Which watched pair a file belongs to**, by name.
///
/// `known` lists FILE STEMS — `XAUUSD` — and the watch list is keyed by the
/// pair's SYMBOL — `XAU/USD`. A file is named by taking the slashes out of the
/// symbol, so putting them back is the way home.
///
/// ## The bug this replaces
///
/// The unreadable-file branch looked the stem up in the watch list directly.
/// Every symbol has a slash in it and no stem does, **so the lookup could
/// never hit** — and the branch whose whole job is "keep watching what this
/// pair already had" silently dropped it instead, while printing that it was
/// leaving it alone.
///
/// It only shows when a file cannot be read: he is halfway through editing it,
/// or a write from his phone was caught mid-way. The pair then stops being
/// watched with nothing to say so, and it does not come back on its own — the
/// next reload only happens when a file CHANGES, and nothing has to change
/// again.
///
/// Compared by stripping rather than by rebuilding the symbol, because
/// stripping is what actually made the file name. Found 1 September 2026.
pub(super) fn watched_as(old: &HashMap<String, Watching>, stem: &str) -> Option<String> {
    old.keys()
        .find(|symbol| symbol.replace('/', "") == stem)
        .cloned()
}

/// Reads the levels again, keeping what has not changed.
///
/// **A pair whose levels are untouched keeps the `Watch` it already had.**
/// Rebuilt, it would forget which zones price is sitting in — and then
/// announce every one of them again as though it had just arrived.
///
/// **A pair that cannot be sized keeps the bands it already had.** This used
/// to give back an error, and that error travelled all the way out of `run`
/// and stopped the bot: he sent a level from his phone, the feed was slow
/// for ten seconds, and the bot said "stopped" and quit. Nothing was watched
/// again until he noticed and restarted it — and the weekend, when he does his
/// chart work, is exactly when he would not.
///
/// Gives back the new set, whether anything was armed, and which pairs the
/// feed would not size.
pub async fn again(
    ibkr: &IbkrConnection,
    thickness: Thickness,
    mut old: HashMap<String, Watching>,
) -> Result<Reloaded> {
    let mut now: HashMap<String, Watching> = HashMap::new();
    let mut armed = Vec::new();
    let mut not_sized = Vec::new();

    for name in known(Path::new(PAIRS)) {
        let pair = match load_pair(&Path::new(PAIRS).join(format!("{name}.toml"))) {
            Ok(pair) => pair,

            // One unreadable file must not take the others down with it. He
            // may be halfway through editing it by hand, or a write from his
            // phone may have been caught in the middle.
            Err(trouble) => {
                match watched_as(&old, &name).and_then(|symbol| old.remove(&symbol)) {
                    Some(kept) => {
                        eprintln!("{name} — cannot read it, still watching what it had: {trouble}");
                        now.insert(kept.pair.symbol.clone(), kept);
                    }

                    None => eprintln!(
                        "{name} — cannot read it, and it was not being watched: {trouble}"
                    ),
                }

                continue;
            }
        };

        if let Some(kept) = old.remove(&pair.symbol)
            && kept.pair.levels == pair.levels
        {
            now.insert(pair.symbol.clone(), kept);
            continue;
        }

        let found = match bands::for_pair(ibkr, &pair, thickness).await {
            Ok(found) => found,

            // The feed would not answer. Keep whatever this pair already had
            // and try again on the next look — the alternative is stopping the
            // bot over a hiccup.
            Err(trouble) => {
                eprintln!("{name} — could not size its bands: {trouble:#}");
                not_sized.push(pair.symbol.clone());

                if let Some(kept) = old.remove(&pair.symbol) {
                    now.insert(kept.pair.symbol.clone(), kept);
                }
                continue;
            }
        };

        if found.is_empty() {
            println!("{name} — no levels, skipping");
            continue;
        }

        println!("{} — now watching {} level(s)", pair.symbol, found.len());
        armed.push(pair.symbol.clone());

        let watch = Watch::over(found, pair.reach_share(thickness));
        now.insert(pair.symbol.clone(), Watching { pair, watch });
    }

    Ok(Reloaded {
        watching: now,
        armed,
        not_sized,
    })
}

/// Tells him the watcher has picked the new levels up.
///
/// **The inbox already drew him the picture** of where they landed, with the
/// pair on it and the bands in his colours. This says the one thing that
/// picture cannot: that they are now being WATCHED. Saved and armed were two
/// separate states and nothing told him which one he had.
///
/// No pair names, no counts. He has just sent it — he knows what he sent, and
/// the picture that came back said so. Repeating it back is a second message
/// telling him something he already had.
pub async fn say_it_is_armed(
    client: &reqwest::Client,
    watching: &HashMap<String, Watching>,
    pulse: &mut pulse::Pulse,
) -> anyhow::Result<()> {
    let pairs = watching.len();
    let zones: usize = watching.values().map(|seen| seen.watch.count()).sum();

    // **Chrome runs off the price loop.** Drawing is a blocking wait of two to
    // ten seconds; left here it holds a Tokio worker for all of it, which
    // stops everything on the one-core box this is meant to be hosted on.
    let picture = tokio::task::spawn_blocking(move || {
        card::armed(pairs, zones, &PathBuf::from(PREVIEW).join("armed.png"))
    })
    .await??;

    telegram::send_to(
        client,
        &OWNER.to_string(),
        &[&picture],
        "📐 <b>Got it.</b> Your levels are live.",
    )
    .await?;

    pulse.spoke(chrono::Utc::now());

    Ok(())
}
