//! The test: is the shape at the level?

use nsc_core::levels::Band;
use rust_decimal::Decimal;

use super::Rules;

/// Where a shape sits against a band.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placing {
    /// Inside the band. **No depth rule** — inside is inside.
    Inside,

    /// Above its top edge, within reach.
    JustAbove,

    /// Below its bottom edge, within reach.
    JustBelow,

    /// Too far to have anything to do with the level.
    Away,
}

impl Placing {
    /// Does this count as at the level?
    pub fn counts(self) -> bool {
        !matches!(self, Placing::Away)
    }

    /// How the sentence says it.
    pub fn words(self) -> &'static str {
        match self {
            Placing::Inside => "in",
            Placing::JustAbove => "just above",
            Placing::JustBelow => "just below",
            Placing::Away => "away from",
        }
    }
}

/// Where `price` sits against the band.
///
/// `price` is the shape's own touching point — the pin's tail tip, or an
/// engulfing's close. `shape.rs` decides which, and it decides it from what
/// the shape means.
///
/// **Reach is a share of the band's own thickness, never a distance.** A band
/// on gold is about 78 points and on the euro about 0.004. A number in points
/// works on the pair it was set on and quietly stops working on every other —
/// which is the same reason every threshold in this project is a share of
/// something.
///
/// **There is no touch rule.** Asked whether the pin had to touch the band he
/// said it need not, and that touching was no problem either. So distance is
/// the only test, and a pin that pokes inside simply measures as nought.
pub fn where_it_sits(price: Decimal, band: &Band, rules: &Rules) -> Placing {
    if band.holds(price) {
        return Placing::Inside;
    }

    let reach = band.thickness() * rules.reach_of_band;

    if price > band.top {
        return if price - band.top <= reach {
            Placing::JustAbove
        } else {
            Placing::Away
        };
    }

    if band.bottom - price <= reach {
        Placing::JustBelow
    } else {
        Placing::Away
    }
}

/// The band this shape is nearest to, if any of them count.
///
/// **The nearest one wins, not the first in the list.** Two zones close
/// together would otherwise report whichever happened to be read first, and
/// which zone a shape printed at is the whole content of the signal.
pub fn nearest<'a>(
    price: Decimal,
    bands: &'a [Band],
    rules: &Rules,
) -> Option<(&'a Band, Placing)> {
    bands
        .iter()
        .map(|band| (band, where_it_sits(price, band, rules)))
        .filter(|(_, placing)| placing.counts())
        .min_by_key(|(band, _)| (band.price - price).abs())
}
