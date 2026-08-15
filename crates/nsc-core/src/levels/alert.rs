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

use super::{Band, Nearness, Pair};

/// The one line that goes under the picture.
///
/// **Short on purpose.** The card carries the numbers; this is what Telegram
/// shows in the notification banner, where only the first few words survive.
pub fn caption(pair: &Pair, band: &Band, near: Nearness, price: Decimal) -> String {
    let show = |value: Decimal| value.round_dp(pair.digits).to_string();

    let (mark, doing) = match near {
        Nearness::Inside => ("🔔", "is in"),

        // Away never gets here — `Watch::arrive` only gives back bands price
        // has actually arrived at.
        Nearness::Approaching | Nearness::Away => ("👀", "is coming up on"),
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
pub fn note(near: Nearness) -> &'static str {
    match near {
        Nearness::Inside => {
            "Price is in your zone. Watch the close — that is what says whether it was rejected or just passed through."
        }
        Nearness::Approaching | Nearness::Away => {
            "Price is coming up on your zone. Nothing has formed yet."
        }
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
