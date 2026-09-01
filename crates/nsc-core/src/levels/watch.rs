//! Watching bands, and saying what a candle did at one.
//!
//! ## Price arriving at a level says nothing any more
//!
//! **His call, 1 September 2026:** *"when price is getting to a level we do not
//! want an alert, so remove the card."*
//!
//! Two messages are left in the whole bot:
//!
//! ```text
//!   a candle BREAKS a level    came from below and closed above, or
//!                              came from above and closed below
//!   a shape he trades          at a level, or within half a band of one
//! ```
//!
//! Nothing else. Price walking into a zone, sitting in it, wobbling at its
//! edge, leaving and coming back — all silent.
//!
//! **A lot of machinery went with the card**, and that is the point of writing
//! it down. There used to be a "deepest price has got this visit" per band, a
//! leaving distance measured wider than the arriving one, and a "has this
//! level ever spoken" flag. All three existed to make one visit fire one
//! alert. With no alert there is no visit to count, so they are gone rather
//! than left lying around looking load-bearing. They are in git at `99ed9f1`
//! if approach cards ever come back.
//!
//! What is left holds one thing per band: **the last close each timeframe
//! reported there**, which is what stops the same news going out twice.

use rust_decimal::Decimal;

use super::{AtZone, Band};
use crate::candle::Bar;

/// Watches a set of bands, and says whether a candle closing at one is worth a
/// card.
pub struct Watch {
    seen: Vec<Level>,

    /// How close counts as being **at** a band, **as a share of that band's own
    /// thickness**, from [`Pair::reach_share`](super::Pair::reach_share).
    ///
    /// **A share, so every band gets a reach its own size.** It was one price
    /// for the whole pair until 31 August 2026 — four pips, which is 22% of an
    /// AUD/USD daily band and 0.03% of a gold weekly one.
    ///
    /// Only [`Watch::resting_at`] reads it now, for the report made when
    /// watching resumes.
    share: Decimal,

    /// The last price seen.
    ///
    /// For the report made when watching RESUMES — it has to say where price
    /// is, and the socket may not send another for a second or two.
    last: Option<Decimal>,
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
                    closes: Vec::new(),
                })
                .collect(),
            share,
            last: None,
        }
    }

    /// Feeds in a price. **It says nothing and sends nothing.**
    ///
    /// Prices come down the websocket about once a second and barely move.
    /// All this keeps is the latest one, so the report made when watching
    /// resumes can say where price stands without waiting for the next.
    pub fn saw(&mut self, price: Decimal) {
        self.last = Some(price);
    }

    /// A candle closed at this level. **Is it worth a card?**
    ///
    /// Yes when it **broke through in the direction price was travelling** —
    /// and only when that is a different ending from the last one this
    /// timeframe reported.
    ///
    /// ## A rejection is silent, and that is his call
    ///
    /// 1 September 2026: *"we should only get alerts if the price came from
    /// below the band level and closed above it, and vice versa."*
    ///
    /// **The rejection is not lost.** It reaches him as a SETUP if a shape he
    /// trades printed there, which is rung 3 and runs on its own: *"as for the
    /// setups, candlestick patterns, it stays the same."*
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

    pub fn count(&self) -> usize {
        self.seen.len()
    }

    /// Every band being watched, whether price is at it or not.
    ///
    /// **This is the list closes are reported on.** Not the bands price is
    /// standing on — a break is price LEAVING a zone, so by the time the poll
    /// runs it has often gone.
    pub fn bands(&self) -> Vec<Band> {
        self.seen.iter().map(|level| level.band).collect()
    }

    /// The last price it was given.
    pub fn last_price(&self) -> Option<Decimal> {
        self.last
    }

    /// Which bands price is at **right now**. For the report made when
    /// watching resumes, and nothing else.
    ///
    /// **Measured fresh, every time.** It used to be a remembered "deepest
    /// this visit" with a wider distance for leaving than for arriving, so
    /// that one visit fired one alert. There is no alert now, and a report
    /// sent once a session wants to know where price is — not where it has
    /// been since it last properly left.
    pub fn resting_at(&self) -> Vec<Band> {
        let Some(price) = self.last else {
            return Vec::new();
        };

        self.seen
            .iter()
            .filter(|level| {
                let reach = level.band.thickness() * self.share;
                nearness(&level.band, price, reach) != Nearness::Away
            })
            .map(|level| level.band)
            .collect()
    }
}

/// One of his levels, and everything remembered about it.
#[derive(Debug, Clone)]
struct Level {
    band: Band,

    /// The last close **reported** per timeframe, by its stored spelling.
    ///
    /// **What was reported, not what happened.** A rejection is silent, so it
    /// does not become "the last close" — otherwise a silent one would go on
    /// to silence a real break that ended the same way.
    ///
    /// **Per timeframe, and that is deliberate.** A 4-hour candle breaking a
    /// weekly level and a daily candle breaking it are two different pieces of
    /// news about one line, and the daily is the bigger one. Sharing one
    /// memory would let whichever arrived first silence the other.
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
/// **A break, and nothing else.**
///
/// ```text
///     came from below, closed above   broke through   -> card
///     came from above, closed below   broke through   -> card
///     came from below, closed below   thrown back     -> silence
///     came from above, closed above   thrown back     -> silence
///     closed inside                   still there     -> see below
/// ```
///
/// **A candle that settled INSIDE the band is not a break**, and since
/// 31 August it is silent too — `only_breaks` in `config/levels.toml`. It is
/// left as a setting rather than deleted because it is the one line he might
/// want back: a candle closing in a zone is the most common of the three and
/// says the least, but it does say price is standing there.
///
/// **Not knowing counts as worth saying.** A candle that opened inside the
/// zone has no side to be judged against. Saying it twice is stopped by the
/// per-timeframe memory instead, not by this.
fn worth_a_card(came_from: Option<Side>, did: AtZone) -> bool {
    match did {
        AtZone::Missed => false,
        AtZone::ClosedInside => true,

        AtZone::ClosedAbove => came_from != Some(Side::Above),
        AtZone::ClosedBelow => came_from != Some(Side::Below),
    }
}

/// How near this price is to this band.
///
/// `reach` is a **price**, not a share — this band's own thickness already
/// multiplied through.
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
