//! What the retracement says about where price is now.

use rust_decimal::Decimal;

use super::{Leg, Rules};

/// Where price sits in the move.
///
/// **A reading, not a decision.** "Price is in the golden zone" belongs here.
/// "Therefore buy" belongs in `nsc-strategy`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Where {
    /// Barely pulled back at all — shallower than the strong-trend level.
    ///
    /// **Not a place to get in, and that is the point.** A pullback this
    /// shallow means the market barely paused, which is what a powerful move
    /// looks like. The information is about the MOVE, not about an entry.
    BarelyPaused,

    /// On its way down into the zone, past the strong-trend level.
    ComingBack,

    /// **In the golden zone.** The thing to pay attention to.
    GoldenZone,

    /// Through the zone but not yet at the stop level.
    Deeper,

    /// Past where a stop would be looked at. The move is in question.
    PastTheStop,

    /// All the way back through where the move began.
    ///
    /// **The move is not a move any more.** Whatever the retracement was
    /// measuring has been undone.
    Undone,

    /// Beyond the extreme — the move extended rather than pulled back.
    StillGoing,
}

impl Where {
    /// What to call it to a person.
    pub fn spoken(self) -> &'static str {
        match self {
            Self::BarelyPaused => "barely paused",
            Self::ComingBack => "coming back",
            Self::GoldenZone => "in the golden zone",
            Self::Deeper => "deeper than the zone",
            Self::PastTheStop => "past the stop level",
            Self::Undone => "undone",
            Self::StillGoing => "still going",
        }
    }
}

/// Where price sits in this move.
///
/// **Depth is a share, so it works the same on gold and the euro.** Nought at
/// the extreme, one back at the start.
pub fn read(leg: Leg, price: Decimal, rules: &Rules) -> Where {
    let deep = leg.how_deep(price);
    let (low, high) = rules.zone;

    if deep < Decimal::ZERO {
        return Where::StillGoing;
    }

    if deep < rules.strong_trend {
        return Where::BarelyPaused;
    }

    if deep < low {
        return Where::ComingBack;
    }

    if deep <= high {
        return Where::GoldenZone;
    }

    if deep <= rules.stop_level {
        return Where::Deeper;
    }

    if deep <= Decimal::ONE {
        return Where::PastTheStop;
    }

    Where::Undone
}

/// Every level this move draws, in the order they sit on the chart.
///
/// **Four, and only four.** A level with no job attached is a line the bot
/// draws and nothing reads.
pub fn levels(leg: Leg, rules: &Rules) -> Vec<(Decimal, Decimal)> {
    let (low, high) = rules.zone;

    let mut shares = vec![rules.strong_trend, low, high, rules.stop_level];
    shares.sort();
    shares.dedup();

    shares
        .into_iter()
        .map(|share| (share, leg.retracement(share)))
        .collect()
}

/// Where the targets sit, beyond the extreme.
pub fn targets(leg: Leg, rules: &Rules) -> Vec<(Decimal, Decimal)> {
    rules
        .extensions
        .iter()
        .map(|ratio| (*ratio, leg.extension(*ratio)))
        .collect()
}
