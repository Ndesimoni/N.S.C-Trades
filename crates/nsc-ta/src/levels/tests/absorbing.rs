//! When a smaller timeframe's level does not get its own line.

use nsc_core::level::{Band, Level, NotDrawn};
use nsc_core::timeframe::Timeframe;

use super::helpers::*;
use crate::config::LevelSettings;
use crate::levels::decide_what_gets_a_line;

/// The settings in ta.toml: a daily must be 1.5 weekly bands clear, and two
/// levels on one timeframe must be 3 bands apart.
fn rules() -> LevelSettings {
    LevelSettings {
        band_atr_multiple: 0.5,
        min_touches: 2,
        max_age_bars: 500,
        absorb_gap_bands: 1.5,
        min_separation_bands: 3.0,
    }
}

/// Same, but nothing is hidden for merely being near.
fn touching_only() -> LevelSettings {
    LevelSettings {
        absorb_gap_bands: 0.0,
        min_separation_bands: 0.0,
        ..rules()
    }
}

fn mark_absorbed(levels: Vec<Level>, settings: &LevelSettings) -> Vec<Level> {
    decide_what_gets_a_line(levels, settings).expect("valid")
}

/// A level with a band from `low` to `high`, on `timeframe`.
fn level(timeframe: Timeframe, low: i64, high: i64) -> Level {
    let band = Band::new(price(low), price(high)).expect("valid band");

    Level::new(band, timeframe, 3, at(1), at(5), at(6)).expect("valid level")
}

/// A level with a touch count of its own.
fn level_with(timeframe: Timeframe, low: i64, high: i64, touches: u32) -> Level {
    let band = Band::new(price(low), price(high)).expect("valid band");

    Level::new(band, timeframe, touches, at(1), at(5), at(6)).expect("valid level")
}

fn drawn(levels: &[Level]) -> Vec<Timeframe> {
    levels
        .iter()
        .filter(|level| level.is_drawn())
        .map(|level| level.timeframe())
        .collect()
}

// ── The rule ──

#[test]
fn a_daily_landing_on_a_weekly_is_not_drawn() {
    let out = mark_absorbed(
        vec![
            level(Timeframe::D1, 4345, 4355),
            level(Timeframe::W1, 4335, 4365),
        ],
        &rules(),
    );

    assert_eq!(drawn(&out), vec![Timeframe::W1], "the bigger one wins");

    let daily = out
        .iter()
        .find(|level| level.timeframe() == Timeframe::D1)
        .expect("still here");

    assert_eq!(daily.absorbed_by(), Some(Timeframe::W1));
}

#[test]
fn a_four_hour_landing_on_a_daily_is_not_drawn() {
    let out = mark_absorbed(
        vec![
            level(Timeframe::H4, 4182, 4188),
            level(Timeframe::D1, 4175, 4195),
        ],
        &rules(),
    );

    assert_eq!(drawn(&out), vec![Timeframe::D1]);
}

// ── Kissing, but not overlapping ──

// The case spotted on the drawing: a daily band just clear of a weekly one,
// not touching it, still reads as one thick line and says nothing new.
#[test]
fn a_level_that_only_kisses_a_bigger_one_still_loses_its_line() {
    // The weekly band is 30 thick, so half a band is 15. This daily sits 5
    // clear of it — close enough to read as the same line.
    let out = mark_absorbed(
        vec![
            level(Timeframe::W1, 4325, 4355),
            level(Timeframe::D1, 4360, 4370),
        ],
        &rules(),
    );

    assert_eq!(
        drawn(&out),
        vec![Timeframe::W1],
        "5 clear of a 30-wide band"
    );
}

#[test]
fn the_same_pair_keeps_both_lines_when_only_touching_counts() {
    let out = mark_absorbed(
        vec![
            level(Timeframe::W1, 4325, 4355),
            level(Timeframe::D1, 4360, 4370),
        ],
        &touching_only(),
    );

    assert_eq!(drawn(&out).len(), 2, "they do not actually overlap");
}

#[test]
fn a_level_far_enough_clear_keeps_its_line() {
    // The weekly band is 30 thick, so it demands 45 of clearance. This daily
    // sits 95 clear — plainly a separate line, and it stays.
    let out = mark_absorbed(
        vec![
            level(Timeframe::W1, 4325, 4355),
            level(Timeframe::D1, 4450, 4460),
        ],
        &rules(),
    );

    assert_eq!(drawn(&out).len(), 2);
}

// The clearance is measured against the BIGGER band, so being near a weekly
// costs more room than being near a daily. Same gap, different answer.
#[test]
fn the_clearance_scales_with_the_bigger_bands_thickness() {
    let under_a_weekly = mark_absorbed(
        vec![
            level(Timeframe::W1, 4325, 4355), // 30 thick, so 45 of clearance
            level(Timeframe::D1, 4375, 4385), // 20 clear
        ],
        &rules(),
    );

    let under_a_daily = mark_absorbed(
        vec![
            level(Timeframe::D1, 4347, 4353), // 6 thick, so 9 of clearance
            level(Timeframe::H4, 4373, 4379), // the same 20 clear
        ],
        &rules(),
    );

    assert_eq!(
        drawn(&under_a_weekly).len(),
        1,
        "45 of clearance swallows 20"
    );
    assert_eq!(
        drawn(&under_a_daily).len(),
        2,
        "only 9 of clearance, so 20 is clear"
    );
}

