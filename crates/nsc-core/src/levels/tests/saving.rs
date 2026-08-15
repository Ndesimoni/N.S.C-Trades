//! Levels arriving from his phone, and going back off again.

use super::super::{Timeframe, digits_for, known, load_pair, save, undo, with_slash};
use super::support::{d, scratch};

// ── Saving what he sends ──

#[test]
fn a_pair_that_has_never_been_seen_gets_its_own_file() {
    let folder = scratch("new-pair");

    let saved = save(&folder, "EURUSD", Timeframe::Weekly, &[d("1.0850")], 5)
        .expect("saved")
        .pair;

    assert_eq!(saved.symbol, "EUR/USD");
    assert_eq!(saved.digits, 5);
    assert_eq!(saved.levels.len(), 1);
    assert!(folder.join("EURUSD.toml").exists());
}

// Sending more levels must not lose the ones already there — and must not lose
// the comments either, which is why the file is appended to rather than
// rewritten.
#[test]
fn sending_more_levels_keeps_the_ones_already_there() {
    let folder = scratch("more");

    save(&folder, "GBPUSD", Timeframe::Weekly, &[d("1.28")], 5).expect("saved");
    let saved = save(
        &folder,
        "GBPUSD",
        Timeframe::Daily,
        &[d("1.37"), d("1.31")],
        5,
    )
    .expect("saved again")
    .pair;

    assert_eq!(saved.levels.len(), 3);

    let text = std::fs::read_to_string(folder.join("GBPUSD.toml")).expect("readable");
    assert!(
        text.contains("THIS FILE IS WHY THE PAIR IS WATCHED"),
        "the comments survived"
    );
}

// The files ARE the list. There is no second one to keep in sync.
#[test]
fn the_pairs_are_whatever_files_exist() {
    let folder = scratch("listing");

    assert!(known(&folder).is_empty(), "nothing yet");

    save(&folder, "XAUUSD", Timeframe::Weekly, &[d("4094")], 2).expect("saved");
    save(&folder, "GBPUSD", Timeframe::Weekly, &[d("1.28")], 5).expect("saved");

    assert_eq!(known(&folder), vec!["GBPUSD", "XAUUSD"]);
}

// He types GBPUSD. The feed wants GBP/USD.
#[test]
fn the_name_he_types_becomes_the_name_the_feed_wants() {
    assert_eq!(with_slash("GBPUSD"), "GBP/USD");
    assert_eq!(with_slash("XAUUSD"), "XAU/USD");
    assert_eq!(with_slash("GBP/USD"), "GBP/USD", "already done");
}

// Worked out from the name, and the file it writes says so. Gold to two
// decimals, yen pairs to three, the rest to five.
#[test]
fn how_many_decimals_is_worked_out_from_the_name() {
    assert_eq!(digits_for("XAUUSD"), 2);
    assert_eq!(digits_for("USDJPY"), 3);
    assert_eq!(digits_for("GBPJPY"), 3);
    assert_eq!(digits_for("EURUSD"), 5);
}

// A price must survive the trip through a file exactly. Written as a TOML
// number it would go through a float and 1.21279 would stop being 1.21279.
#[test]
fn a_price_comes_back_out_of_the_file_unchanged() {
    let folder = scratch("exact");

    save(&folder, "GBPUSD", Timeframe::Weekly, &[d("1.21279")], 5).expect("saved");
    let saved = load_pair(&folder.join("GBPUSD.toml")).expect("read back");

    assert_eq!(saved.levels[0].price.to_string(), "1.21279");
}

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

// ── The same level, twice ──

// THE ONE THAT ACTUALLY HAPPENED. He sent the same three euro levels twice and
// got both copies, so one line on his chart became two bands — two alerts, two
// closes, and a heartbeat card claiming seven levels where he had drawn four.
#[test]
fn a_level_he_already_has_is_not_added_again() {
    let folder = scratch("no-doubles");

    save(
        &folder,
        "EURUSD",
        Timeframe::Weekly,
        &[d("1.15"), d("1.17")],
        5,
    )
    .expect("saved");
    let again = save(
        &folder,
        "EURUSD",
        Timeframe::Weekly,
        &[d("1.15"), d("1.17")],
        5,
    )
    .expect("saved");

    assert_eq!(again.added, 0, "nothing new");
    assert_eq!(again.already, 2, "and it says he had both");
    assert_eq!(again.pair.levels.len(), 2, "still two levels, not four");
}

// Tapping send twice is the commonest way it happens, and both copies arrive
// in the same message.
#[test]
fn a_repeat_inside_one_message_is_dropped_too() {
    let folder = scratch("no-doubles-in-one");

    let saved = save(
        &folder,
        "EURUSD",
        Timeframe::Weekly,
        &[d("1.15"), d("1.15"), d("1.17")],
        5,
    )
    .expect("saved");

    assert_eq!(saved.added, 2);
    assert_eq!(saved.already, 1);
    assert_eq!(saved.pair.levels.len(), 2);
}

// 1.15 and 1.15000 are the same line on his chart, and he may type either.
// Compared as text they are two different levels.
#[test]
fn the_same_price_written_differently_is_still_the_same_level() {
    let folder = scratch("no-doubles-text");

    save(&folder, "EURUSD", Timeframe::Weekly, &[d("1.15")], 5).expect("saved");
    let again = save(&folder, "EURUSD", Timeframe::Weekly, &[d("1.15000")], 5).expect("saved");

    assert_eq!(again.added, 0);
    assert_eq!(again.pair.levels.len(), 1);
}

// The same number on two timeframes is TWO levels — a weekly line at 1.15 and
// a daily line at 1.15 are different things, drawn on different charts.
#[test]
fn the_same_price_on_another_timeframe_is_a_different_level() {
    let folder = scratch("doubles-across");

    save(&folder, "EURUSD", Timeframe::Weekly, &[d("1.15")], 5).expect("saved");
    let daily = save(&folder, "EURUSD", Timeframe::Daily, &[d("1.15")], 5).expect("saved");

    assert_eq!(daily.added, 1);
    assert_eq!(daily.pair.levels.len(), 2);
}
