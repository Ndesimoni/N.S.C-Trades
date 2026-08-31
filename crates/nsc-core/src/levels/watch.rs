//! Watching bands for price arriving.
//!
//! Prices come down the websocket about once a second and barely move —
//! 4375.35, 4375.36, 4375.35. **A touch has to fire once, not once per
//! price**, or one visit to a level becomes twenty alerts and he stops
//! reading them.
//!
//! So this holds one fact per band: **how deep price has got this visit.** An
//! alert is the moment that gets deeper, and nothing else.
//!
//! ## Deeper, not different
//!
//! Away, then approaching, then inside. Each step down speaks once:
//!
//! ```text
//!   4,132.97   price arrives near the zone   ->  "approaching"
//!   4,132.57   it enters the zone            ->  "in the zone"
//!   4,120.00   it goes further in            ->  nothing
//!   4,133.00   it drifts back to the edge    ->  nothing
//!   4,120.00   and back in again             ->  nothing
//! ```
//!
//! **Entering is the thing he actually wanted to know**, and it used to say
//! nothing at all: the band was marked "at it" the moment price came near, so
//! walking in was not a change. He heard "coming up on your zone" and then had
//! to wait for a candle, which on the hourly is up to an hour.
//!
//! Wobbling at the edge still says nothing, because it never gets deeper than
//! it already was.
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
//! only once price is properly gone — [`CLEAR_BY`] of its own thickness beyond
//! where *approaching* ends.
//!
//! **Beyond approaching, and that word is the fix.** Measured from the band
//! instead, the reset line landed INSIDE the approaching zone on his own
//! levels, so a two-pip wobble re-armed the alert and fired it again. He got
//! the cards to prove it — see [`clear_of`].
//!
//! Easy to trigger, hard to reset.

use rust_decimal::Decimal;

