//! Taking levels back off, and stopping a pair altogether.
//!
//! **The other half of `write.rs`.** Both touch the same files, and both keep
//! the comments in them — these files explain what a level is and where the
//! numbers came from, and he is meant to be able to open one and read it.

use std::path::Path;

use super::LevelError;
use super::write::{read, write};
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
