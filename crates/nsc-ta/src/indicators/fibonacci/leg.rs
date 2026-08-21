//! The move being measured.

use rust_decimal::Decimal;

/// A move worth measuring a retracement over.
///
/// **It holds the move, not the levels.** The prices are only ever a share of
/// it, and storing them instead would lose the one thing worth arguing about.
///
/// When a Fibonacci reading looks wrong, the move it picked is nearly always
/// the disagreement — and an argument about a move is one you can settle by
/// looking at a chart.
///
/// **It does not know where it came from**, and that is deliberate. Swings
/// will anchor it one day; his own drawn levels can anchor it today. Neither
/// belongs in here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Leg {
    from: Decimal,
    to: Decimal,
}

impl Leg {
    /// A move from one price to another.
    ///
    /// **`None` when there is no move.** Two identical prices describe nothing,
    /// and every share of nothing is nothing — dividing by it would make every
    /// answer meaningless without saying so.
    pub fn new(from: Decimal, to: Decimal) -> Option<Leg> {
        (from != to).then_some(Leg { from, to })
    }

    /// Where the move started.
    pub fn from(self) -> Decimal {
        self.from
    }

    /// Where it got to — the extreme a retracement comes back from.
    pub fn to(self) -> Decimal {
        self.to
    }

    /// How far it travelled, signed.
    pub fn run(self) -> Decimal {
        self.to - self.from
    }

    /// Did it go up?
    pub fn up(self) -> bool {
        self.to > self.from
    }

    /// The price at a given retracement — **measured back from the extreme**.
    ///
    /// `0.618` of a move up from 100 to 200 is 138.2, not 161.8. A retracement
    /// is how much was GIVEN BACK, so it counts from where the move ended.
    ///
    /// One formula serves both directions, because `run` carries the sign: a
    /// move down from 200 to 100 gives 161.8 for the same share.
    pub fn retracement(self, share: Decimal) -> Decimal {
        self.to - self.run() * share
    }

    /// The price at a given extension — **beyond the extreme**, for targets.
    ///
    /// `1.272` of that same move up is 227.2. Note this is NOT `retracement`
    /// with a number over one: that would run off the far end, below where the
    /// move began, which is a different place entirely and not a target.
    pub fn extension(self, ratio: Decimal) -> Decimal {
        self.from + self.run() * ratio
    }

    /// How deep price has come back, as a share of the move.
    ///
    /// Nought at the extreme, one back at the start. Over one means price has
    /// gone through where the move began — the move is not a move any more.
    pub fn how_deep(self, price: Decimal) -> Decimal {
        (self.to - price) / self.run()
    }
}