// ── The worked example ──

#[test]
fn the_worked_example_draws_three_lines() {
    let out = mark_absorbed(
        vec![
            level(Timeframe::W1, 4325, 4355), // 4340
            level(Timeframe::D1, 4340, 4350), // on the weekly
            level(Timeframe::D1, 4175, 4185), // on its own
            level(Timeframe::H4, 4339, 4345), // on the weekly
            level(Timeframe::H4, 4182, 4188), // on the daily
            level(Timeframe::H4, 3977, 3983), // on its own
        ],
        &rules(),
    );

    assert_eq!(out.len(), 6, "nothing is thrown away");
    assert_eq!(
        drawn(&out),
        vec![Timeframe::H4, Timeframe::D1, Timeframe::W1],
        "lowest price first: the lone 4-hour, the lone daily, the weekly"
    );
}

// ── The cascade ──

// If a daily is hidden behind a weekly, it cannot then hide a 4-hour level of
// its own. Otherwise that price ends up with no line at all.
#[test]
fn a_hidden_level_cannot_hide_another() {
    let out = mark_absorbed(
        vec![
            level(Timeframe::W1, 4200, 4220),
            level(Timeframe::D1, 4218, 4260), // overlaps the weekly, so hidden
            level(Timeframe::H4, 4300, 4306), // 80 clear of the weekly
        ],
        &rules(),
    );

    let four_hour = out
        .iter()
        .find(|level| level.timeframe() == Timeframe::H4)
        .expect("still here");

    assert!(
        four_hour.is_drawn(),
        "no bigger DRAWN line is near it, so it keeps its own"
    );
}

// ── Order does not matter ──

#[test]
fn the_answer_does_not_depend_on_the_order_they_arrive_in() {
    let forwards = vec![
        level(Timeframe::W1, 4330, 4350),
        level(Timeframe::D1, 4335, 4345),
        level(Timeframe::H4, 4338, 4342),
    ];

    let mut backwards = forwards.clone();
    backwards.reverse();

    let a = mark_absorbed(forwards, &rules());
    let b = mark_absorbed(backwards, &rules());

    assert_eq!(drawn(&a), drawn(&b));
    assert_eq!(drawn(&a), vec![Timeframe::W1]);
}

// ── Crowding: one timeframe does not stack lines on itself ──

// The consolidation case. Price chopped around one area and turned a dozen
// times, so the finder saw a level at every turn. One line goes on the chart.
#[test]
fn two_weeklies_too_close_leave_one_line() {
    let out = mark_absorbed(
        vec![
            level_with(Timeframe::W1, 4000, 4125, 5),
            level_with(Timeframe::W1, 4200, 4325, 3),
        ],
        &rules(),
    );

    assert_eq!(drawn(&out).len(), 1, "one line for one area");
}

#[test]
fn the_most_touched_one_keeps_the_line() {
    let out = mark_absorbed(
        vec![
            level_with(Timeframe::W1, 4000, 4125, 2),
            level_with(Timeframe::W1, 4200, 4325, 6),
        ],
        &rules(),
    );

    let kept: Vec<_> = out.iter().filter(|l| l.is_drawn()).collect();

    assert_eq!(kept.len(), 1);
    assert_eq!(
        kept[0].touches(),
        Some(6),
        "the price it actually turned at"
    );
    assert_eq!(
        out.iter()
            .find(|l| l.touches() == Some(2))
            .map(|l| l.not_drawn()),
        Some(Some(NotDrawn::CrowdedOut))
    );
}

#[test]
fn two_weeklies_far_enough_apart_both_get_lines() {
    // A weekly band is 125 thick here, so they need 375 between them.
    let out = mark_absorbed(
        vec![
            level_with(Timeframe::W1, 4000, 4125, 4),
            level_with(Timeframe::W1, 4600, 4725, 3),
        ],
        &rules(),
    );

    assert_eq!(drawn(&out).len(), 2);
}

// The demotion, and it needs no rule of its own. A weekly that lost its line
// cannot cover anything, so the daily at that price draws itself in blue.
#[test]
fn a_crowded_out_weekly_stops_hiding_the_daily_beneath_it() {
    let out = mark_absorbed(
        vec![
            level_with(Timeframe::W1, 4000, 4125, 5), // keeps the line
            level_with(Timeframe::W1, 4200, 4325, 3), // crowded out
            level_with(Timeframe::D1, 4400, 4440, 3), // clear of the first
        ],
        &rules(),
    );

    let daily = out
        .iter()
        .find(|l| l.timeframe() == Timeframe::D1)
        .expect("still here");

    assert!(daily.is_drawn(), "the weekly above it was never drawn");
    assert_eq!(drawn(&out), vec![Timeframe::W1, Timeframe::D1]);
}
