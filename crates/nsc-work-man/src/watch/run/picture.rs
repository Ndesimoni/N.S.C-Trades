//! The live picture, for anything that asks rather than watches.

use std::collections::HashMap;

use chrono::Utc;
use nsc_core::when::{self, Allowed, Rules};

use crate::watch::Watching;
use crate::watch::standing::{Snapshot, Standing};

/// The live picture, for anything that asks rather than watches.
pub(crate) fn snapshot(watching: &HashMap<String, Watching>, calendar: &Rules) -> Snapshot {
    // Sorted, so the list does not shuffle between askings. A HashMap hands
    // them back in a different order every time, and a list that reorders
    // itself looks like something changed when nothing did.
    let mut seen: Vec<&Watching> = watching.values().collect();
    seen.sort_by(|a, b| a.pair.symbol.cmp(&b.pair.symbol));

    Snapshot {
        pairs: seen
            .iter()
            .map(|one| Standing {
                pair: one.pair.clone(),
                bands: one.watch.bands(),
                price: one.watch.last_price(),
            })
            .collect(),
        opened: when::opened(Utc::now(), calendar),
        quiet: when::allowed(Utc::now(), calendar) == Allowed::Silence,
    }
}
