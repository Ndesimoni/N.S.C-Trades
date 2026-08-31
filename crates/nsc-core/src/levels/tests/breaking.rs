//! **What a close at a level is worth** — a break, or a rejection.
//!
//! Settled with him on 31 August 2026. A candle that broke through the way
//! price was travelling is news. One that was thrown back where it came from
//! is not — it still reaches him as a setup if a shape printed, which is the
//! message that was actually about it.

use super::super::{AtZone, Watch, came_from};
use super::support::{aussie, bar, d, from_above, from_below, gold, share};

fn watching() -> Watch {
    Watch::over(vec![gold()], share())
}

/// **A close is the only thing that follows an approach.**
///
/// His sequence, 31 August 2026: approaching, then the candle closes below,
/// then silence however often price comes back — until a candle closes ABOVE.
#[test]
fn a_close_speaks_and_then_only_a_different_close_does() {
    let mut watch = Watch::over(vec![aussie()], share());
    let band = aussie();

    watch.arrive(d("0.71800"));
    assert_eq!(watch.arrive(d("0.71390")).len(), 1, "approaching");

    // The candle fell in from above and closed below. A break, so it speaks.
    assert!(
        watch.closed(&band, "4h", AtZone::ClosedBelow, from_above()),
        "closed below"
    );

    // Later candles come back to the level. Silence — his exact complaint.
    watch.arrive(d("0.71300"));
    assert!(
        watch.arrive(d("0.71390")).is_empty(),
        "coming back is silent"
    );
    assert!(watch.arrive(d("0.71390")).is_empty(), "and again");

    // Another candle wicks up into the level and is thrown back. It opened
    // below and closed below, so it broke nothing — silence.
    assert!(
        !watch.closed(&band, "4h", AtZone::ClosedBelow, from_below()),
        "thrown back where it came from is not news"
    );

    // A close ABOVE is a different ending. This he wants.
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

    // Price is below, and rises into the level.
    watch.arrive(d("0.71000"));
    watch.arrive(d("0.71390"));

    // A 4-hour candle breaks up through it.
    assert!(
        watch.closed(&band, "4h", AtZone::ClosedAbove, from_below()),
        "a break up"
    );

    // A daily candle later breaks back down. Different direction, so news —
    // and on its own timeframe.
    assert!(
        watch.closed(&band, "1d", AtZone::ClosedBelow, from_above()),
        "a break down"
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

    watch.arrive(d("0.71000"));
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
/// 31 August 2026, asked directly whether he wanted a card when price is
/// thrown back where it came from: *"I do not want a notification on it."*
///
/// It is not lost — a shape printing there still reaches him as a setup.
#[test]
fn being_thrown_back_says_nothing() {
    let mut watch = Watch::over(vec![aussie()], share());
    let band = aussie();

    // Price is below and rises into the level.
    watch.arrive(d("0.71000"));
    watch.arrive(d("0.71390"));

    // The candle is refused and closes back below. Silence.
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

/// **A candle settling inside the zone always speaks**, whichever way price
/// came. His call: *"within the zone stays same."*
#[test]
fn settling_inside_the_zone_still_speaks() {
    let mut watch = Watch::over(vec![aussie()], share());
    let band = aussie();

    watch.arrive(d("0.71000"));
    assert!(watch.closed(&band, "4h", AtZone::ClosedInside, from_below()));

    // But not twice in a row.
    assert!(!watch.closed(&band, "4h", AtZone::ClosedInside, from_below()));
}

/// **Any close on any timeframe ends the approach card for good.**
///
/// Price being near a line stops being news the moment the line has a story.
#[test]
fn a_close_on_any_timeframe_silences_the_approach() {
    let mut watch = Watch::over(vec![aussie()], share());
    let band = aussie();

    // A daily close, before price has ever been reported as approaching.
    watch.closed(&band, "1d", AtZone::ClosedAbove, from_below());

    watch.arrive(d("0.71800"));
    assert!(
        watch.arrive(d("0.71390")).is_empty(),
        "the level already has a story"
    );
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

    watch.arrive(d("4000"));
    watch.arrive(d("4100"));

    // The candle closes above and price runs on past the reset line.
    watch.arrive(d("4150"));

    assert!(
        watch.bands().iter().any(|one| one.price == gold().price),
        "the level did not stop existing because price left it"
    );
    assert!(
        watch.resting_at().is_empty(),
        "and price really has gone — which is why resting_at was the wrong list"
    );
}

/// **The close must be judged by the CANDLE, not by where the ticker is now.**
///
/// Same race, one layer down. Price came up from below, so a close above is a
/// break. But by the time the poll runs, the ticker has moved the remembered
/// side to Above — so the break read as a rejection and went quiet.
///
/// The candle's own open answers it and cannot race: it is a fact about a
/// finished candle. **It is also the only version the backtester can run**,
/// which is the rule that decides it — tick memory does not exist there.
#[test]
fn a_break_is_judged_by_the_candle_not_by_the_ticker() {
    let mut watch = watching();

    // Price rises from below, into the zone, then breaks out the top.
    watch.arrive(d("4000"));
    watch.arrive(d("4100"));
    watch.arrive(d("4150"));

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
