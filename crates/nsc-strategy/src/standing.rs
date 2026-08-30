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
/// ```
///
/// **Two variants, not a number.** A confidence score would let them blur into
/// each other, and they are two different statements rather than two strengths
/// of one: the first has happened and the second has not, quite.
///
/// **THERE IS NO TIER FOR A SHAPE AWAY FROM EVERY ZONE.** A third one existed
/// for a day — 2x a normal candle with no level under it, about four messages
/// a day — and he took it out on 30 August. A shape with no level under it is
/// not a setup, his own `nsc-bull` and `nsc-bear` measured without levels came
/// back at 38%, and four a day of those is four a day of nothing.
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

}

impl Standing {
    /// The band it printed at. **There is always one** — a shape away from
    /// every zone is not a signal at all.
    pub fn band(self) -> Band {
        match self {
            Standing::Inside { band, .. } | Standing::Close { band, .. } => band,
        }
    }

    /// Where it sits against that band.
    pub fn placing(self) -> Placing {
        match self {
            Standing::Inside { .. } => Placing::Inside,
            Standing::Close { placing, .. } => placing,
        }
    }

    /// Did the candle close outside the band? Never true without one.
    pub fn broke_out(self) -> bool {
        matches!(self, Standing::Inside { broke_out: true, .. })
    }

    /// **Is this one he should act on?**
    ///
    /// Only the first. `Close` is worth knowing and is not the same statement:
    /// it has not happened yet.
    pub fn solid(self) -> bool {
        matches!(self, Standing::Inside { .. })
    }

    /// What the card is called.
    pub fn label(self) -> &'static str {
        match self {
            Standing::Inside { .. } => "in the zone",
            Standing::Close { .. } => "extremely close",
        }
    }

    /// Which colour the card leads with.
    ///
    /// **Red is his, chosen on 29 August**, and it is kept for the tier that
    /// asks for action. The near miss must not wear it, or the strongest thing
    /// the bot says stops looking any different from the rest.
    pub fn colour(self) -> &'static str {
        match self {
            Standing::Inside { .. } => "red",
            Standing::Close { .. } => "amber",
        }
    }
}
