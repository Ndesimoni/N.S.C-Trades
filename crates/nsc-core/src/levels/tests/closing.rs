//! What a finished candle did at a zone.

use rust_decimal::Decimal;

use super::super::{AtZone, Band, Timeframe, gapped_in, how_deep, what_it_did};
use super::support::d;
use crate::candle::Bar;

/// The gold band he drew at 4094 — 4055.43 to 4132.57, about 77 thick.
fn gold() -> Band {
    Band::around(Timeframe::Weekly, d("4094"), d("220.42"), d("0.35"))
}

fn candle(open: &str, high: &str, low: &str, close: &str) -> Bar {
    serde_json::from_str(&format!(
        r#"{{"datetime":"2026-08-19 13:00:00","open":"{open}","high":"{high}",
            "low":"{low}","close":"{close}"}}"#
    ))
    .expect("a valid candle")
}

// ── A wick counts ──

// THE ONE THAT MATTERS. A candle that only wicked in and closed back out is
// the whole reason to look at closes rather than at price. Treating it as a
// miss throws away the rejection he is waiting for.
#[test]
fn a_wick_into_the_zone_counts_even_though_the_body_closed_outside() {
    // Opened above, dipped to 4120 — inside — and closed back above.
    let bar = candle("4160", "4165", "4120", "4155");

    assert_eq!(what_it_did(&gold(), &bar), AtZone::ClosedAbove);
}

#[test]
fn a_wick_from_below_counts_the_same_way() {
    let bar = candle("4000", "4070", "3995", "4010");

    assert_eq!(what_it_did(&gold(), &bar), AtZone::ClosedBelow);
}

// ── Missing it ──

// One pip clear is still clear. Without this, "near enough" creeps into the
// close rule as well, and the close rule is the one that has to be exact —
// approaching is a heads-up, a close is a fact.
#[test]
fn a_candle_entirely_above_the_zone_is_a_miss() {
    let bar = candle(
        "4200",
        "4220",
        &(gold().top + d("0.01")).to_string(),
        "4210",
    );

    assert_eq!(what_it_did(&gold(), &bar), AtZone::Missed);
}

#[test]
fn a_candle_entirely_below_the_zone_is_a_miss() {
    let bar = candle(
        "4000",
        &(gold().bottom - d("0.01")).to_string(),
        "3990",
        "4010",
    );

    assert_eq!(what_it_did(&gold(), &bar), AtZone::Missed);
}

#[test]
fn a_miss_is_the_only_thing_not_worth_saying() {
    assert!(!AtZone::Missed.worth_saying());
    assert!(AtZone::ClosedInside.worth_saying());
    assert!(AtZone::ClosedAbove.worth_saying());
    assert!(AtZone::ClosedBelow.worth_saying());
}

// ── The edges ──

// A band HOLDS its own edges, so a close sitting exactly on the top is inside.
// Anywhere else this would be a coin toss; here it follows `holds`, and both
// have to agree or price is "at the level" for the alert and "above it" for
// the close.
#[test]
fn a_close_exactly_on_the_top_edge_is_inside() {
    let band = gold();
    let bar = candle("4100", "4140", "4090", &band.top.to_string());

    assert_eq!(what_it_did(&band, &bar), AtZone::ClosedInside);
    assert!(band.holds(band.top), "and it agrees with holds");
}

// A candle whose HIGH is exactly the band's bottom has touched it. One tick
// lower and it has not — which is the whole difference between a rejection
// reported and a rejection missed.
#[test]
fn a_high_that_just_reaches_the_bottom_edge_has_touched() {
    let band = gold();

    let touching = candle("4000", &band.bottom.to_string(), "3990", "4010");
    assert_eq!(what_it_did(&band, &touching), AtZone::ClosedBelow);

    let short = candle(
        "4000",
        &(band.bottom - d("0.01")).to_string(),
        "3990",
        "4010",
    );
    assert_eq!(what_it_did(&band, &short), AtZone::Missed, "a cent short");
}

// ── Closing inside ──

#[test]
fn a_candle_that_ended_in_the_zone_says_so() {
    let bar = candle("4110", "4125", "4100", "4120");

    assert_eq!(what_it_did(&gold(), &bar), AtZone::ClosedInside);
}

// Straight through, top to bottom. It touched, so it is reported — and where
// it closed is what says it was not held.
#[test]
fn a_candle_that_cut_straight_through_still_reports() {
    let bar = candle("4160", "4165", "4000", "4010");

    assert_eq!(what_it_did(&gold(), &bar), AtZone::ClosedBelow);
}

// ── How deep it went ──

#[test]
fn a_candle_across_the_whole_band_is_all_of_it() {
    assert_eq!(
        how_deep(&gold(), &candle("4200", "4210", "4000", "4010")).round_dp(2),
        d("1.00")
    );
}

#[test]
fn a_candle_that_grazed_the_edge_is_barely_any_of_it() {
    // Dipped one point past the top of a band 77 thick.
    let deep = how_deep(&gold(), &candle("4160", "4165", "4131.57", "4155"));

    assert!(deep < d("0.02"), "barely in: {deep}");
    assert!(deep > Decimal::ZERO, "but in");
}

#[test]
fn a_candle_that_missed_went_no_depth_at_all() {
    assert_eq!(
        how_deep(&gold(), &candle("4200", "4220", "4150", "4210")),
        Decimal::ZERO
    );
}

// ── Gapping in ──

// All week the open IS the last close, so this is false and there is no
// "a candle opened in the zone" message. A gap is the only version that
// carries anything he did not already have.
#[test]
fn walking_into_the_zone_is_not_a_gap() {
    let band = gold();
    let before = candle("4160", "4165", "4140", "4150");
    let now = candle("4150", "4155", "4120", "4125");

    assert_eq!(what_it_did(&band, &now), AtZone::ClosedInside);
    assert!(!gapped_in(&band, &before, &now), "it walked in");
}

// Sunday's open, or gold's hour off. Price jumped into his level instead of
// walking there, and that is worth knowing.
#[test]
fn jumping_into_the_zone_over_a_break_is_a_gap() {
    let band = gold();
    let before = candle("4160", "4170", "4150", "4165");
    let now = candle("4120", "4130", "4110", "4118");

    assert!(gapped_in(&band, &before, &now));
}

#[test]
fn opening_outside_is_never_a_gap_in() {
    let band = gold();
    let before = candle("4110", "4130", "4100", "4120");
    let now = candle("4200", "4210", "4190", "4205");

    assert!(!gapped_in(&band, &before, &now));
}
