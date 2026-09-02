//! **Which settings produced this row.**
//!
//! A short hash of the config files the rules are read from.
//!
//! ## Why it has to be stored
//!
//! Without it, *"these signals came back at 38%"* is unanswerable — 38% under
//! **which thresholds?** Every number in `config/` moves over time, and a
//! result measured under one set of settings says nothing about another.
//!
//! The bot already refuses to reload settings while it is running, precisely
//! so that "which rules produced which signals" has an answer. This is that
//! promise written down.
//!
//! **Worked out once, at startup.** Reading four files on every candle would
//! be four reads a minute for an answer that cannot change while the bot is
//! up — because it refuses to reload.

use std::path::Path;

use crate::places::{PATTERNS, STRATEGY, THICKNESS};

/// The settings rung 3 actually depends on.
///
/// **Not every file in `config/`.** `when.toml` decides when the bot speaks
/// and `news.toml` what the calendar says; neither changes what a shape at a
/// level IS. Hashing them would make the version move for reasons that have
/// nothing to do with the rules, and then it stops meaning anything.
const READS: [&str; 3] = [STRATEGY, PATTERNS, THICKNESS];

/// A short hash of those files, as they are on disk right now.
///
/// **Unreadable counts as a version too.** A file that cannot be read is a
/// real state the bot runs in — it falls back to defaults — and calling that
/// `unknown` is honest, where refusing to produce a version would stop the row
/// being written at all.
pub(in crate::watch::closes) fn rules_version() -> String {
    let mut soup = String::new();

    for path in READS {
        match std::fs::read_to_string(Path::new(path)) {
            Ok(text) => soup.push_str(&text),
            Err(_) => soup.push_str("<unreadable>"),
        }

        // A separator, so moving a line from one file to the next changes the
        // answer. Without it two different sets of files can hash the same.
        soup.push('\0');
    }

    short(&soup)
}

/// **A twelve-character hash, and it is not a cryptographic one.**
///
/// It only has to say "these settings are not those settings". FxHash-style
/// mixing over the bytes is enough for that and costs nothing, where pulling
/// in a hashing crate for a label would be a dependency earning its keep once.
fn short(text: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;

    for byte in text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100_0000_01b3);
    }

    // **Masked to twelve hex digits, not padded to them.** `{:012x}` on a u64
    // pads a short number and lets a long one run to sixteen — so "twelve
    // characters" was a hope rather than a fact until a test asked.
    format!("{:012x}", hash & 0xFFFF_FFFF_FFFF)
}

#[cfg(test)]
mod tests {
    use super::short;

    #[test]
    fn the_same_settings_hash_the_same() {
        assert_eq!(short("min_reach = 1.0"), short("min_reach = 1.0"));
    }

    /// **A changed threshold has to change the version.** That is the whole
    /// job: a row saying `38%` under settings nobody can name is not a result.
    #[test]
    fn a_changed_threshold_changes_it() {
        assert_ne!(short("min_reach = 1.0"), short("min_reach = 1.2"));
    }

    /// One character, and it must still differ. A hash that collides on a
    /// near-miss would put two different rule sets under one name.
    #[test]
    fn even_one_character_changes_it() {
        assert_ne!(short("0.55"), short("0.56"));
    }

    #[test]
    fn it_is_always_the_same_length() {
        assert_eq!(short("").len(), 12);
        assert_eq!(short("a very much longer set of settings indeed").len(), 12);
    }
}
