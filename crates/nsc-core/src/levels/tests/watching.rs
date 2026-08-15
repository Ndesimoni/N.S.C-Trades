//! Firing once per touch, not once per price — and firing on approach.

use rust_decimal::Decimal;

use super::super::{Band, Nearness, Timeframe, Watch, nearness};
use super::support::d;

/// A quarter of the band's thickness counts as arriving.
fn approach() -> Decimal {
    d("0.25")
}

/// The gold band he drew at 4094 — 4055.42 to 4132.57, about 77 thick.
/// So the zone reaches about 19 further, to roughly 4151.8 and 4036.1.
fn gold() -> Band {
    Band::around(Timeframe::Weekly, d("4094"), d("220.42"), d("0.35"))
}

fn watching() -> Watch {
    Watch::over(vec![gold()], approach())
}

// ── A band is a zone, not a wall ──

// The whole point of approach. 4140 is above the band's top of 4132.57 — but
// it is 8 away on a band 77 thick, and price at 4140 is at that level in every
// way that matters.
#[test]
fn price_near_the_band_counts_as_arriving() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    let arrived = watch.arrive(d("4140"));

    assert_eq!(arrived.len(), 1, "near enough is arrived");
    assert_eq!(arrived[0].1, Nearness::Approaching, "and it says which");
}

#[test]
fn price_inside_the_band_says_so() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    assert_eq!(watch.arrive(d("4100"))[0].1, Nearness::Inside);
}

// Told it is COMING, he can get to his chart before the reaction starts. An
// alert that waits for price to be strictly inside arrives too late to use.
#[test]
fn the_alert_comes_before_price_is_inside() {
    let band = gold();

    assert_eq!(
        nearness(&band, d("4140"), approach()),
        Nearness::Approaching
    );
    assert_eq!(nearness(&band, d("4132"), approach()), Nearness::Inside);
    assert_eq!(nearness(&band, d("4200"), approach()), Nearness::Away);
}

#[test]
fn how_close_counts_is_a_setting() {
    let band = gold();

    // Nothing counts but the band itself.
    assert_eq!(nearness(&band, d("4140"), d("0")), Nearness::Away);

    // Half a band's thickness of warning reaches further.
    assert_eq!(nearness(&band, d("4160"), d("0.25")), Nearness::Away);
    assert_eq!(nearness(&band, d("4160"), d("0.5")), Nearness::Approaching);
}

// ── Once per touch ──

// The reason the type exists. Prices come about once a second and barely move.
#[test]
fn sitting_at_a_band_fires_nothing_more() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    watch.arrive(d("4140"));

    for price in ["4139", "4135", "4100", "4060", "4145"] {
        assert!(
            watch.arrive(d(price)).is_empty(),
            "{price} was already at it"
        );
    }
}

#[test]
fn leaving_and_coming_back_fires_again() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    watch.arrive(d("4100"));
    watch.arrive(d("4300"));

    assert_eq!(watch.arrive(d("4100")).len(), 1, "a second visit");
}

// Price hovering at the edge of the zone crosses it again and again, and
// describes one moment where nothing happened.
#[test]
fn hovering_at_the_edge_does_not_fire_over_and_over() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    assert_eq!(
        watch.arrive(d("4151")).len(),
        1,
        "the first crossing counts"
    );

    let mut fired = 0;
    for price in ["4153", "4150", "4153", "4151", "4154"] {
        fired += watch.arrive(d(price)).len();
    }

    assert_eq!(fired, 0, "the same moment, told five times");
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
    let mut watch = Watch::over(vec![gold(), daily], approach());

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
