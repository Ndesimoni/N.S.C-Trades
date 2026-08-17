//! Where he is in a flow.

use nsc_core::levels::Timeframe;

/// Where he is in the flow.
///
/// It stays put once set, so a run of six weekly levels is two taps and six
/// numbers — the pair and the timeframe are never typed twice.
#[derive(Default)]
pub struct Adding {
    /// Reachable from `one`, which hands him into this flow when he taps
    /// "add levels" on a pair's page.
    pub(in crate::inbox) pair: Option<String>,
    pub(in crate::inbox) timeframe: Option<Timeframe>,
    pub(super) naming: bool,
    /// What the last message added, so Undo knows how much to take back off.
    pub(super) just_added: Option<(String, usize)>,

    /// He has sent /remove and is picking which pair.
    ///
    /// Reachable from `stopping`, which owns that whole conversation.
    pub(in crate::inbox) removing: bool,

    /// The pair he has asked to stop watching, waiting on a second tap.
    pub(in crate::inbox) stopping: Option<String>,

    /// He has sent /restore and is picking which set to bring back.
    pub(in crate::inbox) restoring: bool,

    /// He has sent /pairs and is picking which one to look at.
    pub(in crate::inbox) browsing: bool,

    /// The pair whose page he is on, from /pairs.
    pub(in crate::inbox) chosen: Option<String>,

    /// He is picking which level to take off that pair.
    pub(in crate::inbox) dropping: bool,

    /// He has sent /chart and is picking which pair to look at.
    pub(in crate::inbox) charting: bool,

    /// The pair he wants a chart of, waiting on him to pick which chart.
    ///
    /// **This is what tells the three timeframe buttons apart.** `Weekly` means
    /// "add weekly levels" in one flow and "draw the weekly chart" in this one,
    /// and the button sends the same word either way. Set, it is a chart he is
    /// asking for; clear, it is a level he is adding.
    pub(in crate::inbox) chart_of: Option<String>,
}
