//! What price did next, and whether that means anything.

use nsc_core::candle::Bar;
use rust_decimal::Decimal;

/// How far ahead to look, in candles.
pub(super) const AHEAD: [usize; 4] = [1, 3, 5, 10];

/// One pattern's record at one horizon.
#[derive(Default, Clone, Copy)]
pub(super) struct Record {
    pub(super) went_its_way: usize,
    pub(super) tried: usize,

    /// Every move added up, in ATR, signed the way the pattern claimed.
    ///
    /// **A win rate on its own can lie in both directions.** Nine small wins
    /// and one enormous loss is a 90% pattern that loses money; the reverse is
    /// a 30% pattern worth trading. This is the other half of the answer.
    pub(super) moved: Decimal,
}

impl Record {
    pub(super) fn rate(&self) -> f64 {
        if self.tried == 0 {
            return 0.0;
        }

        self.went_its_way as f64 * 100.0 / self.tried as f64
    }

    /// The average move, in ATR, in the direction the pattern claimed.
    pub(super) fn edge(&self) -> Decimal {
        if self.tried == 0 {
            return Decimal::ZERO;
        }

        (self.moved / Decimal::from(self.tried)).round_dp(3)
    }
}

/// Did price close further along than it started, `ahead` candles later?
///
/// **Closes, not highs and lows.** A high that was touched and given straight
/// back is not a move he could have taken, and counting it is the surest way
/// to build a pattern that works beautifully and cannot be traded.
pub(super) fn played_out(
    bars: &[Bar],
    at: usize,
    ahead: usize,
    up: bool,
    normal: Decimal,
) -> Option<(bool, Decimal)> {
    let later = bars.get(at + ahead)?;
    let moved = later.close - bars[at].close;

    if normal <= Decimal::ZERO {
        return None;
    }

    // Signed the way the pattern claimed, so a bearish pattern that fell is a
    // positive number and the two can be added together.
    let its_way = if up { moved } else { -moved };

    Some((its_way > Decimal::ZERO, its_way / normal))
}
