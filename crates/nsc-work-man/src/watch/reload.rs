//! Noticing that he has sent a new level, without being restarted.
//!
//! **The levels used to be read once, at startup.** He would send one from his
//! phone, the inbox would save it correctly, the file would be right — and the
//! watcher would never look again. Nothing said so. The level simply did
//! nothing until the next restart, which might be days.

use std::collections::HashMap;
use std::path::Path;
use std::time::SystemTime;

use anyhow::Result;
use nsc_core::levels::{Thickness, Watch, known, load_pair};

use super::{OWNER, PAIRS, Watching, bands, pulse};
use crate::telegram;

/// Remembers how the levels folder looked, so a change can be spotted.
///
/// **By the clock on the files, not by reading them.** Parsing every pair file
/// every ten minutes to find out that nothing happened is work done for
/// nothing, and nothing is the normal answer.
pub struct Files {
    newest: Option<SystemTime>,
    count: usize,
}

impl Files {
    pub fn look() -> Self {
        let (newest, count) = state();

        Files { newest, count }
    }

    /// Has anything been added, changed or removed since the last look?
    pub fn changed(&mut self) -> bool {
        let (newest, count) = state();

        // The count matters as well as the clock. A file deleted leaves every
        // remaining timestamp exactly as it was.
        let moved = newest != self.newest || count != self.count;

        self.newest = newest;
        self.count = count;

        moved
    }
}

/// The newest change in the folder, and how many files are in it.
fn state() -> (Option<SystemTime>, usize) {
    let names = known(Path::new(PAIRS));

    let newest = names
        .iter()
        .filter_map(|name| {
            std::fs::metadata(Path::new(PAIRS).join(format!("{name}.toml")))
                .and_then(|about| about.modified())
                .ok()
        })
        .max();

    (newest, names.len())
}

/// Reads the levels again, keeping what has not changed.
///
/// **A pair whose levels are untouched keeps the `Watch` it already had.**
/// Rebuilt, it would forget which zones price is sitting in — and then
/// announce every one of them again as though it had just arrived.
///
/// Gives back the new set, and the names of pairs that were armed or re-armed.
pub async fn again(
    client: &reqwest::Client,
    thickness: Thickness,
    mut old: HashMap<String, Watching>,
) -> Result<(HashMap<String, Watching>, Vec<String>)> {
    let mut now: HashMap<String, Watching> = HashMap::new();
    let mut fresh = Vec::new();

    for name in known(Path::new(PAIRS)) {
        let pair = match load_pair(&Path::new(PAIRS).join(format!("{name}.toml"))) {
            Ok(pair) => pair,

            // One unreadable file must not take the others down with it. He
            // may be halfway through editing it by hand.
            Err(trouble) => {
                eprintln!("{name} — cannot read it, leaving it alone: {trouble}");

                if let Some(kept) = old.remove(&name) {
                    now.insert(kept.pair.symbol.clone(), kept);
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

        let found = bands::for_pair(client, &pair, thickness).await?;
        if found.is_empty() {
            println!("{name} — no levels, skipping");
            continue;
        }

        println!("{} — now watching {} level(s)", pair.symbol, found.len());
        fresh.push(format!("{} · {} levels", pair.symbol, found.len()));

        let watch = Watch::over(found, pair.reach(thickness));
        now.insert(pair.symbol.clone(), Watching { pair, watch });
    }

    Ok((now, fresh))
}

/// Tells him the watcher has picked the new levels up.
///
/// **The inbox already drew him the picture** of where they landed. This says
/// the different thing: that they are now being WATCHED. Saved and armed were
/// two separate states and nothing said which one he had — the file was right
/// and the level did nothing until the next restart.
pub async fn say_it_is_armed(
    client: &reqwest::Client,
    armed: &[String],
    pulse: &mut pulse::Pulse,
) -> anyhow::Result<()> {
    let words = format!(
        "📐 <b>Armed.</b> Now watching:\n\n{}",
        armed
            .iter()
            .map(|line| format!("· {line}"))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    telegram::send_words(client, &OWNER.to_string(), &words).await?;
    pulse.spoke(chrono::Utc::now());

    Ok(())
}
