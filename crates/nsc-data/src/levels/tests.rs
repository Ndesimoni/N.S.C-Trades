use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

use chrono::{DateTime, Utc};
use nsc_core::level::Origin;
use nsc_core::price::Price;
use nsc_core::timeframe::Timeframe;
use rust_decimal::Decimal;

use nsc_core::price::PriceDistance;

use super::{Thickness, read_levels};
use crate::error::DataError;

/// A normal weekly candle of 250 and a normal daily one of 95 — roughly gold.
struct Gold;

impl Thickness for Gold {
    fn for_timeframe(&self, timeframe: Timeframe) -> Option<PriceDistance> {
        let points = match timeframe {
            Timeframe::W1 => 250.0 * 0.35,
            Timeframe::D1 => 95.0 * 0.60,
            _ => return None,
        };
        Decimal::from_f64_retain(points).map(PriceDistance::new)
    }
}

/// Writes a levels file to a scratch path and reads it back.
///
/// The counter matters: tests run in parallel, and without it every test
/// writes to the same file and they read each other's.
fn read(body: &str) -> Result<Vec<nsc_core::level::Level>, DataError> {
    static NEXT: AtomicUsize = AtomicUsize::new(0);

    let mut path = std::env::temp_dir();
    path.push(format!(
        "nsc_levels_{}_{}.toml",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));

    let mut file = std::fs::File::create(&path).expect("scratch file");
    file.write_all(body.as_bytes()).expect("write");
    drop(file);

    let out = read_levels(&path, &Gold);
    let _ = std::fs::remove_file(&path);
    out
}

fn at(text: &str) -> DateTime<Utc> {
    text.parse::<DateTime<Utc>>().expect("valid timestamp")
}

const GOLD: &str = r#"
[[level]]
timeframe = "W1"
price     = 4000
from      = 2026-08-14
note      = "an 866-point crash stopped dead here"

[[level]]
timeframe = "D1"
price     = 4280
from      = 2026-08-01
"#;

#[test]
fn a_levels_file_is_read_as_written() {
    let levels = read(GOLD).expect("valid file");

    assert_eq!(levels.len(), 2);

    assert_eq!(levels[0].timeframe(), Timeframe::W1);

    // 250 x 0.35 = 87.5 thick, so 43.75 either side of 4000.
    assert_eq!(levels[0].centre(), Price::new(Decimal::from(4000)));
    assert_eq!(
        levels[0].band().thickness(),
        PriceDistance::new(Decimal::from_f64_retain(87.5).expect("fits"))
    );

    assert_eq!(levels[1].timeframe(), Timeframe::D1);
    assert_eq!(levels[1].centre(), Price::new(Decimal::from(4280)));
}

// He drew it because a big move ended there. That has no count, and inventing
// one would poison every later comparison against the finder's levels.
#[test]
fn a_hand_drawn_level_has_no_touch_count() {
    let levels = read(GOLD).expect("valid file");

    assert_eq!(levels[0].origin(), Origin::DrawnByHand);
    assert_eq!(levels[0].touches(), None);
    assert_eq!(levels[0].first_touch(), None);
    assert_eq!(levels[0].last_touch(), None);
}

// ── The rule that keeps a backtest honest ──

#[test]
fn a_level_does_not_exist_before_the_day_it_was_drawn() {
    let levels = read(GOLD).expect("valid file");
    let weekly = levels[0];

    assert!(!weekly.is_known_at(at("2026-08-13T23:59:00Z")));
    assert!(weekly.is_known_at(at("2026-08-14T00:00:00Z")));
    assert!(weekly.is_known_at(at("2026-12-01T00:00:00Z")));
}

#[test]
fn two_levels_drawn_on_different_days_start_on_different_days() {
    let levels = read(GOLD).expect("valid file");

    // The daily one was drawn a fortnight earlier, so it is live while the
    // weekly still is not.
    assert!(levels[1].is_known_at(at("2026-08-05T00:00:00Z")));
    assert!(!levels[0].is_known_at(at("2026-08-05T00:00:00Z")));
}

// ── Refusing a broken file ──

#[test]
fn an_unknown_timeframe_is_refused() {
    let out = read(
        r#"
[[level]]
timeframe = "M3"
price     = 100
from      = 2026-08-14
"#,
    );

    assert!(matches!(out, Err(DataError::BadLevelsFile { .. })));
}

// The thickness comes from a normal candle on that timeframe. Ask for one the
// history cannot measure and it says so rather than guessing a width.
#[test]
fn a_timeframe_with_no_thickness_is_refused() {
    let out = read(
        r#"
[[level]]
timeframe = "H1"
price     = 4000
from      = 2026-08-14
"#,
    );

    assert!(matches!(out, Err(DataError::BadLevelsFile { .. })));
}

// A mistyped field is a line drawn in the wrong place, and every decision made
// against it is wrong with nothing to show that it happened.
#[test]
fn a_misspelled_field_is_refused_rather_than_ignored() {
    let out = read(
        r#"
[[level]]
timeframe = "W1"
pirce     = 4000
from      = 2026-08-14
"#,
    );

    assert!(matches!(out, Err(DataError::BadLevelsFile { .. })));
}

#[test]
fn an_empty_file_is_fine() {
    let levels = read("").expect("an empty file is not an error");

    assert!(levels.is_empty());
}
