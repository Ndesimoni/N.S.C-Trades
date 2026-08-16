//! Taking levels back off a pair.
//!
//! Two ways, and they answer different questions. **Undo** reaches what the
//! last message added — a typo, caught the moment it happens. **Take one off**
//! reaches any level by its price, which is the one he actually needs when a
//! line he drew last week turns out to be wrong.

use super::super::{Timeframe, save, undo};
use super::support::{d, scratch};

// ── Taking one back off ──

#[test]
fn undo_takes_off_only_what_was_just_added() {
    let folder = scratch("undo");

    save(
        &folder,
        "EURUSD",
        Timeframe::Weekly,
        &[d("1.05"), d("1.10")],
        5,
    )
    .expect("saved");
    save(&folder, "EURUSD", Timeframe::Daily, &[d("1.08")], 5).expect("saved again");

    let left = undo(&folder, "EURUSD", 1).expect("undone");

    assert_eq!(left.levels.len(), 2, "the two weeklies stay");
    assert!(
        left.levels
            .iter()
            .all(|line| line.timeframe == Timeframe::Weekly)
    );
}

#[test]
fn undo_can_take_off_a_whole_batch() {
    let folder = scratch("undo-batch");

    save(&folder, "EURUSD", Timeframe::Weekly, &[d("1.05")], 5).expect("saved");
    save(
        &folder,
        "EURUSD",
        Timeframe::Weekly,
        &[d("1.10"), d("1.15"), d("1.20")],
        5,
    )
    .expect("saved again");

    let left = undo(&folder, "EURUSD", 3).expect("undone");

    assert_eq!(left.levels.len(), 1);
    assert_eq!(left.levels[0].price.to_string(), "1.05");
}

// Undoing more than there is empties it rather than breaking the file — and
// the pair's own settings survive, because it is still the same pair.
#[test]
fn undoing_everything_leaves_the_pair_itself_intact() {
    let folder = scratch("undo-all");

    save(
        &folder,
        "EURUSD",
        Timeframe::Weekly,
        &[d("1.05"), d("1.10")],
        5,
    )
    .expect("saved");
    let left = undo(&folder, "EURUSD", 9).expect("undone");

    assert!(left.levels.is_empty());
    assert_eq!(left.symbol, "EUR/USD");
    assert_eq!(left.digits, 5);
}

// The comments explaining what a level is must survive a removal, same as they
// survive a save.
#[test]
fn undo_keeps_the_comments() {
    let folder = scratch("undo-comments");

    save(&folder, "EURUSD", Timeframe::Weekly, &[d("1.05")], 5).expect("saved");
    undo(&folder, "EURUSD", 1).expect("undone");

    let text = std::fs::read_to_string(folder.join("EURUSD.toml")).expect("readable");
    assert!(text.contains("THIS FILE IS WHY THE PAIR IS WATCHED"));
}

// ── Taking one particular level off ──

// UNDO ONLY REACHES THE LAST MESSAGE. That covers a typo the moment it
// happens, and does nothing for "that 1.15 from last week was wrong".
#[test]
fn one_level_can_be_taken_off_by_its_price() {
    use super::super::take_off;

    let folder = scratch("take-off");
    save(
        &folder,
        "GBPUSD",
        Timeframe::Weekly,
        &[d("1.14"), d("1.21279"), d("1.28")],
        5,
    )
    .expect("saved");

    let left = take_off(&folder, "GBPUSD", d("1.21279"))
        .expect("taken off")
        .pair;

    assert_eq!(left.levels.len(), 2);
    let prices: Vec<_> = left.levels.iter().map(|line| line.price).collect();
    assert_eq!(
        prices,
        vec![d("1.14"), d("1.28")],
        "and the others keep order"
    );
}

// He may tap a level written 1.15000 and mean the one he typed as 1.15. As
// text those are two different levels; as numbers they are one.
#[test]
fn it_matches_the_price_as_a_number() {
    use super::super::take_off;

    let folder = scratch("take-off-text");
    save(&folder, "EURUSD", Timeframe::Weekly, &[d("1.15000")], 5).expect("saved");

    let left = take_off(&folder, "EURUSD", d("1.15"))
        .expect("taken off")
        .pair;
    assert!(left.levels.is_empty());
}

// The comments explain what a level is and where the numbers came from. He is
// meant to be able to open one of these files and read it.
#[test]
fn taking_one_off_keeps_the_comments() {
    use super::super::take_off;

    let folder = scratch("take-off-comments");
    save(
        &folder,
        "GBPUSD",
        Timeframe::Weekly,
        &[d("1.28"), d("1.31")],
        5,
    )
    .expect("saved");

    take_off(&folder, "GBPUSD", d("1.28")).expect("taken off");

    let text = std::fs::read_to_string(folder.join("GBPUSD.toml")).expect("readable");
    assert!(text.contains('#'), "the comments are still there");
    assert!(text.contains("1.31"), "and so is the other level");
    assert!(!text.contains("1.28"), "but not the one taken off");
}

// A price that is not on the pair changes nothing. He may tap a stale button.
#[test]
fn taking_off_something_that_is_not_there_changes_nothing() {
    use super::super::take_off;

    let folder = scratch("take-off-missing");
    save(&folder, "GBPUSD", Timeframe::Weekly, &[d("1.28")], 5).expect("saved");

    let took = take_off(&folder, "GBPUSD", d("9.99")).expect("no trouble");
    assert_eq!(took.pair.levels.len(), 1);

    // **And it has to SAY nothing came off.** The reply is built from this,
    // and it told him "9.99 taken off" while the file was untouched. A stale
    // button on a phone is one tap, and old keyboards stay tappable forever.
    assert!(!took.was_there, "it should know it removed nothing");
}

// The ordinary case still reports that it did remove one.
#[test]
fn taking_off_something_that_is_there_says_so() {
    use super::super::take_off;

    let folder = scratch("take-off-present");
    save(&folder, "GBPUSD", Timeframe::Weekly, &[d("1.28")], 5).expect("saved");

    let took = take_off(&folder, "GBPUSD", d("1.28")).expect("no trouble");
    assert!(took.was_there);
    assert!(took.pair.levels.is_empty());
}
