//! Something the structure of the chart did.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::attempts::FailedAttempt;
use super::breaks::StructureBreak;
use crate::swing::SwingKind;

/// One thing that happened at an old extreme.
///
/// Both outcomes are reported, not just the good one. A market that tried to
/// take a high and could not is telling you something, and those rows are the
/// "do not take this" examples nothing else in the system collects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructureEvent {
    /// Price crossed an old extreme and carried far enough past it.
    Taken(StructureBreak),

    /// Price crossed an old extreme, ran out of steam, and came back under it.
    Failed(FailedAttempt),
}

impl StructureEvent {
    /// Which kind of extreme this happened at.
    pub fn kind(self) -> SwingKind {
        match self {
            StructureEvent::Taken(broken) => broken.kind(),
            StructureEvent::Failed(attempt) => attempt.kind(),
        }
    }

    /// When it finished.
    pub fn at(self) -> DateTime<Utc> {
        match self {
            StructureEvent::Taken(broken) => broken.at(),
            StructureEvent::Failed(attempt) => attempt.to(),
        }
    }

    /// How far past the old extreme price got, as a share of the run behind
    /// it. Past the threshold for a break, short of it for a failure.
    pub fn share_of_run(self) -> Option<f64> {
        match self {
            StructureEvent::Taken(broken) => broken.share_of_run(),
            StructureEvent::Failed(attempt) => attempt.share_of_run(),
        }
    }

    /// Did the market actually go somewhere?
    ///
    /// **Check this before letting an event change the trend.** A failed
    /// attempt is evidence, not a direction.
    pub fn is_taken(self) -> bool {
        matches!(self, StructureEvent::Taken(_))
    }
}
