//! Which way round a swing is.

use serde::{Deserialize, Serialize};

/// A peak or a trough.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SwingKind {
    /// A peak. Price came up to it and turned back down.
    High,
    /// A trough. Price came down to it and turned back up.
    Low,
}

impl SwingKind {
    pub fn is_high(self) -> bool {
        matches!(self, SwingKind::High)
    }

    pub fn is_low(self) -> bool {
        matches!(self, SwingKind::Low)
    }

    /// The other one. A swing high's opposite is a swing low.
    ///
    /// Used when walking a chart looking for alternating peaks and troughs —
    /// after a high you want the next low, not the next high.
    pub fn opposite(self) -> SwingKind {
        match self {
            SwingKind::High => SwingKind::Low,
            SwingKind::Low => SwingKind::High,
        }
    }
}
