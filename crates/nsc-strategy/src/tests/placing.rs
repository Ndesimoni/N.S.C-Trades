//! The test that decides everything: is the shape at the level?

use super::support::{band, d, rules};
use crate::place::{Placing, nearest, where_it_sits};
use nsc_core::levels::{Band, Timeframe};

#[test]
fn inside_the_band_is_inside() {
    assert_eq!(where_it_sits(d("4500"), &band(), &rules()), Placing::Inside);
    assert_eq!(where_it_sits(d("4549"), &band(), &rules()), Placing::Inside);
    assert_eq!(where_it_sits(d("4451"), &band(), &rules()), Placing::Inside);
}

/// **There is no depth rule.** He was asked whether the pin had to touch and
/// said it need not, and that touching was no problem either — so how far in
/// it sits is not a question the code asks.
#[test]
fn how_deep_inside_is_not_a_question() {
    let edge = where_it_sits(d("4549.99"), &band(), &rules());
    let middle = where_it_sits(d("4500"), &band(), &rules());

    assert_eq!(edge, middle);
}

#[test]
fn just_outside_counts_on_both_sides() {
    // The band is 100 thick, so half a band is 50.
    assert_eq!(
        where_it_sits(d("4560"), &band(), &rules()),
        Placing::JustAbove
    );
    assert_eq!(
        where_it_sits(d("4440"), &band(), &rules()),
        Placing::JustBelow
    );
}

/// The boundary either side. **An off-by-one here is invisible** — both
/// answers read as reasonable, so only a test says which one it gives.
#[test]
fn half_a_band_out_still_counts_and_a_hair_further_does_not() {
    assert_eq!(
        where_it_sits(d("4600"), &band(), &rules()),
        Placing::JustAbove,
        "exactly half a band above the top edge is still at the level"
    );

    assert_eq!(
        where_it_sits(d("4600.01"), &band(), &rules()),
        Placing::Away
    );

    assert_eq!(
        where_it_sits(d("4400"), &band(), &rules()),
        Placing::JustBelow
    );
    assert_eq!(
        where_it_sits(d("4399.99"), &band(), &rules()),
        Placing::Away
    );
}

#[test]
fn far_away_counts_as_nothing() {
    assert_eq!(where_it_sits(d("5000"), &band(), &rules()), Placing::Away);
    assert!(!Placing::Away.counts());
    assert!(Placing::Inside.counts());
}

/// **Reach is a share of the band, never a distance.** A thin band reaches a
/// short way and a thick one reaches further, which is what lets the same
/// setting work on gold and on the euro.
#[test]
fn a_thinner_band_reaches_a_shorter_way() {
    let thin = Band {
        timeframe: Timeframe::Daily,
        price: d("4500"),
        top: d("4510"),
        bottom: d("4490"),
    };

    // Twenty thick, so half a band is ten.
    assert_eq!(
        where_it_sits(d("4520"), &thin, &rules()),
        Placing::JustAbove
    );
    assert_eq!(where_it_sits(d("4521"), &thin, &rules()), Placing::Away);

    // The same price against the wide band is comfortably inside its reach.
    assert_eq!(where_it_sits(d("4521"), &band(), &rules()), Placing::Inside);
}

/// **The nearest zone wins, not the first in the list.** Which zone a shape
/// printed at is the whole content of the signal, and reading them in file
/// order would report whichever happened to be loaded first.
#[test]
fn the_nearest_zone_wins() {
    let far = Band {
        timeframe: Timeframe::Daily,
        price: d("4560"),
        top: d("4610"),
        bottom: d("4510"),
    };

    let bands = vec![far, band()];

    let (found, _) = nearest(d("4505"), &bands, &rules()).expect("both are in reach");

    assert_eq!(found.price, d("4500"), "4505 is nearer 4500 than 4560");
}

#[test]
fn no_zone_in_reach_is_no_signal() {
    assert!(nearest(d("9000"), &[band()], &rules()).is_none());
    assert!(nearest(d("4500"), &[], &rules()).is_none());
}
