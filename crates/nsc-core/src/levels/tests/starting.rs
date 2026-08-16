//! The first price, and everything before it.
//!
//! **Nothing is resting at a zone until a price has been fed in.** Obvious
//! written down, and it cost the report that says where price already stands:
//! the watcher asked what was resting BEFORE handing over the first price,
//! found nothing, and marked the session as reported.

use super::super::{Band, Timeframe, Watch};
use super::support::d;

/// The gold band he drew at 4094.
fn gold() -> Band {
    Band::around(Timeframe::Weekly, d("4094"), d("220.42"), d("0.35"))
}

fn watching() -> Watch {
    Watch::over(vec![gold()], d("0.10"))
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
