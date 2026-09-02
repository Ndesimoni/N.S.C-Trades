//! **Noticing he has sent a level**, without being restarted.
//!
//! It watches the folder rather than the clock: how many pair files there are,
//! and the newest time any of them was touched.

use std::path::Path;
use std::time::SystemTime;

use nsc_core::levels::known;

use crate::places::PAIRS;

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
