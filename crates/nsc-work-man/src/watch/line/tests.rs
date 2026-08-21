//! When a refusal is worth stopping over.

use super::refusals::Refusals;

/// **Nought refused out of nought asked is not a total refusal.**
///
/// Removing his last pair left the watcher asking about nothing. `0 == 0` came
/// out true, so it reported every pair as refused — which read as the price
/// line breaking, every thirty seconds, over a bot doing exactly what it had
/// been told.
#[test]
fn asking_about_nothing_is_not_being_refused_everything() {
    let mut refusals = Refusals::watching(0);

    assert!(!refusals.and_that_is_everything("EUR/USD", "no subscription"));
}

/// Every pair refused really is fatal — nothing would ever arrive.
#[test]
fn every_pair_refused_stops_the_line() {
    let mut refusals = Refusals::watching(1);

    assert!(refusals.and_that_is_everything("XAU/USD", "no metals subscription"));
}

/// Some refused, some not: said out loud, but it carries on.
#[test]
fn one_bad_pair_does_not_stop_the_others() {
    let mut refusals = Refusals::watching(2);

    assert!(!refusals.and_that_is_everything("XAU/USD", "no metals subscription"));
}

/// **One pair complaining twice is still one pair.**
///
/// IBKR repeats a notice whenever it feels like it. Counted twice, two
/// complaints from gold alone would look like all four pairs refusing and take
/// down a line that was watching three of them perfectly well.
#[test]
fn the_same_pair_refused_twice_is_still_one_pair() {
    let mut refusals = Refusals::watching(2);

    assert!(!refusals.and_that_is_everything("XAU/USD", "no metals subscription"));
    assert!(!refusals.and_that_is_everything("XAU/USD", "no metals subscription"));
    assert!(!refusals.and_that_is_everything("XAU/USD", "said it again"));
}

/// What it says when it does stop names every pair and why.
#[test]
fn it_says_which_pairs_and_why() {
    let mut refusals = Refusals::watching(2);

    refusals.and_that_is_everything("EUR/USD", "no forex subscription");
    refusals.and_that_is_everything("XAU/USD", "no metals subscription");

    let said = refusals.what_they_said();

    assert!(said.contains("EUR/USD"), "{said}");
    assert!(said.contains("no metals subscription"), "{said}");
}
