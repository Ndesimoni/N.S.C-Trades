//! What must never travel on a card, and what must never be stale.

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

// ── A picture that is there is not a picture that was just drawn ──

// The only check that Chrome drew anything is whether a file appeared — and
// one was already there, left by the last card of the same kind. A failed draw
// left the old picture in place and it would have gone out with today's caption
// on yesterday's chart.
//
// It does not run Chrome. Chrome takes ten seconds and, worse, it SUCCEEDS on
// a missing page — it screenshots its own error page, which is the reason
// "a file exists" was never a check in the first place.
#[test]
fn the_last_picture_is_cleared_before_anything_draws() {
    let folder = std::env::temp_dir().join("nsc-card-stale");
    let _ = std::fs::create_dir_all(&folder);

    let out = folder.join("alert.png");
    std::fs::write(&out, b"yesterday").expect("a stale picture");

    super::super::chrome::clear_the_way(&out).expect("cleared");

    assert!(!out.exists(), "the old picture must not survive");

    // Nothing to clear is not a failure. Most draws are the first of their
    // kind since the bot started.
    super::super::chrome::clear_the_way(&out).expect("nothing to clear is fine");
}

// ── Text that goes into a message he must receive ──

// Every message the inbox sends is parsed as HTML. A stray `<` in an error is
// not a stray `<` — it is an unclosed tag, and Telegram refuses the WHOLE
// message.
//
// The one place carrying text nobody wrote on purpose is the reply that says
// what went wrong, which is exactly the message that has to arrive.
#[test]
fn what_went_wrong_survives_being_put_in_a_message() {
    use crate::inbox::plainly;

    let raw = "could not read <config/pairs/EUR&USD.toml>";
    let safe = plainly(raw);

    assert!(!safe.contains('<'), "{safe}");
    assert!(!safe.contains('>'), "{safe}");
    assert_eq!(safe.matches("&amp;").count(), 1, "the & is escaped once");
    assert!(safe.contains("EUR&amp;USD"), "{safe}");
}
