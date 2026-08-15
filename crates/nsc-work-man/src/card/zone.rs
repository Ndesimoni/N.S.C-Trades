//! The alert card — price arriving at one of his zones, drawn.
//!
//! **Why a picture and not a line of text.** Telegram gives no colour, no font
//! size and no layout, so every message looks like every other message. On a
//! card the state can actually look different, and the zone can be drawn on
//! the same shape he drew it on — he sees how close price is instead of
//! working it out from three numbers.

use std::path::{Path, PathBuf};

use rust_decimal::Decimal;

use serde_json::{Value, json};

use super::facts::rounded;
use super::{CardError, fill};
use nsc_core::candle::Bar;
use nsc_core::levels::{self, Action, AtZone, Band, Nearness, News, Pair};
use nsc_core::settings::{timeframe_name, unit_for};

/// The templates these cards are drawn from.
const ALERT: &str = "alert.html";
const CLOSE: &str = "close.html";

/// Draws one alert and gives back the picture's absolute path.
///
/// `reach` is how close counted as arriving — the card draws that line, so it
/// answers "why now?" without anybody writing the explanation.
///
/// `stamp` is the moment it happened, already worded. Nothing in `nsc-core`
/// reads a clock, and nothing here works one out twice.
#[allow(clippy::too_many_arguments)]
pub fn alert(
    pair: &Pair,
    band: &Band,
    near: Nearness,
    news: News,
    price: Decimal,
    reach: Decimal,
    stamp: &str,
    out: &Path,
) -> Result<PathBuf, CardError> {
    fill::draw(
        ALERT,
        &[(
            "/*__ALERT__*/",
            alert_facts(pair, band, near, news, price, reach, stamp).to_string(),
        )],
        out,
    )
}

/// Draws one rung 2 card — a finished candle inside the zone it touched.
///
/// The candle is drawn ON the band, because a wick deep in with the body
/// closing back out is a rejection and no arrangement of numbers says that as
/// fast as the shape does.
#[allow(clippy::too_many_arguments)]
pub fn closed(
    pair: &Pair,
    band: &Band,
    bar: &Bar,
    did: AtZone,
    was: Action,
    interval: &str,
    forming: bool,
    out: &Path,
) -> Result<PathBuf, CardError> {
    fill::draw(
        CLOSE,
        &[(
            "/*__CLOSE__*/",
            close_facts(pair, band, bar, did, was, interval, &bar.datetime, forming).to_string(),
        )],
        out,
    )
}

/// Everything the alert card says out loud.
///
/// **Every distance is worked out here.** The template places things on the
/// card and reads no prices of its own — the same rule the chart cards follow,
/// for the same reason: a number worked out in two places drifts in one.
fn alert_facts(
    pair: &Pair,
    band: &Band,
    near: Nearness,
    news: News,
    price: Decimal,
    reach: Decimal,
    stamp: &str,
) -> Value {
    let digits = pair.digits;

    json!({
        "symbol":    pair.symbol,
        "state":     if near == Nearness::Inside { "inside" } else { "approaching" },
        // Did anybody watch this happen? The card must not call a Monday move
        // a Tuesday arrival.
        "already":   news == News::Already,
        // Which side price is on, so the little picture puts it on the right
        // side of the line.
        "side":      if price > band.top {
            "above"
        } else if price < band.bottom {
            "below"
        } else {
            "inside"
        },
        "timeframe": band.timeframe.name(),
        "colour":    band.timeframe.colour(),
        "note":      levels::note(near, news),
        "stamp":     stamp,
        "unit":      unit_for(&pair.symbol),
        "digits":    digits,
        "price":     rounded(price, digits),
        "level":     rounded(band.price, digits),
        "top":       rounded(band.top, digits),
        "bottom":    rounded(band.bottom, digits),
        "gap":       rounded(levels::gap(band, price), digits),
        "from_line": rounded((price - band.price).abs(), digits),
        "reach":     rounded(reach, digits),
    })
}

/// Everything the close card says out loud.
///
/// **The candle is handed over finished.** Nothing here asks whether it is —
/// `Bar::finished_by` is the single place that decides, and a card drawn from
/// a candle still running would show a close that has not happened.
#[allow(clippy::too_many_arguments)]
fn close_facts(
    pair: &Pair,
    band: &Band,
    bar: &Bar,
    did: AtZone,
    was: Action,
    interval: &str,
    stamp: &str,
    forming: bool,
) -> Value {
    let digits = pair.digits;
    let deep = levels::how_deep(band, bar);
    let (name, means) = levels::happening_words(was, did);

    json!({
        "symbol":     pair.symbol,
        "did":        match did {
            AtZone::ClosedInside => "inside",
            AtZone::ClosedAbove  => "above",
            AtZone::ClosedBelow  => "below",
            AtZone::Missed       => "missed",
        },
        "timeframe":  band.timeframe.name(),
        "colour":     band.timeframe.colour(),
        "interval":   timeframe_name(interval),
        "action":     name,
        "note":       if forming { FORMING } else { means },
        // STILL RUNNING. The card has to say so on its face — this is the one
        // place in the project that reads a candle before it has finished, and
        // a card that looked final would put a guess where a fact belongs.
        "forming":    forming,
        // Which way the candle came from, so the little picture on the left
        // puts it on the right side of the line.
        "side":       match did {
            AtZone::ClosedAbove => "above",
            AtZone::ClosedBelow => "below",
            _ => "inside",
        },
        "stamp":      stamp,
        "digits":     digits,
        "open":       rounded(bar.open, digits),
        "high":       rounded(bar.high, digits),
        "low":        rounded(bar.low, digits),
        "close":      rounded(bar.close, digits),
        "level":      rounded(band.price, digits),
        "top":        rounded(band.top, digits),
        "bottom":     rounded(band.bottom, digits),
        "deep_words": deep_words(deep),
    })
}

/// How far in, in words. "A third of the way" beats 0.34 on a phone.
fn deep_words(deep: Decimal) -> String {
    let percent = (deep * Decimal::from(100)).round();

    if percent >= Decimal::from(98) {
        "all the way through".into()
    } else if percent <= Decimal::ONE {
        "barely at all".into()
    } else {
        format!("{percent}%")
    }
}

/// What a card says when the candle has not finished.
///
/// **Where it ends is the whole point**, and it has not ended. Everything on
/// the card is where it stands right now and could be the opposite in forty
/// minutes.
const FORMING: &str = "This candle is still running. Where it closes is what counts — this is only what it has done so far.";
