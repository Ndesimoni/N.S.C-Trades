//! Where a level came from.

use serde::{Deserialize, Serialize};

/// Who decided this price was a level.
///
/// The two are not interchangeable, and the bot has to be able to tell them
/// apart. The trader's own levels are what gets traded. The found ones exist
/// so the finder can be scored against his — never to trade on their own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Origin {
    /// Worked out by `nsc-ta` from swing points clustering at one price.
    ///
    /// Carries a touch count and the dates of the first and last touch,
    /// because those are what made it a level.
    Found,

    /// Drawn by hand and read from `config/levels/`.
    ///
    /// Has **no touch count**, and asking for one gives `None` rather than a
    /// made-up number. A hand-drawn level is not there because price turned
    /// five times — it is there because a big move ended on it, which is a
    /// different reason and cannot be counted.
    ///
    /// Faking a touch count would quietly poison every later comparison
    /// between his levels and the finder's.
    DrawnByHand,
}

impl Origin {
    pub fn is_drawn_by_hand(self) -> bool {
        matches!(self, Origin::DrawnByHand)
    }
}
