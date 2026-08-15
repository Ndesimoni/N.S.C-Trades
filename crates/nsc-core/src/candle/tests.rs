use chrono::{DateTime, Utc};

use super::Bar;

/// A candle opening at 15:00 on 14 August 2026.
fn at(hour: u32) -> String {
    format!("2026-08-14 {hour:02}:00:00")
}

fn clock(text: &str) -> DateTime<Utc> {
    text.parse::<DateTime<Utc>>().expect("valid timestamp")
}

/// One hourly candle, opening at `hour`.
fn candle(hour: u32) -> Bar {
    serde_json::from_str(&format!(
        r#"{{"datetime":"{}","open":"4377.70","high":"4383.10","low":"4372.54","close":"4373.35"}}"#,
        at(hour)
    ))
    .expect("valid candle")
}

// ── The one question that matters ──

// The 17:00 candle runs until 18:00. At 18:19 it has finished.
#[test]
fn a_candle_whose_hour_is_over_has_finished() {
    let bar = candle(17);

    assert!(
        bar.finished_by(clock("2026-08-14T18:19:00Z"), 60)
            .expect("valid")
    );
}

// The 18:00 candle runs until 19:00. At 18:19 it has ten minutes to run — its
// high is not its high and its close is not its close.
#[test]
fn a_candle_still_running_has_not_finished() {
    let bar = candle(18);

    assert!(
        !bar.finished_by(clock("2026-08-14T18:19:00Z"), 60)
            .expect("valid")
    );
}

// The boundary, exactly. The hour ending at 18:00 is finished at 18:00 and not
// a second before. One second either side of this line is the difference
// between a real backtest and a flattering one.
#[test]
fn a_candle_finishes_the_instant_its_hour_is_up_and_not_before() {
    let bar = candle(17);

    assert!(
        !bar.finished_by(clock("2026-08-14T17:59:59Z"), 60)
            .expect("valid"),
        "one second early"
    );
    assert!(
        bar.finished_by(clock("2026-08-14T18:00:00Z"), 60)
            .expect("valid"),
        "on the dot"
    );
}

// This is why the rule cannot be "skip the first one in the list".
//
// Ask Twelve Data at 18:00:02 and the newest candle is either the 18:00 one,
// if a price has already landed, or the 17:00 one, if none has. In the first
// case the finished candle is second in the list; in the second it is first.
//
// Position is right most of the time. The clock is right always.
#[test]
fn which_candle_is_newest_and_which_is_finished_are_different_questions() {
    let now = clock("2026-08-14T18:00:02Z");

    // A feed that has already opened the new hour.
    let busy = [candle(18), candle(17)];
    let first_finished = busy
        .iter()
        .position(|b| b.finished_by(now, 60).expect("valid"));
    assert_eq!(first_finished, Some(1), "the finished one is second");

    // A quiet feed that has not.
    let quiet = [candle(17), candle(16)];
    let first_finished = quiet
        .iter()
        .position(|b| b.finished_by(now, 60).expect("valid"));
    assert_eq!(first_finished, Some(0), "the finished one is first");
}

// ── Reading the stamp ──

#[test]
fn the_stamp_is_read_as_the_candles_open_time() {
    assert_eq!(
        candle(17).opened_at().expect("valid"),
        clock("2026-08-14T17:00:00Z")
    );
}

// A daily candle's stamp is a bare date, and it means the day the candle
// *ended* — not the same field's meaning at all. Refusing to read it is right:
// guessing would put every daily candle in the wrong place by a whole day.
#[test]
fn a_daily_stamp_is_refused_rather_than_guessed_at() {
    let daily: Bar = serde_json::from_str(
        r#"{"datetime":"2026-08-14","open":"4350.64","high":"4396.58","low":"4310.61","close":"4391.82"}"#,
    )
    .expect("valid candle");

    assert!(daily.opened_at().is_err());
}

// ── The numbers on the card ──

#[test]
fn a_candle_that_fell_reports_a_negative_change() {
    let bar = candle(17);

    assert!(bar.change().is_sign_negative(), "4377.70 down to 4373.35");
    assert_eq!(bar.change().to_string(), "-4.35");
    assert_eq!(bar.change_percent().to_string(), "-0.10");
}

// A flat candle has no range. Dividing by its open would be fine, but dividing
// by a zero open would not — and gold has traded at prices that round to
// nothing on other instruments.
#[test]
fn a_candle_that_opened_at_nothing_does_not_divide_by_zero() {
    let broken: Bar = serde_json::from_str(
        r#"{"datetime":"2026-08-14 17:00:00","open":"0","high":"0","low":"0","close":"0"}"#,
    )
    .expect("valid candle");

    assert_eq!(broken.change_percent().to_string(), "0");
}

// ── A 4-hour candle is not four times an hourly one ──

// THE LOOKAHEAD RULE, on the timeframe he executes on. A 4-hour candle stamped
// 12:00 is not finished at 13:00, however finished the 1-hour inside it is.
//
// Reading it early would report a rejection at his zone three hours before the
// market had decided there was one — and that mistake does not error, it makes
// the results look better.
#[test]
fn a_four_hour_candle_waits_for_all_four_hours() {
    let bar: Bar = serde_json::from_str(
        r#"{"datetime":"2026-08-19 12:00:00","open":"4100","high":"4110",
            "low":"4090","close":"4105"}"#,
    )
    .expect("valid candle");

    let at = |text: &str| {
        chrono::DateTime::parse_from_rfc3339(text)
            .expect("a real moment")
            .with_timezone(&chrono::Utc)
    };

    let four = |text: &str| bar.finished_by(at(text), 240).expect("a real stamp");

    assert!(!four("2026-08-19T13:00:00Z"), "one hour in");
    assert!(!four("2026-08-19T15:59:59Z"), "a second short");
    assert!(four("2026-08-19T16:00:00Z"), "all four hours done");

    // And the hourly rule is unchanged — the same candle, asked as an hour.
    assert!(
        bar.finished_by(at("2026-08-19T13:00:00Z"), 60)
            .expect("a real stamp")
    );
}
