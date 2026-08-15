//! Firing once per touch, not once per price.

use super::super::{Band, Timeframe, Watch};
use super::support::d;

/// The gold band he drew at 4094 — 4055.42 to 4132.57.
fn gold() -> Band {
    Band::around(Timeframe::Weekly, d("4094"), d("220.42"), d("0.35"))
}

fn watching() -> Watch {
    Watch::over(vec![gold()])
}

// ── The rule ──

#[test]
fn arriving_in_a_band_fires_once() {
    let mut watch = watching();

    watch.arrive(d("4200"));
    assert_eq!(watch.arrive(d("4100")).len(), 1, "it arrived");
}

// This is the whole reason the type exists. Prices come about once a second
// and barely move. One visit to a level must not become twenty alerts.
#[test]
fn sitting_in_a_band_fires_nothing_more() {
    let mut watch = watching();

    watch.arrive(d("4200"));
    watch.arrive(d("4100"));

    for price in ["4100.5", "4099", "4101", "4094", "4130"] {
        assert!(
            watch.arrive(d(price)).is_empty(),
            "{price} was already inside"
        );
    }
}

#[test]
fn leaving_and_coming_back_fires_again() {
    let mut watch = watching();

    watch.arrive(d("4200"));
    watch.arrive(d("4100"));
    watch.arrive(d("4300"));

    assert_eq!(watch.arrive(d("4100")).len(), 1, "a second visit");
}

// Price hovering on the edge. 4132.57 is the top, so these cross it three
// times — and describe one moment where nothing happened.
#[test]
fn hovering_on_the_edge_does_not_fire_over_and_over() {
    let mut watch = watching();

    watch.arrive(d("4200"));
    assert_eq!(
        watch.arrive(d("4132")).len(),
        1,
        "the first crossing counts"
    );

    let mut fired = 0;
    for price in ["4133", "4132.4", "4133", "4132.5", "4133"] {
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

    assert!(watch.arrive(d("4100")).is_empty(), "started inside");
    assert_eq!(watch.resting_in().len(), 1, "but it knows where price is");
}

#[test]
fn starting_inside_still_fires_on_a_real_arrival_later() {
    let mut watch = watching();

    watch.arrive(d("4100"));
    watch.arrive(d("4300"));

    assert_eq!(watch.arrive(d("4100")).len(), 1);
}

// ── Several bands ──

#[test]
fn only_the_band_price_entered_fires() {
    let daily = Band::around(Timeframe::Daily, d("2984"), d("70.36"), d("0.46"));
    let mut watch = Watch::over(vec![gold(), daily]);

    watch.arrive(d("3500"));

    let entered = watch.arrive(d("4100"));
    assert_eq!(entered.len(), 1);
    assert_eq!(entered[0].timeframe, Timeframe::Weekly);
}

#[test]
fn a_price_in_no_band_fires_nothing() {
    let mut watch = watching();

    watch.arrive(d("4200"));
    assert!(watch.arrive(d("4250")).is_empty());
}
