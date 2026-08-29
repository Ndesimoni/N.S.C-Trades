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

use super::{Action, AtZone, Band, Nearness, Pair};
use crate::candle::Bar;
use crate::candle::timeframe_name;

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

/// A price the way the cards write it.
///
/// **Rounded to the pair, and grouped in thousands.** The cards have done both
/// since the beginning; the captions did neither, so gold arrived as 4094.00
/// under a picture calling it 4,094.00.
pub fn pretty(value: Decimal, digits: u32) -> String {
    // `{:.*}` rather than `round_dp`. Rounding alone drops trailing zeros, so
    // a level typed as 4094 came out "4,094" beneath a card saying "4,094.00"
    // — the same number twice, written two ways.
    let text = format!("{:.*}", digits as usize, value);
    let (whole, rest) = text.split_once('.').unwrap_or((text.as_str(), ""));

    let (sign, digits_only) = match whole.strip_prefix('-') {
        Some(rest) => ("-", rest),
        None => ("", whole),
    };

    let grouped: String = digits_only
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(|chunk| String::from_utf8_lossy(chunk).into_owned())
        .collect::<Vec<_>>()
        .join(",");

    if rest.is_empty() {
        format!("{sign}{grouped}")
    } else {
        format!("{sign}{grouped}.{rest}")
    }
}

/// The one line that goes under the picture.
///
/// **Short on purpose.** The card carries the numbers; this is what Telegram
/// shows in the notification banner, where only the first few words survive.
pub fn caption(pair: &Pair, band: &Band, near: Nearness, news: News, price: Decimal) -> String {
    let show = |value: Decimal| pretty(value, pair.digits);

    let (mark, doing) = match (news, near) {
        (News::Already, Nearness::Inside) => ("📍", "is already in"),
        (News::Already, _) => ("📍", "is already at"),
        (News::Fresh, Nearness::Inside) => ("🔔", "is in"),

        // Away never gets here — `Watch::arrive` and `Watch::resting_at` only
        // give back bands price is actually at.
        (News::Fresh, _) => ("👀", "is coming up on"),
    };

    format!(
        "{} <b>{}</b> {} your <b>{}</b> zone at {}. Now {}.",
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
pub fn closed_caption(
    pair: &Pair,
    band: &Band,
    bar: &Bar,
    did: AtZone,
    was: Action,
    interval: &str,
) -> String {
    let show = |value: Decimal| pretty(value, pair.digits);
    let each = timeframe_name(interval);

    format!(
        "🕯 <b>{}</b> — the {} candle <b>{}</b> your {} zone at {}. It closed at {}.",
        pair.symbol,
        each,
        happening(was, did),
        band.timeframe.name(),
        show(band.price),
        show(bar.close),
    )
}

/// The action in three or four words, with the side it happened on.
///
/// **The side matters as much as the action.** "Kissed it" says nothing about
/// whether the level held support or resistance; "kissed it and held above"
/// says both.
pub fn happening(was: Action, did: AtZone) -> &'static str {
    let above = did == AtZone::ClosedAbove;

    match was {
        Action::Kissed if above => "kissed it and held above",
        Action::Kissed => "kissed it and held below",
        Action::Rejected if above => "was pushed back above",
        Action::Rejected => "was pushed back below",
        Action::Settled => "closed inside",
        Action::CutThrough if above => "cut straight up through",
        Action::CutThrough => "cut straight down through",

        // Never sent — `worth_saying` is checked before anything gets here.
        Action::Missed => "did nothing at",
    }
}

/// The action as a short name for the card, and one line saying what it means.
pub fn happening_words(was: Action, did: AtZone) -> (&'static str, &'static str) {
    let above = did == AtZone::ClosedAbove;

    match was {
        Action::Kissed => (
            "kissed it",
            "It grazed the zone and closed straight back out. A touch, not a fight.",
        ),
        Action::Rejected if above => (
            "pushed back",
            "It drove into the zone and was sold back out above it.",
        ),
        Action::Rejected => (
            "pushed back",
            "It drove into the zone and was bought back out below it.",
        ),
        Action::Settled => (
            "closed inside",
            "Price is sitting in the zone. Nothing is settled yet.",
        ),
        Action::CutThrough => (
            "cut through",
            "It went in one side and out the other. The level did not hold.",
        ),
        Action::Missed => ("no touch", "It never reached the zone."),
    }
}
