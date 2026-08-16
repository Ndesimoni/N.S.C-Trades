//! Firing once per touch, not once per price.

use rust_decimal::Decimal;

use super::super::{Band, Nearness, Timeframe, Watch, nearness};
use super::support::d;

/// A pip on gold. It is quoted to two decimals, so a pip is ten cents.
fn reach() -> Decimal {
    d("0.10")
}

/// The gold band he drew at 4094 — 4055.43 to 4132.57, about 77 thick.
///
/// So a touch is anything up to 4132.67, and the band goes quiet again only
/// past 4140.28 — a tenth of its own thickness clear.
fn gold() -> Band {
    Band::around(Timeframe::Weekly, d("4094"), d("220.42"), d("0.35"))
}

fn watching() -> Watch {
    Watch::over(vec![gold()], reach())
}

// ── A touch, not a crossing ──

// The band's top is 4132.57. Price at 4132.60 has touched it in every way that
// matters, and staying quiet over three cents would be silly.
#[test]
fn a_price_just_outside_still_counts_as_arriving() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    let arrived = watch.arrive(d("4132.60"));

    assert_eq!(arrived.len(), 1, "a touch is an arrival");
    assert_eq!(arrived[0].1, Nearness::Approaching, "and it says which");
}

#[test]
fn price_inside_the_band_says_so() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    assert_eq!(watch.arrive(d("4100"))[0].1, Nearness::Inside);
}

// The slack is a pip, NOT time to react. The band already gives him that — its
// top is about three hours of gold movement from the line he drew.
#[test]
fn the_slack_is_a_pip_and_no_more() {
    let band = gold();

    assert_eq!(
        nearness(&band, d("4132.60"), reach()),
        Nearness::Approaching
    );
    assert_eq!(nearness(&band, d("4132.50"), reach()), Nearness::Inside);

    // A dollar out is not a touch, and neither is anything beyond.
    assert_eq!(nearness(&band, d("4133.60"), reach()), Nearness::Away);
    assert_eq!(nearness(&band, d("4140"), reach()), Nearness::Away);
}

#[test]
fn how_close_counts_is_a_setting() {
    let band = gold();

    // Nothing counts but the band itself.
    assert_eq!(nearness(&band, d("4132.60"), d("0")), Nearness::Away);

    // A pip of slack reaches it.
    assert_eq!(
        nearness(&band, d("4132.60"), d("0.10")),
        Nearness::Approaching
    );

    // Ten pips reaches further.
    assert_eq!(nearness(&band, d("4133.50"), d("0.10")), Nearness::Away);
    assert_eq!(
        nearness(&band, d("4133.50"), d("1.00")),
        Nearness::Approaching
    );
}

// ── Once per touch ──

// The reason the type exists. Prices come about once a second and barely move.
//
// Once price is INSIDE, going further in says nothing — it is already as deep
// as it gets.
#[test]
fn sitting_inside_fires_nothing_more() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    assert_eq!(watch.arrive(d("4132.50")).len(), 1, "it went in");

    for price in ["4130", "4120", "4100", "4060"] {
        assert!(
            watch.arrive(d(price)).is_empty(),
            "{price} was already inside"
        );
    }
}

// THE ONE HE ASKED FOR. Price drifts up to the zone, then walks into it.
//
// Coming near used to mark the band as reached, so walking IN was not a change
// and said nothing at all — he heard "coming up on your zone" and then waited
// for a candle, which on the hourly is up to an hour. Entering is the thing he
// actually wanted to know.
#[test]
fn coming_near_and_then_going_in_are_two_different_messages() {
    let mut watch = watching();
    watch.arrive(d("4300"));

    let near = watch.arrive(d("4132.60"));
    assert_eq!(near.len(), 1);
    assert_eq!(near[0].1, Nearness::Approaching, "first, that it is close");

    let inside = watch.arrive(d("4132.50"));
    assert_eq!(inside.len(), 1);
    assert_eq!(inside[0].1, Nearness::Inside, "then, that it is in");
}

