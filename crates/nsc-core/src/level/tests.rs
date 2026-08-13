use chrono::{DateTime, TimeDelta, Utc};
use rust_decimal::Decimal;

use super::*;
use crate::error::CoreError;
use crate::price::{Price, PriceDistance};
use crate::timeframe::Timeframe;

fn price(n: i64) -> Price {
    Price::new(Decimal::from(n))
}

fn distance(n: i64) -> PriceDistance {
    PriceDistance::new(Decimal::from(n))
}

fn at(day: i64) -> DateTime<Utc> {
    let start = "2026-08-10T00:00:00Z"
        .parse::<DateTime<Utc>>()
        .expect("valid timestamp");

    start + TimeDelta::try_days(day).expect("in range")
}

/// A band from 100 to 110, on the daily, touched three times.
fn level() -> Level {
    let band = Band::new(price(100), price(110)).expect("valid band");

    Level::new(band, Timeframe::D1, 3, at(1), at(5), at(6)).expect("valid level")
}

// ── The band ──

#[test]
fn the_edges_of_a_band_count_as_inside_it() {
    let band = Band::new(price(100), price(110)).expect("valid band");

    assert!(band.contains(price(100)));
    assert!(band.contains(price(110)));
    assert!(band.contains(price(105)));
    assert!(!band.contains(price(111)));
}

#[test]
fn a_band_built_around_a_price_has_exactly_the_thickness_asked_for() {
    let band = Band::around(price(100), distance(10)).expect("valid band");

    assert_eq!(band.low(), price(95));
    assert_eq!(band.high(), price(105));
    assert_eq!(band.thickness(), distance(10));
    assert_eq!(band.centre(), price(100));
}

#[test]
fn an_upside_down_band_is_refused() {
    let refused = Band::new(price(110), price(100));

    assert!(matches!(refused, Err(CoreError::ImpossibleLevel { .. })));
}

#[test]
fn distance_is_measured_to_the_nearest_edge_and_keeps_its_side() {
    let band = Band::new(price(100), price(110)).expect("valid band");

    // Above the band.
    assert_eq!(band.distance_to(price(115)), distance(5));
    // Below it.
    assert_eq!(band.distance_to(price(90)), distance(-10));
    // A wick that reached into it has arrived, so it is zero away.
    assert_eq!(band.distance_to(price(104)), distance(0));
}

// ── The level ──

#[test]
fn a_level_reports_the_facts_it_was_built_from() {
    let level = level();

    assert_eq!(level.touches(), Some(3));
    assert_eq!(level.timeframe(), Timeframe::D1);
    assert_eq!(level.centre(), price(105));
    assert!(level.contains(price(102)));
}

#[test]
fn a_level_with_no_touches_is_refused() {
    let band = Band::new(price(100), price(110)).expect("valid band");
    let refused = Level::new(band, Timeframe::D1, 0, at(1), at(5), at(6));

    assert!(matches!(refused, Err(CoreError::ImpossibleLevel { .. })));
}

#[test]
fn a_last_touch_before_the_first_one_is_refused() {
    let band = Band::new(price(100), price(110)).expect("valid band");
    let refused = Level::new(band, Timeframe::D1, 3, at(5), at(1), at(6));

    assert!(matches!(refused, Err(CoreError::ImpossibleLevel { .. })));
}

// The one that catches a lookahead bug. A level cannot be known on the same
// candle as its last touch — that touch is a swing, and a swing takes a few
// candles to confirm.
#[test]
fn a_level_known_on_the_candle_of_its_last_touch_is_refused() {
    let band = Band::new(price(100), price(110)).expect("valid band");
    let refused = Level::new(band, Timeframe::D1, 3, at(1), at(5), at(5));

    assert!(matches!(refused, Err(CoreError::LevelKnownTooEarly { .. })));
}

#[test]
fn a_level_cannot_be_used_before_it_confirmed() {
    let level = level();

    assert!(!level.is_known_at(at(5)));
    assert!(level.is_known_at(at(6)));
    assert!(level.is_known_at(at(7)));
}

// ── Being covered by a bigger timeframe ──

#[test]
fn a_level_is_drawn_unless_something_bigger_covers_it() {
    let daily = level();

    assert!(daily.is_drawn());
    assert_eq!(daily.absorbed_by(), None);

    let covered = daily.covered_by(Timeframe::W1).expect("weekly is bigger");

    assert!(!covered.is_drawn());
    assert_eq!(covered.absorbed_by(), Some(Timeframe::W1));
}

// Being covered is a drawing rule. Everything that made the level worth
// having is still on it, because two timeframes turning at one price is the
// confluence you actually want.
#[test]
fn a_covered_level_keeps_everything_it_knew() {
    let daily = level();
    let covered = daily.covered_by(Timeframe::W1).expect("weekly is bigger");

    assert_eq!(covered.band(), daily.band());
    assert_eq!(covered.touches(), daily.touches());
    assert_eq!(
        covered.timeframe(),
        Timeframe::D1,
        "it is still a daily level"
    );
    assert_eq!(covered.confirmed_at(), daily.confirmed_at());
}

// The rule is that the bigger timeframe wins. This is where that is enforced
// rather than remembered.
#[test]
fn a_smaller_timeframe_cannot_swallow_a_bigger_one() {
    let daily = level();

    assert!(matches!(
        daily.covered_by(Timeframe::H4),
        Err(CoreError::ImpossibleLevel { .. })
    ));

    assert!(matches!(
        daily.covered_by(Timeframe::D1),
        Err(CoreError::ImpossibleLevel { .. })
    ));
}
