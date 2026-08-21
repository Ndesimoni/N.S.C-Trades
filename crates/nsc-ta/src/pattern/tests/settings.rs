//! The thresholds the tests judge against.

use std::path::Path;

use crate::pattern::Rules;

/// **The ones he actually runs with**, read from the same file the bot reads.
///
/// Not a set made up for the tests. A threshold that only exists in a test is
/// a threshold nobody has to live with — and the first thing it does is pass.
pub(super) fn rules() -> Rules {
    crate::pattern::load(Path::new("../../config/patterns.toml"))
        .expect("config/patterns.toml should read")
}
