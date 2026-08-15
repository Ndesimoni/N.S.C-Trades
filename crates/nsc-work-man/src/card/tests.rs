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

// THE LAST ONE WINS, because that is what the browser does with two
// declarations of the same custom property.
//
// It matters now that style.css sets a shared height and is dropped in at the
// top of every template: a card wanting its own says so further down. Reading
// the first would have Rust asking Chrome for one height while the page drew
// another, and that difference comes out as a strip of white — the exact bug
// this number exists to stop.
#[test]
fn a_templates_own_height_beats_the_shared_one() {
    assert_eq!(
        height_of("--card-height:647px; --card-height:462px;"),
        Some(462)
    );
}

// The alert card is shorter than the chart cards on purpose, and it is the
// first template to overrule the shared height. If this stops being true the
// card gets sent with a field of white under it.
#[test]
fn the_alert_card_asks_for_its_own_height() {
    let shared = std::fs::read_to_string("../../assets/card/style.css").expect("style.css");
    let card = std::fs::read_to_string("../../assets/card/alert.html").expect("alert.html");

    let together = format!("{shared}{card}");

    assert_ne!(
        height_of(&shared),
        height_of(&card),
        "it wants a different one"
    );
    assert_eq!(height_of(&together), height_of(&card), "and it gets it");
}

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
    use nsc_core::candle::Bar;

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
