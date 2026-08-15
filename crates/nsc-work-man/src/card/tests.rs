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

/// A card, assembled the way `draw` assembles one: the shared styling, then
/// the card\'s own, then its markup.
///
/// **Composed rather than read from one file**, because that is what actually
/// runs. Reading only the `.html` was what these tests did until the card
/// styling moved into `<name>.css` beside it, and both of them then passed on
/// a file that no longer held the number they were checking.
fn assembled(name: &str) -> (String, String) {
    let read = |file: &str| {
        std::fs::read_to_string(format!("../../assets/card/{file}"))
            .unwrap_or_else(|_| panic!("{file} should be there"))
    };

    let own = read(&format!("{name}.css"));
    let page = format!(
        "{}{own}{}",
        read("style.css"),
        read(&format!("{name}.html"))
    );

    (own, page)
}

// The alert card is shorter than the chart cards on purpose. If this stops
// being true the card goes out with a field of white under it.
#[test]
fn the_alert_card_asks_for_its_own_height() {
    let shared = std::fs::read_to_string("../../assets/card/style.css").expect("style.css");
    let (own, page) = assembled("alert");

    assert_ne!(
        height_of(&shared),
        height_of(&own),
        "it wants a different one"
    );
    assert_eq!(height_of(&page), height_of(&own), "and it gets it");
}

// Every card that asks for its own height has to actually get it. One that
// quietly fell back on the shared 647 would be sent with white under it, and
// nothing would say so.
#[test]
fn every_card_that_wants_its_own_height_gets_it() {
    for name in ["alert", "close", "heartbeat"] {
        let (own, page) = assembled(name);

        assert!(
            own.contains("--card-height"),
            "{name}.css should set its own height"
        );
        assert_eq!(height_of(&page), height_of(&own), "{name}");
    }
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

// The heartbeat grows a row per pair, so its height is worked out rather than
// typed. Two things have to hold for that to be safe.
#[test]
fn the_heartbeat_card_is_told_how_tall_to_be() {
    let (own, page) = assembled("heartbeat");

    assert!(own.contains("/*__TALL__*/"), "it asks to be told");

    // Unfilled, it gives NOTHING rather than falling back on the shared 647.
    // A silent fallback would clip the last pair off every heartbeat he gets.
    assert_eq!(height_of(&page), None);

    // Filled, the number it was given wins over the shared one above it.
    assert_eq!(height_of(&page.replace("/*__TALL__*/", "376")), Some(376));
}

// ── The line under the picture ──

// It was written twice — once for the real message, once for the preview — and
// the preview used its own, so all three states arrived captioned identically
// and none of them said what had happened. There is one of it now, and these
// check it actually differs per state.
#[test]
fn each_kind_of_trouble_says_what_happened() {
    use super::{Wrong, caption};

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

// ── Secrets must not travel on a card ──

// The detail line on a trouble card is an error chain, and an error chain picks
// up whatever the failing code was holding. reqwest puts the URL it was trying
// into its message, and BOTH SECRETS LIVE IN A URL — so "could not reach
// Telegram" once arrived in the terminal with the bot token printed in full.
//
// A card goes to Telegram AND is left on disk in preview/, so this is the last
// place to catch it.
#[test]
fn a_secret_never_reaches_the_card() {
    let token = "8988717584:AAGHHfmyivoFbkDXbaJ0BARMa";
    unsafe { std::env::set_var("TELEGRAM_BOT_TOKEN", token) };

    let leaked = format!("error sending request for url (https://api.telegram.org/bot{token}/x)");
    let clean = crate::watch::scrub_for_tests(&leaked);

    assert!(!clean.contains(token), "the token survived: {clean}");
    assert!(
        clean.contains("api.telegram.org"),
        "and it still says what failed"
    );
}
