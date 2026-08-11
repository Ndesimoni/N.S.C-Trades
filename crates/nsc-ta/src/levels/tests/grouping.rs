//! Where does the band end up when you slide it?
//!
//! These build the swing points by hand and hand the band its thickness, so
//! there is no ATR in the way and every answer can be worked out on paper.

use nsc_core::swing::SwingKind::{High, Low};

use super::helpers::*;
use crate::levels::grouping::best_group;

#[test]
fn nothing_to_group_gives_nothing() {
    assert!(best_group(&[], distance(5)).is_none());
}

#[test]
fn swings_within_one_thickness_are_caught_together() {
    let swings = [
        swing(High, 10, 100),
        swing(High, 20, 103),
        swing(High, 30, 105),
    ];

    let group = best_group(&swings, distance(5)).expect("a group");

    assert_eq!(group.swings.len(), 3);
    assert_eq!(group.spread, distance(5));
}

// The point of the whole design: a price that capped a rally and later held a
// fall is ONE level tested twice, not two that happen to share a price.
#[test]
fn a_high_and_a_low_at_the_same_price_are_one_group() {
    let swings = [swing(High, 10, 100), swing(Low, 40, 101)];

    let group = best_group(&swings, distance(5)).expect("a group");

    assert_eq!(group.swings.len(), 2);
}

#[test]
fn a_swing_further_away_than_the_thickness_is_left_out() {
    let swings = [
        swing(High, 10, 100),
        swing(High, 20, 102),
        swing(High, 30, 120),
    ];

    let group = best_group(&swings, distance(5)).expect("a group");

    assert_eq!(group.swings.len(), 2);
    assert!(!group.swings.iter().any(|s| s.price() == price(120)));
}

// The band slides; it does not stretch. Four swings sit inside 5 of each
// other at the top, three at the bottom. The band goes where the four are,
// and does not grow to try to reach all seven.
#[test]
fn the_band_goes_where_the_touches_are() {
    let swings = [
        swing(High, 10, 100),
        swing(High, 20, 101),
        swing(High, 30, 102),
        swing(High, 40, 130),
        swing(High, 50, 131),
        swing(High, 60, 132),
        swing(High, 70, 133),
    ];

    let group = best_group(&swings, distance(5)).expect("a group");

    assert_eq!(group.swings.len(), 4);
    assert_eq!(group.lowest.price(), price(130));
}

// Two positions catch three swings each. The tighter one wins, because it
// describes a price the market actually kept turning at rather than a loose
// scatter.
#[test]
fn the_tighter_group_wins_a_tie() {
    let swings = [
        swing(High, 10, 100),
        swing(High, 20, 102),
        swing(High, 30, 105),
        swing(High, 40, 200),
        swing(High, 50, 201),
        swing(High, 60, 202),
    ];

    let group = best_group(&swings, distance(5)).expect("a group");

    assert_eq!(group.swings.len(), 3);
    assert_eq!(group.spread, distance(2));
    assert_eq!(group.lowest.price(), price(200));
}

// Same swings, arrived in a different order. A backtest you cannot repeat is
// worth nothing, so the answer must not depend on the order they came in.
#[test]
fn the_order_swings_arrive_in_changes_nothing() {
    let forwards = [
        swing(High, 10, 100),
        swing(High, 20, 102),
        swing(High, 30, 104),
    ];
    let backwards = [
        swing(High, 30, 104),
        swing(High, 20, 102),
        swing(High, 10, 100),
    ];

    let one = best_group(&forwards, distance(5)).expect("a group");
    let other = best_group(&backwards, distance(5)).expect("a group");

    assert_eq!(one.swings, other.swings);
    assert_eq!(one.spread, other.spread);
}
