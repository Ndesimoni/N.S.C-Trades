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
//! ## "At it" is wider than "inside it"
//!
//! **A band is a zone, not a wall.** Price at 4133 when the top is 4132.57 is
//! at the level in every way that matters, and waiting for it to cross by half
//! a dollar is arbitrary.
//!
//! And an alert that only fires once price is strictly inside arrives too late
//! to be any use. Told it is *coming*, he can get to his chart before the
//! reaction starts.
//!
//! How close counts is `approach` in `config/levels.toml` — a share of the
//! band's own thickness, so it travels between instruments the same way the
//! band does.

use rust_decimal::Decimal;

use super::Band;

/// How far outside the zone price must get before that band can fire again.
///
/// A share of the band's own thickness.
///
/// **Without it, price sitting on the edge flickers.** Three crossings of one
/// boundary would be three alerts, all describing one moment where nothing
/// happened.
const CLEAR_BY: Decimal = Decimal::from_parts(10, 0, 0, false, 2); // 0.10

/// Watches a set of bands, and says when price has just arrived at one.
pub struct Watch {
    /// Each band, and whether price is at it now.
    seen: Vec<(Band, bool)>,

    /// How close counts as arriving, as a share of the band's thickness.
    approach: Decimal,

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
    pub fn over(bands: Vec<Band>, approach: Decimal) -> Self {
        Watch {
            seen: bands.into_iter().map(|band| (band, false)).collect(),
            approach,
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
            let near = nearness(band, price, self.approach);
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
            } else if !now_at_it && *at_it && clear_of(band, price, self.approach) {
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
pub fn nearness(band: &Band, price: Decimal, approach: Decimal) -> Nearness {
    if band.holds(price) {
        return Nearness::Inside;
    }

    let reach = band.thickness() * approach;

    if price <= band.top + reach && price >= band.bottom - reach {
        Nearness::Approaching
    } else {
        Nearness::Away
    }
}

/// Is price properly away from this band, rather than hovering at the edge of
/// the zone?
fn clear_of(band: &Band, price: Decimal, approach: Decimal) -> bool {
    let reach = band.thickness() * (approach + CLEAR_BY);

    price > band.top + reach || price < band.bottom - reach
}
