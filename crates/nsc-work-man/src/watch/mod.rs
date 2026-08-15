//! Watch price against the levels he drew, and say what happens at them.
//!
//! **Rungs 1 and 2 of the ladder.**
//!
//!     price reaches a zone        an alert card, once per visit
//!     a candle at one finishes    a card per candle that touched it
//!
//! Silence is the normal state. Prices arrive about once a second and almost
//! all of them are nowhere near anything.
//!
//! **It watches nothing on a Monday.** Not "stays quiet" — nothing checked,
//! nothing fetched, no queue building up to be dumped on him on Tuesday. When
//! Tuesday opens it says what it FOUND rather than pretending it just
//! happened.
//!
//! This is what `cargo run -p nsc-work-man` does. It lives in the library
//! rather than in a binary so the binary can stay four lines, and so nothing
//! here needs a second copy to be looked at.

mod bands;
mod closes;
mod prices;
mod pulse;
mod resumed;
mod run;
mod say;
mod trouble;

use nsc_core::levels::{Pair, Watch};

pub use run::run;
pub use trouble::dying;

/// Reachable from the tests, and nowhere else.
///
/// The scrubbing itself is not worth making public — but a secret leaking is
/// worth a test, and a test cannot check what it cannot call.
#[cfg(test)]
pub(crate) fn scrub_for_tests(what: &str) -> String {
    trouble::scrub(what)
}

const PAIRS: &str = "config/pairs";
const THICKNESS: &str = "config/levels.toml";
const CALENDAR: &str = "config/when.toml";

/// Where his own working goes. Alerts are not signals.
pub(crate) const OWNER: i64 = 6089491075;

/// Where cards are drawn, so the design can be opened in a browser.
pub(crate) const PREVIEW: &str = "preview";

/// How long to wait between requests.
///
/// **The limit is 8 a minute.** Seven and a half seconds is eight a minute
/// exactly, and both the startup sizing and the candle-close checks go through
/// it.
pub(crate) const BREATHE: std::time::Duration = std::time::Duration::from_millis(7_500);

/// Everything being watched for one pair.
pub(crate) struct Watching {
    pub pair: Pair,
    pub watch: Watch,
}
