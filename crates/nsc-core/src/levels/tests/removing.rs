//! Taking levels back off, and stopping a pair.
//!
//! The other half of `saving.rs`. Both write to a real file — none of what
//! these check can be seen without one.

use super::super::{RETIRED, Timeframe, known, retire, save, undo};
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

// ── Stopping a pair ──

// MOVED, NOT DELETED. It is done by tapping a button on a phone and it throws
// away every level he has drawn for that pair — months of chart work in one
// tap. It has to be possible to get it back.
#[test]
fn stopping_a_pair_puts_its_levels_somewhere_they_can_be_found() {
    let folder = scratch("retire");
    save(&folder, "GBPUSD", Timeframe::Weekly, &[d("1.28")], 5).expect("saved");

    let landed = retire(&folder, "GBPUSD").expect("retired");

    assert!(landed.exists(), "the file is still on disk");
    assert!(landed.starts_with(folder.join(RETIRED)));
    assert!(
        !folder.join("GBPUSD.toml").exists(),
        "and out of the watched folder"
    );
}

// The bot must stop seeing it the moment it moves. `known` looks at .toml
// files, and a folder is not one — but that is worth pinning, because the day
// it starts recursing, every retired pair comes back to life.
#[test]
fn a_stopped_pair_is_not_in_the_list_any_more() {
    let folder = scratch("retire-hidden");
    save(&folder, "GBPUSD", Timeframe::Weekly, &[d("1.28")], 5).expect("saved");
    save(&folder, "EURUSD", Timeframe::Weekly, &[d("1.15")], 5).expect("saved");

    retire(&folder, "GBPUSD").expect("retired");

    assert_eq!(known(&folder), vec!["EURUSD".to_string()]);
}

// He may add a pair back, draw it again, and drop it again. The first set is
// still the one he spent an evening on.
#[test]
fn stopping_the_same_pair_twice_keeps_both_sets() {
    let folder = scratch("retire-twice");

    save(&folder, "GBPUSD", Timeframe::Weekly, &[d("1.28")], 5).expect("saved");
    let first = retire(&folder, "GBPUSD").expect("retired");

    save(&folder, "GBPUSD", Timeframe::Weekly, &[d("1.31")], 5).expect("saved again");
    let second = retire(&folder, "GBPUSD").expect("retired again");

    assert_ne!(first, second, "the second must not land on the first");
    assert!(first.exists() && second.exists(), "both are still there");
}
