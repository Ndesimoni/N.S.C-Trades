//! Which way the market is going.

use serde::{Deserialize, Serialize};

use crate::swing::SwingKind;

/// The direction the market has last proved.
///
/// `Unclear` is a real answer, not a missing one. At the start of a history,
/// and after a range that has broken nothing, there is no trend — and saying
/// so is better than picking a side. The rules can refuse to trade it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Trend {
    Up,
    Down,
    Unclear,
}

impl Trend {
    /// The trend proved by taking out an old extreme.
    ///
    /// An old high taken out means up; an old low taken out means down.
    pub fn from_break(kind: SwingKind) -> Self {
        match kind {
            SwingKind::High => Trend::Up,
            SwingKind::Low => Trend::Down,
        }
    }

    pub fn is_up(self) -> bool {
        matches!(self, Trend::Up)
    }

    pub fn is_down(self) -> bool {
        matches!(self, Trend::Down)
    }

    /// Is there a direction at all?
    ///
    /// **Check this before using a trend for anything.** `Unclear` is not a
    /// weak `Up`.
    pub fn is_clear(self) -> bool {
        !matches!(self, Trend::Unclear)
    }
}
