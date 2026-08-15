//! Saving what he sends, and starting a pair's file when it is new.
//!
//! **Levels are appended as text, not by rewriting the file.** These files have
//! comments in them explaining what a level is and where the numbers came from,
//! and rewriting would quietly delete all of it. He is meant to be able to open
//! one and read it.

use std::path::Path;

use anyhow::{Context, Result};
use rust_decimal::Decimal;

use super::{Pair, Timeframe, load_pair};

/// Every pair that has a file. **The files are the list** — there is no second
/// one to keep in sync.
pub fn known(folder: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(folder) else {
        return Vec::new();
    };

    let mut names: Vec<String> = entries
        .flatten()
        .filter(|entry| entry.path().extension().is_some_and(|kind| kind == "toml"))
        .filter_map(|entry| entry.path().file_stem()?.to_str().map(str::to_owned))
        .collect();

    names.sort();
    names
}

/// Adds levels to a pair, starting its file if this is the first time.
///
/// Gives back the pair as it now stands, so the reply can say what it holds
/// rather than only what was just added.
pub fn save(
    folder: &Path,
    name: &str,
    timeframe: Timeframe,
    prices: &[Decimal],
    digits: u32,
) -> Result<Pair> {
    let file = folder.join(format!("{name}.toml"));

    if !file.exists() {
        std::fs::create_dir_all(folder)
            .with_context(|| format!("could not make {}", folder.display()))?;

        std::fs::write(&file, opening(name, digits))
            .with_context(|| format!("could not start {}", file.display()))?;
    }

    let mut text = std::fs::read_to_string(&file)
        .with_context(|| format!("could not read {}", file.display()))?;

    for price in prices {
        text.push_str(&format!(
            "\n[[level]]\ntimeframe = \"{}\"\nprice = \"{price}\"\n",
            timeframe.name()
        ));
    }

    std::fs::write(&file, text).with_context(|| format!("could not write {}", file.display()))?;

    load_pair(&file)
}

/// A brand new pair's file, with what can be worked out from its name.
fn opening(name: &str, digits: u32) -> String {
    let symbol = with_slash(name);
    let nightly = nightly_break(name);

    format!(
        "# ── {symbol} ─────────────────────────────────────────────────────\n\
         #\n\
         # THIS FILE IS WHY THE PAIR IS WATCHED. Delete it and it stops.\n\
         #\n\
         # Started by the bot when he first sent a level for it. The two\n\
         # settings below were WORKED OUT FROM THE NAME, not checked — correct\n\
         # them if the pair behaves differently.\n\
         # ───────────────────────────────────────────────────────────────────\n\
         \n\
         symbol = \"{symbol}\"\n\
         digits = {digits}\n\
         nightly_break_minutes = {nightly}\n\
         \n\
         \n\
         # ── The levels ──────────────────────────────────────────────────────\n\
         #\n\
         # ONE PRICE EACH. He draws a line; the band comes from the thicknesses\n\
         # in config/levels.toml — 0.35 of a weekly candle, 0.46 of a daily.\n\
         # ───────────────────────────────────────────────────────────────────\n"
    )
}

/// `GBPUSD` is how he types it. `GBP/USD` is how the feed wants it.
pub fn with_slash(name: &str) -> String {
    if name.contains('/') || name.len() != 6 {
        return name.to_string();
    }

    format!("{}/{}", &name[..3], &name[3..])
}

/// How many decimals a pair is quoted to, from its name.
///
/// **Worked out, not checked.** Enough to start a file with, and the file says
/// so.
pub fn digits_for(name: &str) -> u32 {
    let upper = name.to_uppercase();

    if upper.starts_with("XAU") || upper.starts_with("XAG") || upper.starts_with("XCU") {
        2
    } else if upper.contains("JPY") {
        3
    } else {
        5
    }
}

/// Metals and oil shut for an hour at 17:00 New York every weekday. Spot forex
/// runs straight through.
fn nightly_break(name: &str) -> i64 {
    let upper = name.to_uppercase();

    if upper.starts_with('X') || upper.contains("OIL") {
        60
    } else {
        0
    }
}
