//! What a report is remembered by.
//!
//! Nothing here reaches the feed. These pin the KEY — which is the thing that
//! decides whether he hears about a candle at all.

use nsc_data::source::Interval;

use super::look::Closes;
use super::said::{Kind, Said};

fn about(band: &str, kind: Kind) -> Said {
    Said {
        symbol: "XAU/USD".to_string(),
        interval: Interval::H1,
        kind,
        band: band.to_string(),
    }
}

/// **The one that was broken.**
///
/// Price sits at 4,120, the 13:00 candle closes, and that gets reported. Price
/// then runs to 4,135 at half past. The bot had already marked the 13:00
/// candle as done, so it never said what that candle did at 4,135 — he waited
/// a full hour for news the bot was holding.
#[test]
fn a_second_zone_on_the_same_candle_is_still_owed_a_report() {
    let mut closes = Closes::new();

    closes.told.insert(
        about("4120", Kind::Closed),
        "2026-08-16 13:00:00".to_string(),
    );

    assert!(
        closes.already_said(&about("4120", Kind::Closed), "2026-08-16 13:00:00"),
        "the zone it reported is remembered"
    );

    assert!(
        !closes.already_said(&about("4135", Kind::Closed), "2026-08-16 13:00:00"),
        "but the other zone on that same candle is not"
    );
}

/// The next candle is a new candle, on a zone already reported.
#[test]
fn the_next_candle_at_the_same_zone_reports_again() {
    let mut closes = Closes::new();

    closes.told.insert(
        about("4120", Kind::Closed),
        "2026-08-16 13:00:00".to_string(),
    );

    assert!(!closes.already_said(&about("4120", Kind::Closed), "2026-08-16 14:00:00"));
}

/// A candle is spoken about twice — part-way through, then when it finishes.
/// Remembered together, the twenty-minute look would silence the close.
#[test]
fn the_look_does_not_silence_the_close_that_follows_it() {
    let mut closes = Closes::new();

    closes.told.insert(
        about("4120", Kind::SoFar),
        "2026-08-16 13:00:00".to_string(),
    );

    assert!(!closes.already_said(&about("4120", Kind::Closed), "2026-08-16 13:00:00"));
}
