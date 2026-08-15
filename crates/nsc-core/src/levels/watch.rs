//! Watching bands for price arriving.
//!
//! Prices come down the websocket about once a second and barely move —
//! 4375.35, 4375.36, 4375.35. **A touch has to fire once, not once per
//! price**, or one visit to a level becomes twenty alerts and he stops
//! reading them.
//!
//! So this holds one fact per band: is price inside it *now*? An alert is the
//! moment that turns from no to yes, and nothing else.

use rust_decimal::Decimal;

use super::Band;

/// How far outside a band price must get before that band can fire again.
///
/// A share of the band's own thickness, so it travels between instruments the
/// same way the band does.
///
/// **Without it, price sitting on the edge flickers.** 4131.99, 4132.01,
/// 4131.99 against a top of 4132.00 is three crossings and would be three
/// alerts, all describing one moment where nothing happened.
const CLEAR_BY: Decimal = Decimal::from_parts(10, 0, 0, false, 2); // 0.10

/// Watches a set of bands, and says when price has just arrived in one.
pub struct Watch {
    /// Each band, and whether price is inside it now.
    seen: Vec<(Band, bool)>,

    /// Whether any price has arrived yet.
    ///
    /// The first one only says where price *is*. It cannot say price has
    /// *arrived* — it may have been sitting there for hours before the bot
    /// started, and an alert for that is a lie about when it happened.
    started: bool,
}

impl Watch {
    pub fn over(bands: Vec<Band>) -> Self {
        Watch {
            seen: bands.into_iter().map(|band| (band, false)).collect(),
            started: false,
        }
    }

    /// Feeds one price, and gives back every band price has **just entered**.
    ///
    /// Empty almost always. That is the point.
    pub fn arrive(&mut self, price: Decimal) -> Vec<Band> {
        let first = !self.started;
        self.started = true;

        let mut entered = Vec::new();

        for (band, inside) in &mut self.seen {
            let holds = band.holds(price);

            if first {
                // Only note where price is. Arriving is a change, and there is
                // nothing yet to have changed from.
                *inside = holds;
                continue;
            }

            if holds && !*inside {
                *inside = true;
                entered.push(*band);
            } else if !holds && *inside && clear_of(band, price) {
                *inside = false;
            }
        }

        entered
    }

    /// Which bands price is sitting in. For a heartbeat, not an alert.
    pub fn resting_in(&self) -> Vec<Band> {
        self.seen
            .iter()
            .filter(|(_, inside)| *inside)
            .map(|(band, _)| *band)
            .collect()
    }
}

/// Is price properly out of this band, rather than hovering on its edge?
fn clear_of(band: &Band, price: Decimal) -> bool {
    let margin = band.thickness() * CLEAR_BY;

    price > band.top + margin || price < band.bottom - margin
}
