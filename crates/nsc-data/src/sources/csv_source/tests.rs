//! Does it read what it should, and refuse what it should?

use std::io::Write;
use std::path::PathBuf;

use rust_decimal::Decimal;

use super::read_candles;
use crate::error::DataError;

/// Writes a file to the temp directory and gives back its path.
fn a_file(name: &str, contents: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("nsc_csv_{name}.csv"));
    let mut file = std::fs::File::create(&path).expect("can write a temp file");
    file.write_all(contents.as_bytes()).expect("can write");
    path
}

#[test]
fn a_plain_file_reads() {
    let path = a_file(
        "plain",
        "time,open,high,low,close\n\
         2026-08-10 00:00:00,1.0850,1.0870,1.0840,1.0860\n\
         2026-08-10 00:15:00,1.0860,1.0880,1.0855,1.0875\n",
    );

    let candles = read_candles(&path).expect("reads");

    assert_eq!(candles.len(), 2);
    assert_eq!(candles[0].open().value(), Decimal::new(10850, 4));
    assert_eq!(candles[1].high().value(), Decimal::new(10880, 4));
    assert!(candles[0].is_complete());
}

// The header says which column is which, so the order in the file does not
// matter. Guessing by position is what would make a rearranged file parse
// perfectly and be wrong in every price.
#[test]
fn the_columns_can_be_in_any_order() {
    let path = a_file(
        "shuffled",
        "Close,Low,High,Open,Date,Volume\n\
         1.0860,1.0840,1.0870,1.0850,2026-08-10 00:00:00,123\n",
    );

    let candles = read_candles(&path).expect("reads");

    assert_eq!(candles[0].open().value(), Decimal::new(10850, 4));
    assert_eq!(candles[0].close().value(), Decimal::new(10860, 4));
    assert_eq!(candles[0].volume(), None, "volume is ignored, not stored");
}

#[test]
fn the_usual_timestamp_shapes_all_read() {
    for text in [
        "2026-08-10 21:00:00",
        "2026-08-10T21:00:00",
        "2026.08.10 21:00",
        "2026-08-10T21:00:00Z",
    ] {
        let path = a_file(
            "times",
            &format!("time,open,high,low,close\n{text},1,2,0,1\n"),
        );

        assert_eq!(read_candles(&path).expect("reads").len(), 1, "{text}");
    }
}

// A daily export has no time of day. Midnight is what the file means; where
// the trading day actually starts is applied later, once, by nsc-core.
#[test]
fn a_date_with_no_time_reads_as_midnight() {
    let path = a_file("daily", "date,open,high,low,close\n2026-08-10,1,2,0,1\n");

    let candles = read_candles(&path).expect("reads");

    assert_eq!(
        candles[0].open_time().to_string(),
        "2026-08-10 00:00:00 UTC"
    );
}

// ── What it refuses ──

// Guessing would give a file that parses perfectly and is wrong in every
// price, which is the worst outcome available — nothing downstream could
// notice.
#[test]
fn a_missing_column_is_refused_rather_than_guessed() {
    let path = a_file("no_high", "time,open,low,close\n2026-08-10,1,0,1\n");

    assert!(matches!(
        read_candles(&path),
        Err(DataError::BadHeader { .. })
    ));
}

// A live feed sending one bad candle should be shrugged off. A file is the
// same every time it is read, so a bad row is a broken file.
#[test]
fn a_bad_price_stops_the_read_rather_than_skipping_the_row() {
    let path = a_file(
        "bad_price",
        "time,open,high,low,close\n\
         2026-08-10 00:00:00,1.0850,1.0870,1.0840,1.0860\n\
         2026-08-10 00:15:00,1.0860,oops,1.0855,1.0875\n",
    );

    assert!(matches!(read_candles(&path), Err(DataError::BadRow { .. })));
}

#[test]
fn an_unreadable_timestamp_is_refused() {
    let path = a_file(
        "bad_time",
        "time,open,high,low,close\nlast tuesday,1,2,0,1\n",
    );

    assert!(matches!(
        read_candles(&path),
        Err(DataError::BadTimestamp { .. })
    ));
}

// Some exports come newest first. They are refused rather than reversed,
// because anything cleverer would silently accept a file that is only NEARLY
// sorted — and candles out of order break every swing and level built from
// them with nothing to show it happened.
#[test]
fn a_file_in_the_wrong_order_is_refused() {
    let path = a_file(
        "backwards",
        "time,open,high,low,close\n\
         2026-08-10 00:15:00,1.0860,1.0880,1.0855,1.0875\n\
         2026-08-10 00:00:00,1.0850,1.0870,1.0840,1.0860\n",
    );

    assert!(matches!(read_candles(&path), Err(DataError::Core(_))));
}

// A high below the low is not a candle. nsc-core refuses it and the reason
// travels up unchanged, because it says which row to go and look at.
#[test]
fn an_impossible_candle_is_refused() {
    let path = a_file(
        "impossible",
        "time,open,high,low,close\n2026-08-10 00:00:00,1.0850,1.0800,1.0900,1.0860\n",
    );

    assert!(matches!(read_candles(&path), Err(DataError::Core(_))));
}

#[test]
fn a_file_that_is_not_there_says_so() {
    let missing = std::env::temp_dir().join("nsc_csv_definitely_not_here.csv");

    assert!(matches!(
        read_candles(&missing),
        Err(DataError::CannotRead { .. })
    ));
}