// And never more than those two. Drifting back out to the edge and in again is
// one visit, however many times it happens.
#[test]
fn going_in_and_out_of_the_edge_is_still_one_visit() {
    let mut watch = watching();
    watch.arrive(d("4300"));
    watch.arrive(d("4132.60"));
    watch.arrive(d("4132.50"));

    let mut fired = 0;
    for price in ["4132.60", "4132.50", "4132.65", "4130", "4132.62"] {
        fired += watch.arrive(d(price)).len();
    }

    assert_eq!(fired, 0, "in and out of the same edge, five times");
}

#[test]
fn leaving_and_coming_back_fires_again() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    watch.arrive(d("4100"));
    watch.arrive(d("4300"));

    assert_eq!(watch.arrive(d("4100")).len(), 1, "a second visit");
}

// Price hovering at the edge crosses it again and again, and describes one
// moment where nothing happened.
#[test]
fn hovering_at_the_edge_does_not_fire_over_and_over() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    assert_eq!(
        watch.arrive(d("4132.60")).len(),
        1,
        "the first touch counts"
    );

    // All still OUTSIDE the band and inside its reach — the same depth it
    // already reported, five times over.
    let mut fired = 0;
    for price in ["4132.65", "4132.62", "4132.67", "4132.58", "4132.61"] {
        fired += watch.arrive(d(price)).len();
    }

    assert_eq!(fired, 0, "the same moment, told five times");
}

// Leaving is a REAL distance. A pip back out must not reset the band, or the
// next pip back in is a second alert for one visit.
#[test]
fn leaving_takes_more_than_it_took_to_arrive() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    watch.arrive(d("4132.60"));

    // Well outside a touch, still nowhere near clear of the band.
    assert!(watch.arrive(d("4139")).is_empty(), "not gone yet");
    assert!(
        watch.arrive(d("4132.60")).is_empty(),
        "so coming back is not new"
    );

    // Past a tenth of the band's thickness — 4140.28 — it is properly gone.
    watch.arrive(d("4141"));
    assert_eq!(
        watch.arrive(d("4132.60")).len(),
        1,
        "now it is a fresh touch"
    );
}

// ── The first price ──

// It says where price IS. It cannot say price has ARRIVED — it may have been
// sitting there for hours before the bot started, and an alert for that is a
// lie about when it happened.
#[test]
fn the_very_first_price_never_fires() {
    let mut watch = watching();

    assert!(watch.arrive(d("4100")).is_empty(), "started at it");
    assert_eq!(watch.resting_at().len(), 1, "but it knows where price is");
}

#[test]
fn starting_at_a_band_still_fires_on_a_real_arrival_later() {
    let mut watch = watching();

    watch.arrive(d("4100"));
    watch.arrive(d("4400"));

    assert_eq!(watch.arrive(d("4100")).len(), 1);
}

// ── Several bands ──

#[test]
fn only_the_band_price_arrived_at_fires() {
    let daily = Band::around(Timeframe::Daily, d("2984"), d("70.36"), d("0.46"));
    let mut watch = Watch::over(vec![gold(), daily], reach());

    watch.arrive(d("3500"));

    let arrived = watch.arrive(d("4100"));
    assert_eq!(arrived.len(), 1);
    assert_eq!(arrived[0].0.timeframe, Timeframe::Weekly);
}

#[test]
fn a_price_near_nothing_fires_nothing() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    assert!(watch.arrive(d("4280")).is_empty());
}

// ── Before the first price ──

// NOTHING IS RESTING ANYWHERE UNTIL A PRICE HAS BEEN FED IN. Obvious written
// down, and it cost the report that says where price already stands: the
// watcher asked what was resting BEFORE handing over the first price, found
// nothing, and marked the session as reported.
#[test]
fn nothing_is_resting_at_a_zone_before_the_first_price() {
    let watch = watching();

    assert!(watch.last_price().is_none(), "no price has arrived");
    assert!(
        watch.resting_at().is_empty(),
        "so nothing can be resting anywhere"
    );
}

// And the very first price only says where price IS. It is not an arrival —
// there is nothing yet for it to have arrived from.
#[test]
fn the_first_price_makes_price_rest_without_arriving() {
    let mut watch = watching();

    let arrived = watch.arrive(d("4100"));

    assert!(arrived.is_empty(), "the first price is not an arrival");
    assert_eq!(watch.resting_at().len(), 1, "but it IS resting there now");
    assert_eq!(watch.last_price(), Some(d("4100")));
}
