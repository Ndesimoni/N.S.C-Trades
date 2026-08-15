//! Watching bands for price arriving.
//!
//! Prices come down the websocket about once a second and barely move —
//! 4375.35, 4375.36, 4375.35. **A touch has to fire once, not once per
//! price**, or one visit to a level becomes twenty alerts and he stops
//! reading them.
//!
//! So this holds one fact per band: is price at it *now*? An alert is the
//! moment that turns from no to yes, and nothing else.
//!
//! ## Arriving and leaving are measured differently, on purpose
//!
//! **Arriving is a touch.** Price a pip outside the band has reached it, and
//! staying quiet over a cent would be silly. That is all `reach` is for — it
//! is not there to buy him time, because the band already does that. The outer
//! edge of his gold weekly zone is about three hours of movement from the line
//! he drew, and on the pound about six.
//!
//! **Leaving has to be a real distance**, or price sitting on the edge fires
//! over and over: a pip out, a pip back, all afternoon. So a band goes quiet
//! only once price is properly gone — [`CLEAR_BY`] of its own thickness, which
//! is about 8 points on gold and 6 pips on the pound.
//!
//! Easy to trigger, hard to reset.

use rust_decimal::Decimal;

use super::Band;

/// How far outside price must get before that band can fire again.
///
/// A share of the band's own thickness, so it is a real distance on every pair
/// — about 8 points on gold, about 6 pips on the pound.
///
/// **Without it, price sitting on the edge flickers.** Three crossings of one
/// boundary would be three alerts, all describing one moment where nothing
/// happened.
const CLEAR_BY: Decimal = Decimal::from_parts(10, 0, 0, false, 2); // 0.10

/// Watches a set of bands, and says when price has just arrived at one.
pub struct Watch {
    /// Each band, and whether price is at it now.
    seen: Vec<(Band, bool)>,

    /// How close counts as arriving — **a price, not a share**. A pip on this
    /// pair, worked out by [`Pair::reach`](super::Pair::reach).
    reach: Decimal,

    /// Whether any price has arrived yet.
    ///
    /// The first one only says where price *is*. It cannot say price has
    /// *arrived* — it may have been sitting there for hours before the bot
    /// started, and an alert for that is a lie about when it happened.
    started: bool,
}

/// How near price is to a band, and which side.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nearness {
    /// Inside the band he drew.
    Inside,
    /// Not inside, but near enough to count as being at it.
    Approaching,
    /// Nowhere near.
    Away,
}

impl Watch {
    pub fn over(bands: Vec<Band>, reach: Decimal) -> Self {
        Watch {
            seen: bands.into_iter().map(|band| (band, false)).collect(),
            reach,
            started: false,
        }
    }

    /// Feeds one price, and gives back every band price has **just arrived
    /// at**, with how near it got.
    ///
    /// Empty almost always. That is the point.
    pub fn arrive(&mut self, price: Decimal) -> Vec<(Band, Nearness)> {
        let first = !self.started;
        self.started = true;

        let mut arrived = Vec::new();

        for (band, at_it) in &mut self.seen {
            let near = nearness(band, price, self.reach);
            let now_at_it = near != Nearness::Away;

            if first {
                // Only note where price is. Arriving is a change, and there is
                // nothing yet to have changed from.
                *at_it = now_at_it;
                continue;
            }

            if now_at_it && !*at_it {
                *at_it = true;
                arrived.push((*band, near));
            } else if !now_at_it && *at_it && clear_of(band, price) {
                *at_it = false;
            }
        }

        arrived
    }

    /// Which bands price is sitting at. For a heartbeat, not an alert.
    pub fn resting_at(&self) -> Vec<Band> {
        self.seen
            .iter()
            .filter(|(_, at_it)| *at_it)
            .map(|(band, _)| *band)
            .collect()
    }
}

/// How near this price is to this band.
///
/// `reach` is a **price**, not a share — a pip on the pair being watched.
pub fn nearness(band: &Band, price: Decimal, reach: Decimal) -> Nearness {
    if band.holds(price) {
        return Nearness::Inside;
    }

    if price <= band.top + reach && price >= band.bottom - reach {
        Nearness::Approaching
    } else {
        Nearness::Away
    }
}

/// Is price properly away from this band, rather than hovering at its edge?
///
/// **Deliberately not the same sum as arriving.** Arriving is a touch, so a pip
/// is right. Leaving has to be a real distance or one visit becomes an
/// afternoon of alerts, so it is measured against the band itself.
fn clear_of(band: &Band, price: Decimal) -> bool {
    let gone = band.thickness() * CLEAR_BY;

    price > band.top + gone || price < band.bottom - gone
}
