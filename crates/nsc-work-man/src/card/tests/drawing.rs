//! The height read off the template, and the page Chrome is handed.

use super::super::fill::height_of;

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
