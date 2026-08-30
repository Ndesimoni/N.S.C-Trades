//! The three tiers — how much a signal is worth, and why.
//!
//! **Settled with him on 29 August 2026**, and they are three different
//! statements rather than three strengths of one.

use nsc_core::levels::Band;

use super::place::Placing;

/// Where a shape printed, and therefore what it is worth saying about.
///
/// ```text
///     Inside   in the zone            SOLID -- act
///     Close    within half a band     it almost touched and did not
///     Bold     no zone near it        big enough to say so anyway
/// ```
///
/// **Three variants, not a number.** A confidence score would let the three
/// blur into each other, and they are not the same kind of thing: two of them
/// are about a level he drew and one of them is about nothing but the candle.
#[derive(Debug, Clone, Copy)]
pub enum Standing {
    /// **In the zone.** The strongest thing this bot says.
    Inside {
        band: Band,

        /// Did the shape's own candle CLOSE outside the band it printed in?
        ///
        /// Reported, never required. A tail through the band is the level
        /// being tested; a close outside it is the level being left.
        broke_out: bool,
    },

    /// **Within half a band of the zone.** It almost kissed it and did not.
    ///
    /// `placing` says which side — above or below. Half a band is his own
    /// answer from 25 August, and it is a share of that band's own thickness
    /// rather than a distance, so it travels: about 4 pips on the Aussie
    /// 4-hour and about 13 on cable's weekly.
    Close { band: Band, placing: Placing },

    /// **No zone near it at all, and the shape is bold enough to say so.**
    ///
    /// His words on 29 August: *"we see a really bold move, but it's not in
    /// our zone... I think we should also get an alert for that."*
    ///
    /// **This one is not a setup and the card must never let it read as one.**
    /// There is no level under it, and his own measurement of `nsc-bull` and
    /// `nsc-bear` without levels came back at 38% — worse than a coin flip.
    Bold,
}

impl Standing {
    /// The band it printed at, when there is one.
    ///
    /// **`Bold` has none, and that is the whole point of it.**
    pub fn band(self) -> Option<Band> {
        match self {
            Standing::Inside { band, .. } | Standing::Close { band, .. } => Some(band),
            Standing::Bold => None,
        }
    }

    /// Where it sits against that band.
    pub fn placing(self) -> Option<Placing> {
        match self {
            Standing::Inside { .. } => Some(Placing::Inside),
            Standing::Close { placing, .. } => Some(placing),
            Standing::Bold => None,
        }
    }

    /// Did the candle close outside the band? Never true without one.
    pub fn broke_out(self) -> bool {
        matches!(self, Standing::Inside { broke_out: true, .. })
    }

    /// **Is this one he should act on?**
    ///
    /// Only the first. The other two are worth knowing and are not the same
    /// statement — `Close` has not happened yet and `Bold` has no level under
    /// it at all.
    pub fn solid(self) -> bool {
        matches!(self, Standing::Inside { .. })
    }

    /// What the card is called.
    pub fn label(self) -> &'static str {
        match self {
            Standing::Inside { .. } => "in the zone",
            Standing::Close { .. } => "extremely close",
            Standing::Bold => "no zone near it",
        }
    }

    /// Which colour the card leads with.
    ///
    /// **Red is his, chosen on 29 August**, and it is kept for the one tier
    /// that asks for action. The other two must not wear it or the strongest
    /// thing the bot says stops looking different from the rest.
    pub fn colour(self) -> &'static str {
        match self {
            Standing::Inside { .. } => "red",
            Standing::Close { .. } => "amber",
            Standing::Bold => "plain",
        }
    }
}