use super::{AtZone, Band};
use crate::candle::Bar;

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
    /// Each band, and **the deepest price has got this visit**.
    ///
    /// Not "where price is" — where it has BEEN, since it last left properly.
    /// That is what makes wobbling at the edge silent while walking further in
    /// still speaks.
    seen: Vec<Level>,

    /// How close counts as arriving, **as a share of each band's own
    /// thickness**, from [`Pair::reach_share`](super::Pair::reach_share).
    ///
    /// **A share, so every band gets a reach its own size.** It was one price
    /// for the whole pair until 31 August 2026 — four pips, which is 22% of an
    /// AUD/USD daily band and 0.03% of a gold weekly one.
    share: Decimal,

    /// The last price seen, so a resumed session can say where things stand
    /// without waiting for the next one.
    last: Option<Decimal>,

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
    pub fn over(bands: Vec<Band>, share: Decimal) -> Self {
        Watch {
            seen: bands
                .into_iter()
                .map(|band| Level {
                    band,
                    deepest: Nearness::Away,
                    spoken: false,
                    closes: Vec::new(),
                })
                .collect(),
            share,
            last: None,
            started: false,
        }
    }

    /// Feeds in a price, and gives back every band that has **just arrived at
    /// something worth saying**. Empty almost always, and that is the point.
    ///
    /// **A level speaks once and then keeps quiet.** Approaching is news only
    /// while the level has never said anything at all. After
    /// that only a candle closing somewhere new is worth a card, and that
    /// comes through [`Watch::closed`] rather than here.
    pub fn arrive(&mut self, price: Decimal) -> Vec<(Band, Nearness)> {
        let first = !self.started;
        self.started = true;
        self.last = Some(price);

        let mut arrived = Vec::new();

        for level in &mut self.seen {
            // **Each band's own reach**, from its own thickness.
            let reach = level.band.thickness() * self.share;
            let near = nearness(&level.band, price, reach);

            if first {
                // Only note where price is. Arriving is a change, and there is
                // nothing yet to have changed from.
                level.deepest = near;
                continue;
            }

            // Properly gone. The wording starts fresh for the next visit —
            // **but what the level has SAID is not forgotten.** Price leaving
            // and coming back is exactly the case he asked to go quiet.
            if level.deepest != Nearness::Away && clear_of(&level.band, price, reach) {
                level.deepest = Nearness::Away;
                continue;
            }

            if depth(near) <= depth(level.deepest) {
                continue;
            }

            level.deepest = near;

            // **The one gate, and it is the whole change of 31 August 2026.**
            // A level that has already spoken says nothing about price merely
            // being near it again.
            if !level.spoken {
                level.spoken = true;
                arrived.push((level.band, near));
            }
        }

        arrived
    }

    /// A candle closed at this level. **Is it worth a card?**
    ///
    /// Yes when it **broke through in the direction price was travelling**, or
    /// settled inside — and only when that is a different ending from the last
    /// one this timeframe reported.
    ///
    /// ## A rejection is silent, and that is his call
    ///
    /// 31 August 2026: *"if price is coming from below the line we want the
    /// notification if it closes above, and if from above we want it if it
    /// closes below."* Asked whether he wanted the other way round — price
    /// thrown back where it came from — he said: *"I do not want a
    /// notification on it."*
    ///
    /// **The rejection is not lost.** It reaches him as a SETUP if a shape he
    /// trades printed there, which is rung 3 and runs on its own: *"if we did
    /// not break it but a candlestick pattern was formed close to the zone we
    /// need the alert for pattern form."*
    ///
    /// **Any close ends the approach card**, break or not, because "price is
    /// near this line" stops being news the moment the line has a story.
    ///
    /// `on` is the timeframe's stored spelling — `4h`, `1d`. `from` is which
    /// side the candle came from, out of [`came_from`] — **the candle's own
    /// open, never where the ticker happens to be now.**
    pub fn closed(&mut self, band: &Band, on: &str, did: AtZone, from: Option<Side>) -> bool {
        let Some(level) = self
            .seen
            .iter_mut()
            .find(|one| one.band.price == band.price)
        else {
            return false;
        };

        // Whatever happens next, price being near this line is no longer news.
        level.spoken = true;

        if !worth_a_card(from, did) {
            return false;
        }

        // **Only what was SAID is remembered.** A silent rejection must not
        // become "the last close", or it would go on to silence a real break
        // that ended the same way.
        match level.closes.iter_mut().find(|(when, _)| when == on) {
            Some((_, last)) if *last == did => false,

            Some((_, last)) => {
                *last = did;
                true
            }

            None => {
                level.closes.push((on.to_string(), did));
                true
            }
        }
    }

    /// The last close this timeframe reported at this level. For tests, and
    /// for reading the state back.
    pub fn last_close(&self, band: &Band, on: &str) -> Option<AtZone> {
        self.seen
            .iter()
            .find(|one| one.band.price == band.price)?
            .closes
            .iter()
            .find(|(when, _)| when == on)
            .map(|(_, did)| *did)
    }

    /// Has this level said anything yet? **While it has not, approaching is
    /// news; once it has, it never is again.**
    pub fn has_spoken(&self, band: &Band) -> bool {
        self.seen
            .iter()
            .find(|one| one.band.price == band.price)
            .is_some_and(|one| one.spoken)
    }

    pub fn count(&self) -> usize {
        self.seen.len()
    }

    /// Every band being watched, whether price is at it or not. For the
    /// heartbeat, which reports what is being looked after rather than what
    /// happened.
    pub fn bands(&self) -> Vec<Band> {
        self.seen.iter().map(|level| level.band).collect()
    }

    /// The last price it was given.
    ///
    /// For the report made when watching RESUMES — it has to say where price
    /// is, and the socket may not send another for a second or two.
    pub fn last_price(&self) -> Option<Decimal> {
        self.last
    }

    /// Which bands price is at. For a heartbeat, not an alert.
    pub fn resting_at(&self) -> Vec<Band> {
        self.seen
            .iter()
            .filter(|level| level.deepest != Nearness::Away)
            .map(|level| level.band)
            .collect()
    }
}

/// One of his levels, and everything remembered about it.
#[derive(Debug, Clone)]
struct Level {
    band: Band,

    /// The deepest price has got this visit. **For the wording, not the
    /// decision** — a card has to say whether price is approaching the zone or
    /// standing in it.
    deepest: Nearness,

    /// **Has this level ever said anything at all?**
    ///
    /// Settled with him on 31 August 2026, after a day of the other way:
    ///
    /// ```text
    ///     price comes up to the level      approaching   <- one card
    ///     wobbles off and back             silence
    ///     the candle closes below          closed below  <- one card
    ///     a later candle comes back        silence
    ///     another one comes back           silence
    ///     a candle closes ABOVE            closed above  <- this he wants
    /// ```
    ///
    /// **Approaching is said once and never again.** Once anything has been
    /// said about a level, "price is near it" stops being news — the level has
    /// a story now, and only a different ending changes it.
    ///
    /// His words: *"if the price goes below that level and then it's coming
    /// back again on that level we don't get anything until price closes above
    /// that level again."*
    spoken: bool,

