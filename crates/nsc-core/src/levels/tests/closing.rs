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

// ── which closes earn a card ───────────────────────────────────────────────

/// **Only a candle that finished outside the band.** His call, 26 August 2026.
///
/// A candle that settled inside the zone is the most common of the three and
/// the one that says least — the approach alert already told him price was
/// there, and a card as well is the same news twice.
#[test]
fn only_a_close_outside_the_band_counts_as_a_break() {
    assert!(AtZone::ClosedAbove.left_the_band());
    assert!(AtZone::ClosedBelow.left_the_band());

    assert!(
        !AtZone::ClosedInside.left_the_band(),
        "price sitting undecided in the zone is not a break"
    );
    assert!(!AtZone::Missed.left_the_band());
}

/// **The rejection survives, and that is the point of the whole rule.**
///
/// A candle that wicked down into the zone and closed back above it finishes
/// as `ClosedAbove` — so "a wick counts" is untouched by dropping the
/// closed-inside card.
#[test]
fn a_wick_in_and_a_close_back_out_still_earns_a_card() {
    let band = Band {
        timeframe: Timeframe::Weekly,
        price: d("4500"),
        top: d("4550"),
        bottom: d("4450"),
    };

    // Opened above, wicked down into the band, closed back above it.
    let rejected = Bar {
        datetime: "2026-08-26 00:00:00".into(),
        open: d("4600"),
        high: d("4610"),
        low: d("4470"),
        close: d("4590"),
    };

    let did = what_it_did(&band, &rejected);

    assert_eq!(did, AtZone::ClosedAbove);
    assert!(
        did.left_the_band(),
        "the rejection he waits for must not be dropped with the quiet ones"
    );
}

/// The one that is dropped: in and stayed in.
#[test]
fn a_candle_that_settled_in_the_zone_is_the_one_dropped() {
    let band = Band {
        timeframe: Timeframe::Weekly,
        price: d("4500"),
        top: d("4550"),
        bottom: d("4450"),
    };

    let settled = Bar {
        datetime: "2026-08-26 00:00:00".into(),
        open: d("4560"),
        high: d("4565"),
        low: d("4480"),
        close: d("4510"),
    };

    let did = what_it_did(&band, &settled);

    assert_eq!(did, AtZone::ClosedInside);
    assert!(
        did.worth_saying(),
        "it still had something to do with the zone"
    );
    assert!(!did.left_the_band(), "but it is not a break, so no card");
}

// ── Which timeframes say anything about a close ───────────────────────────

use super::super::{ClosesOn, Thickness};

fn settings(close_cards: ClosesOn) -> Thickness {
    Thickness {
        weekly: d("0.35"),
        daily: d("0.46"),
        h4: d("0.55"),
        approach_share: d("0.05"),
        kiss_depth: d("0.25"),
        only_breaks: true,
        close_cards,
    }
}

/// **The daily speaks, and it is the biggest close card the bot sends.**
///
/// Added 31 August 2026 at his word: a daily candle should give a setup, an
/// approach and a close, just as the 4-hour does.
#[test]
fn the_daily_sends_close_cards() {
    assert!(settings(ClosesOn::default()).says_closes_on("1d"));
}

/// **The 1-hour says nothing about a close, and that is his answer.**
///
/// 31 August 2026: *"we don't want those notifications from the one hour. The
/// only notification we want from the one hour should be a setup."*
///
/// The 1-hour is still watched, still fetched and still judged — a candlestick
/// pattern at a zone is the whole reason it is here. What stops is only the
/// card narrating every candle that closed near a level.
#[test]
fn the_one_hour_does_not_send_close_cards() {
    let settings = settings(ClosesOn::default());

    assert!(!settings.says_closes_on("1h"), "his call, 31 August");
    assert!(settings.says_closes_on("4h"), "the 4-hour is unchanged");
}

/// **A timeframe nothing watches answers no**, rather than guessing.
///
/// `closes/fetch.rs` says which are watched. A setting for one that is not
/// would be a setting for something that never happens.
#[test]
fn a_timeframe_nothing_watches_says_no() {
    let settings = settings(ClosesOn::default());

    for never in ["1w", "5m", "15m", "nonsense", ""] {
        assert!(!settings.says_closes_on(never), "{never} is not watched");
    }
}

/// He can turn the 1-hour back on without touching code.
#[test]
fn he_can_have_the_one_hour_back_if_he_wants_it() {
    let settings = settings(ClosesOn {
        h1: true,
        ..Default::default()
    });

    assert!(settings.says_closes_on("1h"));
}

/// **A settings file written before this existed gets his answer**, not the
/// old behaviour — because the old behaviour is the thing he asked to stop.
#[test]
fn a_file_without_the_setting_still_silences_the_one_hour() {
    let text = r#"
weekly = 0.35
daily = 0.46
h4 = 0.55
"#;

    let settings: Thickness = toml::from_str(text).expect("a valid settings file");

    assert!(!settings.says_closes_on("1h"));
    assert!(settings.says_closes_on("4h"));
    assert!(settings.says_closes_on("1d"));
}

/// And typing it in is the only way he will ever change it, so the trip
/// through TOML is the part worth pinning.
#[test]
fn the_setting_can_be_typed_into_the_file() {
    let text = r#"
weekly = 0.35
daily = 0.46
h4 = 0.55

[close_cards]
h1 = true
h4 = false
d1 = false
"#;

    let settings: Thickness = toml::from_str(text).expect("a valid settings file");

    assert!(settings.says_closes_on("1h"));
    assert!(!settings.says_closes_on("4h"));
    assert!(!settings.says_closes_on("1d"));
}
