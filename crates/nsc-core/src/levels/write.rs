//! Saving what he sends, and starting a pair's file when it is new.
//!
//! **Adding only.** Taking away — a level, or a whole pair — is `remove.rs`.
//!
//! **Levels are appended as text, not by rewriting the file.** These files have
//! comments in them explaining what a level is and where the numbers came from,
//! and rewriting would quietly delete all of it. He is meant to be able to open
//! one and read it.

use std::path::Path;

use super::LevelError;
use rust_decimal::Decimal;

use super::naming::{nightly_break, with_slash};
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

/// What came of saving.
#[derive(Debug, Clone)]
pub struct Saved {
    /// The pair as it now stands, so a reply can say what it holds rather than
    /// only what was just added.
    pub pair: Pair,

    /// How many were new.
    pub added: usize,

    /// The ones he already had, and which chart they are already on.
    ///
    /// **The timeframe is carried so the reply can say it.** Refusing a level
    /// he has just re-sent on a different chart is right, but doing it
    /// silently would leave him thinking he had moved it.
    pub already: Vec<(Decimal, Timeframe)>,
}

/// Adds levels to a pair, starting its file if this is the first time.
///
/// **A price he already has is not added again, whatever chart it comes in
/// on.** He sent the same three euro levels twice and got both copies, so one
/// line on his chart became two bands — two alerts, two closes, and a
/// heartbeat card claiming seven levels where he had drawn four.
///
/// The timeframe is deliberately not part of what makes a level unique. He
/// draws ONE line at 1.15000; sending it again off the daily chart has not
/// changed anything about it, and a second band round the same line is the
/// same duplicate wearing a different label — a wider one and a narrower one,
/// firing twice as price passes through.
///
/// Repeats inside one message are dropped too. Tapping send twice is the
/// commonest way it happens.
pub fn save(
    folder: &Path,
    name: &str,
    timeframe: Timeframe,
    prices: &[Decimal],
    digits: u32,
) -> Result<Saved, LevelError> {
    let file = folder.join(format!("{name}.toml"));

    if !file.exists() {
        std::fs::create_dir_all(folder).map_err(|trouble| LevelError::CannotWrite {
            path: folder.display().to_string(),
            detail: trouble.to_string(),
        })?;

        write(&file, &opening(name, digits))?;
    }

    // Every price he already holds, on ANY chart, with the chart it is on.
    //
    // Compared as NUMBERS, not as text: 1.15 and 1.15000 are the same line and
    // he may type either.
    let mut held: Vec<(Decimal, Timeframe)> = load_pair(&file)?
        .levels
        .iter()
        .map(|line| (line.price, line.timeframe))
        .collect();

    let mut text = read(&file)?;
    let mut added = 0;
    let mut already = Vec::new();

    for price in prices {
        if let Some(had) = held.iter().find(|(at, _)| at == price) {
            already.push(*had);
            continue;
        }

        held.push((*price, timeframe));
        added += 1;

        text.push_str(&format!(
            "\n[[level]]\ntimeframe = \"{}\"\nprice = \"{price}\"\n",
            timeframe.name()
        ));
    }

    write(&file, &text)?;

    Ok(Saved {
        pair: load_pair(&file)?,
        added,
        already,
    })
}

pub(super) fn read(file: &Path) -> Result<String, LevelError> {
    std::fs::read_to_string(file).map_err(|trouble| LevelError::CannotRead {
        path: file.display().to_string(),
        detail: trouble.to_string(),
    })
}

/// Writes a pair's file **in one move**.
///
/// The text goes to a file beside it and is then renamed over the top, which
/// the filesystem does as a single step.
///
/// **Because two things read these files now.** The inbox writes them while
/// the watcher is reading them, and a plain write is not one step — the
/// watcher could read a file halfway through being replaced and find half a
/// level. It would recover on the next look, but "recovers in ten minutes" is
/// not the same as "cannot happen".
pub(super) fn write(file: &Path, text: &str) -> Result<(), LevelError> {
    let trouble_at = |path: &Path, trouble: std::io::Error| LevelError::CannotWrite {
        path: path.display().to_string(),
        detail: trouble.to_string(),
    };

    // Beside it, not in a temp folder — a rename is only one step when both
    // ends are on the same filesystem.
    let part = file.with_extension("toml.part");

    std::fs::write(&part, text).map_err(|trouble| trouble_at(&part, trouble))?;
    std::fs::rename(&part, file).map_err(|trouble| trouble_at(file, trouble))
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
