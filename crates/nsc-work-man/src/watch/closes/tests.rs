//! What a report is remembered by.
//!
//! Nothing here reaches the feed. These pin the KEY — which is the thing that
//! decides whether he hears about a candle at all.

use chrono::{DateTime, Utc};
use nsc_core::candle::Bar;
use nsc_data::source::Interval;
use rust_decimal::Decimal;
use std::str::FromStr;

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
    let mut closes = Closes::new(None, None);

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
    let mut closes = Closes::new(None, None);

    closes.told.insert(
        about("4120", Kind::Closed),
        "2026-08-16 13:00:00".to_string(),
    );

    assert!(!closes.already_said(&about("4120", Kind::Closed), "2026-08-16 14:00:00"));
}

/// **A candle can still be worth two messages** — what it did at the band, and
/// the shape it completed. Remembered under one key, whichever arrived second
/// would be silenced.
///
/// This test used to be about the mid-candle look, which was the other thing a
/// candle got spoken about. That card went on 27 August 2026; rung 3 took over
/// as the second message and the reason for keeping the kinds apart is
/// unchanged.
#[test]
fn a_setup_does_not_silence_the_close_on_the_same_candle() {
    let mut closes = Closes::new(None, None);

    closes.told.insert(
        about("4120", Kind::Setup),
        "2026-08-16 13:00:00".to_string(),
    );

    assert!(!closes.already_said(&about("4120", Kind::Closed), "2026-08-16 13:00:00"));
}

// ── what goes into the record ─────────────────────────────────────────────

use super::look::finished_only;

fn d(text: &str) -> Decimal {
    Decimal::from_str(text).unwrap()
}

fn at(stamp: &str) -> Bar {
    Bar {
        datetime: stamp.into(),
        open: d("1"),
        high: d("2"),
        low: d("0.5"),
        close: d("1.5"),
    }
}

/// **A short candle is finished the moment a later one opens.**
///
/// IBKR ends its forex day at 17:15 New York and prints short candles around
/// the boundary. Measured on 30,000 AUD/USD 4-hour candles: 21,446 ran the
/// full 240 minutes and **7,675 did not**, some as short as 75.
///
/// Judged by a stopwatch, a 75-minute candle is "unfinished" for another 165
/// minutes after it has closed — so the close card arrived up to two and
/// three-quarter hours late, twice a day, on every pair. Nothing errored.
///
/// The candle that opened after it is the proof, and it needs no arithmetic.
#[test]
fn a_short_candle_is_finished_when_the_next_one_opens() {
    // The 20:00 four-hour candle was cut off at 21:15 by the session end, and
    // the 21:15 stub is now running. Newest first, as the feed hands them over.
    let bars = [
        at("2026-08-27 21:15:00"),
        at("2026-08-27 20:00:00"),
        at("2026-08-27 16:00:00"),
    ];

    // Ten minutes into the stub. By the stopwatch the 20:00 candle has another
    // 2h50m to run; in fact it closed 75 minutes ago.
    let now = "2026-08-27T21:25:00Z".parse::<DateTime<Utc>>().unwrap();
    let kept = finished_only(&bars, now, 240);

    assert_eq!(kept.len(), 2, "the 20:00 candle closed when 21:15 opened");
    assert_eq!(kept[0].datetime, "2026-08-27 20:00:00");
}

/// **Only finished candles are kept, and the one still running is not.**
///
/// The feed hands back both in one reply. Storing the running one would put a
/// half-drawn bar in the record, and a backtest reading it months later would
/// treat it as settled — which does not look broken, it looks *better*.
#[test]
fn the_candle_still_running_is_not_kept() {
    // Four 1-hour candles; it is 03:30, so 03:00 is still forming.
    let bars = [
        at("2026-08-27 00:00:00"),
        at("2026-08-27 01:00:00"),
        at("2026-08-27 02:00:00"),
        at("2026-08-27 03:00:00"),
    ];

    // Newest first, as the feed hands them over.
    let bars: Vec<_> = bars.into_iter().rev().collect();

    let now = "2026-08-27T03:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let kept = finished_only(&bars, now, 60);

    assert_eq!(kept.len(), 3, "the 03:00 candle had not closed");
    assert_eq!(kept[0].datetime, "2026-08-27 02:00:00");
}

/// **The whole reply is kept, not just the newest.**
///
/// It costs one statement either way, and it means a bot that was off for a
/// day fills that day in on its next look rather than leaving a hole nothing
/// would ever go back for.
#[test]
fn every_finished_candle_is_kept_not_only_the_last() {
    let bars = [
        at("2026-08-27 00:00:00"),
        at("2026-08-27 01:00:00"),
        at("2026-08-27 02:00:00"),
    ];

    let bars: Vec<_> = bars.into_iter().rev().collect();
    let now = "2026-08-27T09:00:00Z".parse::<DateTime<Utc>>().unwrap();

    assert_eq!(finished_only(&bars, now, 60).len(), 3);
}

/// A stamp that will not read is skipped, **not guessed at**. A candle with no
/// readable time has nowhere to sit on a chart.
#[test]
fn a_candle_with_a_broken_stamp_is_left_out() {
    // Newest first, and the broken one is NOT the newest — so it would sail
    // through on "a later candle exists" if the stamp were not checked.
    let bars = [
        at("2026-08-27 02:00:00"),
        at("not a time"),
        at("2026-08-27 00:00:00"),
    ];

    let now = "2026-08-27T09:00:00Z".parse::<DateTime<Utc>>().unwrap();

    assert_eq!(finished_only(&bars, now, 60).len(), 2);
}
