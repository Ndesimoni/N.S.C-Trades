//! Taking levels back off, and stopping a pair altogether.
//!
//! **The other half of `write.rs`.** Both touch the same files, and both keep
//! the comments in them — these files explain what a level is and where the
//! numbers came from, and he is meant to be able to open one and read it.

use std::path::Path;

use super::LevelError;
use super::write::{read, write};
use rust_decimal::Decimal;

use super::{Pair, load_pair};

/// Where a pair goes when he stops watching it.
///
/// A folder, not a delete. `known` only looks at `.toml` files, so anything in
/// here is invisible to the bot — and still on disk.
pub const RETIRED: &str = "removed";

/// Stops watching a pair, by moving its file out of the way.
///
/// **Moved, not deleted.** This is done by tapping a button on a phone, and it
/// throws away every level he has drawn for that pair — months of chart work
/// in one tap. It goes to `config/pairs/removed/` and comes back by being
/// moved out again.
///
/// Gives back where it went, so the reply can say.
pub fn retire(folder: &Path, name: &str) -> Result<std::path::PathBuf, LevelError> {
    let file = folder.join(format!("{name}.toml"));
    let away = folder.join(RETIRED);

    std::fs::create_dir_all(&away).map_err(|trouble| LevelError::CannotWrite {
        path: away.display().to_string(),
        detail: trouble.to_string(),
    })?;

    // Numbered, so retiring the same pair twice does not overwrite the first
    // set of levels with the second. He may add a pair back, draw it again,
    // and drop it again — and the first set is still the one he spent an
    // evening on.
    let landed = free_name(&away, name);

    std::fs::rename(&file, &landed).map_err(|trouble| LevelError::CannotWrite {
        path: file.display().to_string(),
        detail: trouble.to_string(),
    })?;

    Ok(landed)
}

/// A name in `away` that is not taken.
///
/// **Counted, not timestamped.** `nsc-core` may not ask what time it is — that
/// is the rule that lets the backtester run this code — so it counts what is
/// already there instead.
fn free_name(away: &Path, name: &str) -> std::path::PathBuf {
    let first = away.join(format!("{name}.toml"));

    if !first.exists() {
        return first;
    }

    (2..)
        .map(|nth| away.join(format!("{name}-{nth}.toml")))
        .find(|path| !path.exists())
        .unwrap_or(first)
}

/// Takes the last `count` levels back off a pair.
///
/// **Cuts the text, does not rewrite the file** — same reason as `save`. The
/// comments in these files explain what a level is, and rewriting would delete
/// them without a word.
pub fn undo(folder: &Path, name: &str, count: usize) -> Result<Pair, LevelError> {
    let file = folder.join(format!("{name}.toml"));

    let text = read(&file)?;

    // Every level is one `[[level]]` block. Cut from the start of the last
    // `count` of them to the end.
    let starts: Vec<usize> = text
        .match_indices("\n[[level]]")
        .map(|(at, _)| at)
        .collect();

    let keep = match starts.len().checked_sub(count) {
        Some(remaining) => starts.get(remaining).copied().unwrap_or(text.len()),
        None => starts.first().copied().unwrap_or(text.len()),
    };

    write(&file, &text[..keep])?;

    load_pair(&file)
}

/// What taking a level off did.
pub struct TookOff {
    /// The pair as it now stands.
    pub pair: Pair,

    /// **Whether that price was actually on it.**
    ///
    /// A price that is not there changes nothing, which is right — but the
    /// reply is built from this, and without it he was told "1.28 taken off"
    /// while the file sat untouched. Old keyboards stay tappable in Telegram
    /// forever, so a stale button is one thumb away.
    pub was_there: bool,
}

/// Takes one particular level off a pair.
///
/// **Undo only reaches what the last message added.** That covers a typo the
/// moment it happens; it does nothing for "that 1.15 from last week was
/// wrong", which is the one he actually needs.
///
/// Matched on the PRICE AS A NUMBER, the same way saving refuses a duplicate —
/// he may have typed 1.15 where the file says 1.15000, and as text those are
/// two different levels.
///
/// Cuts the one block, and leaves everything else in the file exactly as it
/// was, comments and all.
pub fn take_off(folder: &Path, name: &str, price: Decimal) -> Result<TookOff, LevelError> {
    let file = folder.join(format!("{name}.toml"));
    let text = read(&file)?;

    let head_ends = text.find("\n[[level]]").unwrap_or(text.len());
    let (head, blocks) = text.split_at(head_ends);

    let mut was_there = false;

    let kept: String = blocks
        .split_inclusive("\n[[level]]")
        .scan(String::new(), |carry, piece| {
            // `split_inclusive` puts the marker at the END of each piece, so a
            // block is the tail of one piece and the head of the next. Carrying
            // the marker forward keeps each block whole.
            let block = format!("{carry}{}", piece.trim_end_matches("[[level]]"));
            *carry = if piece.ends_with("[[level]]") {
                "[[level]]".to_string()
            } else {
                String::new()
            };

            Some(block)
        })
        .filter(|block| {
            let this_one = says_price(block, price);
            was_there |= this_one;
            !this_one
        })
        .collect();

    write(&file, &format!("{head}{kept}"))?;

    Ok(TookOff {
        pair: load_pair(&file)?,
        was_there,
    })
}

/// Is this the block for that price?
fn says_price(block: &str, price: Decimal) -> bool {
    block
        .lines()
        .filter_map(|line| line.trim().strip_prefix("price = "))
        .filter_map(|value| value.trim().trim_matches('"').parse::<Decimal>().ok())
        .any(|written| written == price)
}

/// Every pair he has stopped, newest name first.
///
/// **The names are the files as they sit**, so `GBPUSD-2` is a real answer —
/// he stopped that pair twice and both sets are kept.
pub fn retired(folder: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(folder.join(RETIRED)) else {
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

/// Puts a stopped pair back.
///
/// `name` is the file as it sits in `removed/` — `GBPUSD`, or `GBPUSD-2` if he
/// stopped that pair twice. It comes back under the pair's own name, which is
/// what the file inside says.
///
/// **It refuses to land on a pair he is already watching.** Restoring
/// `GBPUSD-2` over a live `GBPUSD` would replace levels he is using with ones
/// he put aside, and nothing would say so.
pub fn restore(folder: &Path, name: &str) -> Result<String, LevelError> {
    let from = folder.join(RETIRED).join(format!("{name}.toml"));

    // The pair's real name comes from the file, not from what it is called on
    // disk. `GBPUSD-2.toml` is still GBPUSD.
    let pair = load_pair(&from)?;
    let under = pair.symbol.replace('/', "");
    let to = folder.join(format!("{under}.toml"));

    if to.exists() {
        return Err(LevelError::AlreadyThere(pair.symbol));
    }

    std::fs::rename(&from, &to).map_err(|trouble| LevelError::CannotWrite {
        path: to.display().to_string(),
        detail: trouble.to_string(),
    })?;

    Ok(pair.symbol)
}
