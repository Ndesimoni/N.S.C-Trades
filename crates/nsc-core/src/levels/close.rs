//! What a finished candle did at one of his zones.
//!
//! **The close is the point.** Price reaching a level says nothing on its own
//! — it may cut straight through. Where the candle *ended* is what says
//! whether the level did anything.
//!
//! Nothing here asks whether the candle has finished. It is handed a finished
//! one, and `Bar::is_finished` is the single place that decides. Reading a
//! candle that is still forming would make every backtest look better than the
//! bot, which is the one mistake in this project that hides.

use rust_decimal::Decimal;

use super::Band;
use crate::candle::Bar;

/// Where a candle that reached the zone ended up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtZone {
    /// It never reached the zone. Silence.
    Missed,

    /// It ended inside. Price is sitting in his level.
    ClosedInside,

    /// It reached in and came back up.
    ClosedAbove,

    /// It reached in and came back down.
    ClosedBelow,
}

impl AtZone {
    /// Did this candle have anything to do with the zone at all?
    pub fn worth_saying(self) -> bool {
        self != AtZone::Missed
    }
}

/// What this candle did at this band.
pub fn what_it_did(band: &Band, bar: &Bar) -> AtZone {
    // TOUCHED means the candle's range and the band overlap at all — A WICK
    // COUNTS. A candle that only wicked in and closed back out is the whole
    // reason to look at closes rather than at price, and treating it as a miss
    // would throw away the rejection he is waiting for.
    let touched = bar.high >= band.bottom && bar.low <= band.top;

    if !touched {
        return AtZone::Missed;
    }

    if band.holds(bar.close) {
        AtZone::ClosedInside
    } else if bar.close > band.top {
        AtZone::ClosedAbove
    } else {
        AtZone::ClosedBelow
    }
}

/// Did this candle **gap** into the zone — open inside when the one before it
/// closed outside?
///
/// Spot forex runs Sunday evening to Friday evening without a break, so a
/// candle's open is normally the last one's close and this is false all week.
/// It is true at the Sunday open, and across gold's hour off each night.
///
/// **That is why there is no "a candle opened in the zone" message.** All week
/// it would repeat what the close said a minute earlier. A gap is the only
/// version of it that carries anything.
pub fn gapped_in(band: &Band, before: &Bar, bar: &Bar) -> bool {
    band.holds(bar.open) && !band.holds(before.close)
}

/// How far into the zone the candle reached, as a share of the zone.
///
/// `1` means it crossed the whole band. `0.05` means it grazed the edge. The
/// card draws this, so nobody has to read the number.
pub fn how_deep(band: &Band, bar: &Bar) -> Decimal {
    let thickness = band.thickness();

    if thickness.is_zero() {
        return Decimal::ZERO;
    }

    let top = bar.high.min(band.top);
    let bottom = bar.low.max(band.bottom);

    ((top - bottom) / thickness).max(Decimal::ZERO)
}
