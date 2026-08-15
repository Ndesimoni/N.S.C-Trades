//! What an alert says.
//!
//! **An alert is not a signal.** No entry, no stop, no target — because there
//! is no trade. Price has arrived where he would be waiting and nothing has
//! formed. If these two ever start looking alike, the price watcher has
//! quietly become a strategy nobody reviewed.
//!
//! The words live here, in the crate that cannot reach anything, so they can
//! be read and tested without a network or a browser.

use rust_decimal::Decimal;

use super::{AtZone, Band, Nearness, Pair};
use crate::candle::Bar;

/// Whether we watched this happen or found it already so.
///
/// [`Nearness`] says *where price is* — geometry, with no opinion. This says
/// *what to tell him*, and the only thing it adds is whether anybody was
/// watching.
///
/// It matters because the bot does not watch on Mondays. Price can walk into a
/// zone on Monday and still be sitting there when Tuesday opens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum News {
    /// It just happened, and we saw it.
    Fresh,

    /// It was already so when watching resumed.
    ///
    /// **Never dress this up as an arrival.** It may have happened during
    /// Monday's silence, and a card saying "arrived" would put a Monday move
    /// on a Tuesday clock.
    Already,
}

/// The one line that goes under the picture.
///
/// **Short on purpose.** The card carries the numbers; this is what Telegram
/// shows in the notification banner, where only the first few words survive.
pub fn caption(pair: &Pair, band: &Band, near: Nearness, news: News, price: Decimal) -> String {
    let show = |value: Decimal| value.round_dp(pair.digits).to_string();

    let (mark, doing) = match (news, near) {
        (News::Already, Nearness::Inside) => ("📍", "is already in"),
        (News::Already, _) => ("📍", "is already at"),
        (News::Fresh, Nearness::Inside) => ("🔔", "is in"),

        // Away never gets here — `Watch::arrive` and `Watch::resting_at` only
        // give back bands price is actually at.
        (News::Fresh, _) => ("👀", "is coming up on"),
    };

    format!(
        "{} <b>{}</b> {} your <b>{}</b> zone at {} — now {}",
        mark,
        pair.symbol,
        doing,
        band.timeframe.name(),
        show(band.price),
        show(price),
    )
}

/// The sentence on the card that says what to do next.
///
/// Being *at* a level says nothing on its own — price may cut straight
/// through. **The close is what says it was a rejection**, so the inside
/// wording points at the thing that has not happened yet.
pub fn note(near: Nearness, news: News) -> &'static str {
    match (news, near) {
        (News::Already, Nearness::Inside) => {
            "Price was already in your zone when watching resumed. This is where things stand, not something that just happened."
        }
        (News::Already, _) => {
            "Price was already up against your zone when watching resumed. Nothing has just happened."
        }
        (News::Fresh, Nearness::Inside) => {
            "Price is in your zone. Watch the close — that is what says whether it was rejected or just passed through."
        }
        (News::Fresh, _) => "Price is coming up on your zone. Nothing has formed yet.",
    }
}

/// How far outside the band price is. Nought if it is inside.
pub fn gap(band: &Band, price: Decimal) -> Decimal {
    if price > band.top {
        price - band.top
    } else if price < band.bottom {
        band.bottom - price
    } else {
        Decimal::ZERO
    }
}

/// The line under a rung 2 card — a candle that touched one of his zones has
/// finished.
///
/// **Still not a signal.** It says what the candle did, not what to do about
/// it. A close above his zone is a fact; whether it is a trade is rung 3.
pub fn closed_caption(pair: &Pair, band: &Band, bar: &Bar, did: AtZone, interval: &str) -> String {
    let show = |value: Decimal| value.round_dp(pair.digits).to_string();

    let (mark, doing) = match did {
        AtZone::ClosedInside => ("🕯", "closed inside"),
        AtZone::ClosedAbove => ("🕯", "reached in and closed above"),
        AtZone::ClosedBelow => ("🕯", "reached in and closed below"),

        // Never sent — a miss is not worth saying, and `worth_saying` is
        // checked before anything gets here.
        AtZone::Missed => ("🕯", "did nothing at"),
    };

    format!(
        "{} <b>{}</b> — the {} candle {} your <b>{}</b> zone at {}. Closed {}.",
        mark,
        pair.symbol,
        interval,
        doing,
        band.timeframe.name(),
        show(band.price),
        show(bar.close),
    )
}
