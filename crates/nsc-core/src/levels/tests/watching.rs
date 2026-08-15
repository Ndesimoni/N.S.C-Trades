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
#[test]
fn sitting_at_a_band_fires_nothing_more() {
    let mut watch = watching();

    watch.arrive(d("4300"));
    watch.arrive(d("4132.60"));

    for price in ["4132.55", "4120", "4100", "4060", "4135"] {
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

    let mut fired = 0;
    for price in ["4134", "4132.50", "4136", "4133", "4139"] {
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
