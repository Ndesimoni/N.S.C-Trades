//! Rounding on the way out, and the line under the picture.

// ── Rounding on the way out ──

// The feed sends gold as 4385.59525. Gold is quoted to two decimals. Letting
// all five through is the single thing that makes a card read like a debug
// dump rather than a signal.
#[test]
fn prices_are_rounded_to_the_instruments_own_precision() {
    use nsc_core::candle::Bar;

    let bar: Bar = serde_json::from_str(
        r#"{"datetime":"2026-08-14 17:00:00","open":"4385.59525","high":"4390.11111",
            "low":"4380.00049","close":"4387.99999"}"#,
    )
    .expect("valid candle");

    let facts = super::super::facts::all(&[&bar], 2);
    let row = &facts[0];

    assert_eq!(row["open"], 4385.60);
    assert_eq!(row["high"], 4390.11);
    assert_eq!(row["close"], 4388.00);

    // And the same candle on a five-decimal pair keeps its detail.
    let facts = super::super::facts::all(&[&bar], 5);
    assert_eq!(facts[0]["open"], 4385.59525);
}

// The chart reads left to right, so the candles go in oldest first and come
// out in that order. Reversed, the chart would be a mirror of itself and still
// look plausible.
#[test]
fn the_candles_keep_the_order_they_were_given() {
    use nsc_core::candle::Bar;

    let candle = |hour: &str, close: &str| -> Bar {
        serde_json::from_str(&format!(
            r#"{{"datetime":"2026-08-14 {hour}:00:00","open":"1","high":"2","low":"0","close":"{close}"}}"#
        ))
        .expect("valid candle")
    };

    let first = candle("15", "10");
    let last = candle("17", "30");

    let facts = super::super::facts::all(&[&first, &last], 2);

    assert_eq!(facts[0]["close"], 10.0);
    assert_eq!(facts[1]["close"], 30.0);
    assert_eq!(facts[0]["at"], "08-14 15:00");
}

// ── The line under the picture ──

// It was written twice — once for the real message, once for the preview — and
// the preview used its own, so all three states arrived captioned identically
// and none of them said what had happened. There is one of it now, and these
// check it actually differs per state.
#[test]
fn each_kind_of_trouble_says_what_happened() {
    use super::super::{Wrong, caption};

    let down = caption(Wrong::LineDown);
    let back = caption(Wrong::LineBack);
    let stopped = caption(Wrong::Stopped);

    assert_ne!(down, back);
    assert_ne!(back, stopped);
    assert_ne!(down, stopped);

    // The sign carries what the colour carries: trying, fine, needs him.
    assert!(down.starts_with('⚠'), "{down}");
    assert!(back.starts_with('✅'), "{back}");
    assert!(stopped.starts_with('🛑'), "{stopped}");

    // And each says what it MEANS, not just what happened. "The line is down"
    // on its own leaves him working out whether that matters.
    for words in [down, back, stopped] {
        assert!(words.contains("watched"), "no consequence in: {words}");
    }
}