    /// The last close **reported** per timeframe, by its stored spelling.
    ///
    /// **What was reported, not what happened.** A rejection is silent, so it
    /// does not become "the last close" — otherwise a silent one would go on
    /// to silence a real break that ended the same way.
    ///
    /// **Per timeframe, and that is deliberate.** A 4-hour candle closing below
    /// a weekly level and a daily candle doing the same are two different
    /// pieces of news about one line, and the daily is the bigger one. Sharing
    /// one memory would let whichever arrived first silence the other.
    ///
    /// What they DO share is `spoken` above — a close on any timeframe ends
    /// the approach card for good, which is what he asked for.
    closes: Vec<(String, AtZone)>,
}

/// Which side of a band price is on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Above,
    Below,
}

/// **Which side this candle came from**, or `None` if it opened in the zone.
///
/// Read off the candle's OPEN, and that choice is the fix of 31 August 2026.
///
/// ## Why not remember it from the prices
///
/// It was remembered from the tick stream for about an hour, and the tick
/// stream and the candle poll are not the same clock. A 4-hour candle closes
/// above the band at 12:00; the poll picks it up seconds later; and in those
/// seconds the ticker has already put price above. The break then read as a
/// rejection and went silent — **and the harder the break, the more certain
/// that was**, which is exactly backwards.
///
/// The open cannot race. It is a fact about a candle that has finished.
///
/// **It is also the only version the backtester can run**, and that is what
/// really decides it. There is no tick stream in a backtest, so a remembered
/// side would have made the backtest and the live bot answer differently —
/// the one mismatch `CLAUDE.md` says never to build, because it makes results
/// look better rather than broken.
///
/// A candle that opened inside the zone came from nowhere: it started there.
/// That is `None`, and `None` speaks.
pub fn came_from(band: &Band, bar: &Bar) -> Option<Side> {
    if bar.open > band.top {
        Some(Side::Above)
    } else if bar.open < band.bottom {
        Some(Side::Below)
    } else {
        None
    }
}

/// Is this close worth a card at all?
///
/// **A break, or a candle that settled inside. Never a rejection.**
///
/// ```text
///     came from below, closed above   broke through   -> card
///     came from above, closed below   broke through   -> card
///     came from below, closed below   thrown back     -> silence
///     came from above, closed above   thrown back     -> silence
///     closed inside                   still there     -> card
/// ```
///
/// `came_from` is the candle's own open — see [`came_from`].
///
/// **Not knowing counts as worth saying.** A candle that opened inside the
/// zone has no side to be judged against, and silence about a level price is
/// standing in is worse than a card he did not need. Saying it twice is
/// stopped by the per-timeframe memory instead, not by this.
fn worth_a_card(came_from: Option<Side>, did: AtZone) -> bool {
    match did {
        AtZone::Missed => false,
        AtZone::ClosedInside => true,

        AtZone::ClosedAbove => came_from != Some(Side::Above),
        AtZone::ClosedBelow => came_from != Some(Side::Below),
    }
}

/// How far in each state counts as being.
///
/// **Away, then approaching, then inside.** Each step down is worth one
/// message; the same step twice is not.
fn depth(near: Nearness) -> u8 {
    match near {
        Nearness::Away => 0,
        Nearness::Approaching => 1,
        Nearness::Inside => 2,
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
/// is right. Leaving has to be a real distance, or one visit becomes an
/// afternoon of alerts.
///
/// ## IT IS MEASURED PAST APPROACHING, NOT PAST THE BAND
///
/// **Measuring from the band was wrong, and he found it live on 31 August
/// 2026:** *"price approaches a level, price goes back, you keep sending me a
/// message every time... I got so many cards."*
///
/// `reach` is how far out still counts as approaching. Measure the way home
/// from the band instead and the two overlap — on his AUD/USD daily level the
/// band is 22.7 pips, so a band-relative reset is 2.3 pips out, while
/// approaching reaches 4.0. **Every price in that 1.7-pip sliver was both
/// "approaching" and "properly gone" at once**, so a wobble smaller than two
/// pips re-armed the alert and fired it again. Forever.
///
/// Sampling four prices an hour through August put it at **45 alerts on one
/// level in one month**. Real ticks arrive about once a second.
///
/// Now the way home starts where approaching ends, so the two can never
/// overlap however thin the band or wide the reach.
///
/// **Easy to trigger, hard to reset** — and now actually hard.
fn clear_of(band: &Band, price: Decimal, reach: Decimal) -> bool {
    let gone = band.thickness() * CLEAR_BY;

    price > band.top + reach + gone || price < band.bottom - reach - gone
}
