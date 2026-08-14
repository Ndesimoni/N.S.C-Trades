use super::fill::height_of;

// ── Reading the height out of the template ──

// Chrome screenshots a window, not a page, so the template has to say how tall
// it is. Get this wrong and the card is either clipped or has a field of empty
// space under it — which happened four times before the number moved into the
// CSS.
#[test]
fn the_height_is_read_out_of_the_css() {
    let css = ":root{\n  --card-height:647px;\n  --vault:#0E1216;\n}";

    assert_eq!(height_of(css), Some(647));
}

#[test]
fn spaces_around_the_number_do_not_matter() {
    assert_eq!(height_of("--card-height:  748px;"), Some(748));
}

// Better to fail loudly than to guess a height. A guess is a clipped footer
// that nobody notices until it is in a signal.
#[test]
fn a_template_with_no_height_line_gives_nothing() {
    assert_eq!(height_of(":root{ --vault:#0E1216; }"), None);
}

#[test]
fn a_height_that_is_not_a_number_gives_nothing() {
    assert_eq!(height_of("--card-height:tall;"), None);
}

// It reads the first one. A template with two is a mistake, and taking the
// first at least makes it a repeatable mistake.
#[test]
fn the_first_height_wins() {
    assert_eq!(
        height_of("--card-height:100px; --card-height:900px;"),
        Some(100)
    );
}

// ── Rounding on the way out ──

// The feed sends gold as 4385.59525. Gold is quoted to two decimals. Letting
// all five through is the single thing that makes a card read like a debug
// dump rather than a signal.
#[test]
fn prices_are_rounded_to_the_instruments_own_precision() {
    use crate::candle::Bar;

    let bar: Bar = serde_json::from_str(
        r#"{"datetime":"2026-08-14 17:00:00","open":"4385.59525","high":"4390.11111",
            "low":"4380.00049","close":"4387.99999"}"#,
    )
    .expect("valid candle");

    let facts = super::facts::all(&[&bar], 2);
    let row = &facts[0];

    assert_eq!(row["open"], 4385.60);
    assert_eq!(row["high"], 4390.11);
    assert_eq!(row["close"], 4388.00);

    // And the same candle on a five-decimal pair keeps its detail.
    let facts = super::facts::all(&[&bar], 5);
    assert_eq!(facts[0]["open"], 4385.59525);
}

// The chart reads left to right, so the candles go in oldest first and come
// out in that order. Reversed, the chart would be a mirror of itself and still
// look plausible.
#[test]
fn the_candles_keep_the_order_they_were_given() {
    use crate::candle::Bar;

    let candle = |hour: &str, close: &str| -> Bar {
        serde_json::from_str(&format!(
            r#"{{"datetime":"2026-08-14 {hour}:00:00","open":"1","high":"2","low":"0","close":"{close}"}}"#
        ))
        .expect("valid candle")
    };

    let first = candle("15", "10");
    let last = candle("17", "30");

    let facts = super::facts::all(&[&first, &last], 2);

    assert_eq!(facts[0]["close"], 10.0);
    assert_eq!(facts[1]["close"], 30.0);
    assert_eq!(facts[0]["at"], "08-14 15:00");
}
