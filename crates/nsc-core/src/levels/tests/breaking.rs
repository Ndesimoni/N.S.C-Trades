//! **What a close at a level is worth** — a break, and nothing else.
//!
//! Settled with him on 1 September 2026: *"we should only get alerts if the
//! price came from below the band level and closed above it, and vice versa."*
//!
//! A candle thrown back where it came from says nothing here. It still reaches
//! him as a setup if a shape printed, which is the message that was actually
//! about it.

use super::super::{AtZone, Watch, came_from};
use super::support::{aussie, bar, d, from_above, from_below, gold, share};

fn watching() -> Watch {
    Watch::over(vec![gold()], share())
}

/// **A break speaks, and then only a break the other way does.**
///
/// His sequence: the candle closes below, then silence however often price
/// comes back — until a candle closes ABOVE.
#[test]
fn a_break_speaks_and_then_only_a_break_back_does() {
    let mut watch = Watch::over(vec![aussie()], share());
    let band = aussie();

    // The candle fell in from above and closed below. A break, so it speaks.
    assert!(
        watch.closed(&band, "4h", AtZone::ClosedBelow, from_above()),
        "closed below"
    );

    // Another candle wicks up into the level and is thrown back. It opened
    // below and closed below, so it broke nothing — silence.
    assert!(
        !watch.closed(&band, "4h", AtZone::ClosedBelow, from_below()),
        "thrown back where it came from is not news"
    );

    // A candle that opens below and closes above. This he wants.
    assert!(
        watch.closed(&band, "4h", AtZone::ClosedAbove, from_below()),
        "closed above"
    );
}

/// **Each timeframe keeps its own record of what it has said.**
///
/// A 4-hour breaking a weekly level and a daily breaking it the other way are
/// two pieces of news about one line, and both are worth hearing.
#[test]
fn the_daily_and_the_four_hour_each_speak() {
    let mut watch = Watch::over(vec![aussie()], share());
    let band = aussie();

    assert!(
        watch.closed(&band, "4h", AtZone::ClosedAbove, from_below()),
        "a break up"
    );

    assert!(
        watch.closed(&band, "1d", AtZone::ClosedBelow, from_above()),
        "a break back down, on its own timeframe"
    );
}

/// **A candle that starts on the side it ends on broke nothing.**
///
/// Once price is above a level, a candle that opens above and closes above has
/// only wicked back into it and carried on. That is where price already was.
#[test]
fn a_candle_that_opened_where_it_closed_broke_nothing() {
    let mut watch = Watch::over(vec![aussie()], share());
    let band = aussie();

    assert!(
        watch.closed(&band, "4h", AtZone::ClosedAbove, from_below()),
        "a break up"
    );

    // The next one opens above, dips into the zone, and closes above again.
    assert!(
        !watch.closed(&band, "4h", AtZone::ClosedAbove, from_above()),
        "price was already above — nothing was broken"
    );

    // And the daily judges its OWN candle, not the 4-hour's.
    assert!(
        watch.closed(&band, "1d", AtZone::ClosedAbove, from_below()),
        "the daily candle did open below, so for the daily it is a break"
    );
}

/// **A rejection is silent, and that is his call.**
///
/// Asked directly whether he wanted a card when price is thrown back where it
/// came from: *"I do not want a notification on it."*
///
/// It is not lost — a shape printing there still reaches him as a setup.
#[test]
fn being_thrown_back_says_nothing() {
    let mut watch = Watch::over(vec![aussie()], share());
    let band = aussie();

    // Price came up from below, was refused, and closed back below.
    assert!(
        !watch.closed(&band, "4h", AtZone::ClosedBelow, from_below()),
        "rejected back where it came from is not a card"
    );

    // And the break that follows still speaks.
    assert!(
        watch.closed(&band, "4h", AtZone::ClosedAbove, from_below()),
        "the break does"
    );
}

/// **A candle settling inside the zone is not a break.**
///
/// `worth_a_card` still passes it — it is `only_breaks` in
/// `config/levels.toml` that stops it, and that setting is `true` since
/// 31 August 2026. **This test pins the switch, not the behaviour**: flip that
/// line and these are the cards that come back.
#[test]
fn settling_inside_the_zone_is_what_only_breaks_turns_off() {
    let mut watch = Watch::over(vec![aussie()], share());
    let band = aussie();

    assert!(watch.closed(&band, "4h", AtZone::ClosedInside, from_below()));

    // And not twice in a row, whatever the setting says.
    assert!(!watch.closed(&band, "4h", AtZone::ClosedInside, from_below()));
}

// ── The two bugs found on the 31 August read-back ──

/// **A break is price LEAVING the zone, and leaving is what used to hide it.**
///
/// The tick loop and the candle poll are not the same clock. A 4-hour candle
/// closes above the band at 12:00 and the poll picks it up seconds later —
/// and in those seconds the ticker has already carried price clear.
///
/// `resting_at` only lists bands price is at *now*, and `look` reported closes
/// on that list. So the harder the break, the more certainly it was silenced.
#[test]
fn a_band_price_has_broken_clear_of_is_still_worth_reporting() {
    let mut watch = watching();

    // The candle closed above and price has run on well clear of the band.
    watch.saw(d("4150"));

    assert!(
        watch.resting_at().is_empty(),
        "price really has gone — which is why resting_at was the wrong list"
    );
    assert!(
        watch.bands().iter().any(|one| one.price == gold().price),
        "but the level did not stop existing because price left it"
    );
}

/// **The close must be judged by the CANDLE, not by where the ticker is now.**
///
/// Same race, one layer down. Price came up from below, so a close above is a
/// break. But by the time the poll ran, the ticker had moved the remembered
/// side to Above — so the break read as a rejection and went quiet.
///
/// The candle's own open answers it and cannot race: it is a fact about a
/// finished candle. **It is also the only version the backtester can run**,
/// which is the rule that decides it — tick memory does not exist there.
#[test]
fn a_break_is_judged_by_the_candle_not_by_the_ticker() {
    let mut watch = watching();

    // The ticker has already carried price above the band.
    watch.saw(d("4150"));

    // The candle that did it opened below the band and closed above it.
    let broke = bar(d("4000"), d("4150"));

    assert!(
        watch.closed(
            &gold(),
            "4h",
            AtZone::ClosedAbove,
            came_from(&gold(), &broke)
        ),
        "it opened below and closed above — that is the break"
    );
}
