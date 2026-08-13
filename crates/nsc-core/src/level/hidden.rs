//! Why a level does not get a line on the chart.

use serde::{Deserialize, Serialize};

use crate::timeframe::Timeframe;

/// The reason a level is not drawn.
///
/// Both reasons mean the same thing on screen — no line. They are kept apart
/// because the bot has to be able to say *why* a level you expected is
/// missing, and "a weekly is already there" is a different answer from "your
/// own timeframe already has one nearby".
///
/// Neither is a deletion. The level is still found, still counted, and the
/// engine still knows the price matters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotDrawn {
    /// A bigger timeframe's band already covers this price.
    ///
    /// You look at the weekly line and you already know the price matters. A
    /// second, thinner line on the same spot says nothing new.
    CoveredBy(Timeframe),

    /// A level on the **same** timeframe, with more touches, sits too near.
    ///
    /// This is the consolidation case. Price chops around one area for two
    /// years and turns a dozen times, so the finder sees a level at every
    /// turn. You look at all of it and draw one line saying "price did
    /// something here".
    ///
    /// The most-touched one keeps the line. The rest are crowded out.
    CrowdedOut,
}

impl NotDrawn {
    /// The bigger timeframe sitting over this price, if that is the reason.
    pub fn covering_timeframe(self) -> Option<Timeframe> {
        match self {
            NotDrawn::CoveredBy(timeframe) => Some(timeframe),
            NotDrawn::CrowdedOut => None,
        }
    }
}
