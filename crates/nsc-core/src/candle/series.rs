//! What the feed hands back.

use super::Bar;
use serde::Deserialize;

/// What Twelve Data sends back for a time series request.
///
/// Only the part we need. Serde skips the `meta` block and anything else they
/// add later, so a new field on their side cannot break this.
#[derive(Debug, Deserialize)]
pub struct Series {
    pub values: Vec<Bar>,
}
