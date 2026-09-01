//! The first price, and everything before it.
//!
//! **Nothing is resting at a zone until a price has been fed in.** Obvious
//! written down, and it cost the report that says where price already stands:
//! the watcher asked what was resting BEFORE handing over the first price,
//! found nothing, and marked the session as reported.

use super::super::Watch;
use super::support::{d, gold, share};

fn watching() -> Watch {
    Watch::over(vec![gold()], share())
}

#[test]
fn nothing_is_resting_at_a_zone_before_the_first_price() {
    let watch = watching();

    assert!(watch.last_price().is_none(), "no price has arrived");
    assert!(
        watch.resting_at().is_empty(),
        "so nothing can be resting anywhere"
    );
}

#[test]
fn the_first_price_makes_price_rest() {
    let mut watch = watching();

    watch.saw(d("4100"));

    assert_eq!(watch.resting_at().len(), 1, "it IS resting there now");
    assert_eq!(watch.last_price(), Some(d("4100")));
}

/// **A price says nothing on its own, since 31 August 2026.**
///
/// It used to be rung 1 — *price is coming up on your zone*. His call: *"when
/// price is getting to a level we do not want an alert, so remove the card."*
///
/// So the only thing a price does now is get remembered. Watching it walk
/// into a zone, sit in it and leave again sends nothing at any point.
#[test]
fn walking_into_a_zone_and_out_again_sends_nothing() {
    let mut watch = watching();

    for price in ["4300", "4136", "4100", "4060", "4132", "4300"] {
        watch.saw(d(price));
    }

    assert_eq!(watch.last_price(), Some(d("4300")));
    assert!(
        watch.resting_at().is_empty(),
        "price left, and said nothing"
    );
}

/// **Resting is measured fresh, not remembered.**
///
/// There used to be a "deepest price got this visit" per band, kept so that
/// one visit fired one alert. With no alert there is nothing to count, and a
/// report sent once a session wants to know where price is now — not where it
/// has been since it last properly left.
#[test]
fn resting_follows_the_latest_price_both_ways() {
    let mut watch = watching();

    watch.saw(d("4100"));
    assert_eq!(watch.resting_at().len(), 1, "in the zone");

    watch.saw(d("4300"));
    assert!(watch.resting_at().is_empty(), "and out of it again");

    watch.saw(d("4100"));
    assert_eq!(watch.resting_at().len(), 1, "and back, with no fuss");
}
