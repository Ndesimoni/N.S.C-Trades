//! Stopping a pair, and putting it back.
//!
//! **Moved, not deleted.** It is done by tapping a button on a phone and it
//! sets aside every level he has drawn for that pair — months of chart work in
//! one tap. Getting it back has to be possible.

use super::super::{RETIRED, Timeframe, known, load_pair, retire, save};
use super::support::{d, scratch};

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

// ── Putting one back ──

#[test]
fn a_stopped_pair_can_be_put_back() {
    use super::super::{restore, retired};

    let folder = scratch("restore");
    save(&folder, "GBPUSD", Timeframe::Weekly, &[d("1.28")], 5).expect("saved");
    retire(&folder, "GBPUSD").expect("retired");

    assert_eq!(retired(&folder), vec!["GBPUSD".to_string()]);
    assert!(known(&folder).is_empty(), "not watched while it is aside");

    let back = restore(&folder, "GBPUSD").expect("restored");

    assert_eq!(back, "GBP/USD", "it says which pair, the way he reads it");
    assert_eq!(known(&folder), vec!["GBPUSD".to_string()]);
    assert!(retired(&folder).is_empty(), "and out of the aside pile");
}

// THE ONE THAT WOULD LOSE LEVELS. He stops a pair, draws it again from
// scratch, then restores the old set — which would land on top of the new one
// and replace levels he is using with ones he put aside, silently.
#[test]
fn putting_one_back_never_lands_on_a_pair_he_is_watching() {
    use super::super::{restore, retired};

    let folder = scratch("restore-clash");

    save(&folder, "GBPUSD", Timeframe::Weekly, &[d("1.28")], 5).expect("saved");
    retire(&folder, "GBPUSD").expect("retired");

    // Drawn again from scratch, and being watched.
    save(&folder, "GBPUSD", Timeframe::Weekly, &[d("1.31")], 5).expect("saved again");

    assert!(restore(&folder, "GBPUSD").is_err(), "it must refuse");
    assert_eq!(retired(&folder).len(), 1, "and leave the old set alone");

    let live = load_pair(&folder.join("GBPUSD.toml")).expect("still readable");
    assert_eq!(live.levels.len(), 1);
    assert_eq!(live.levels[0].price, d("1.31"), "the one he is using");
}

// A pair stopped twice sits as GBPUSD and GBPUSD-2, and comes back under its
// own name either way — the name on disk is bookkeeping, the name in the file
// is the pair.
#[test]
fn the_second_set_comes_back_under_the_pairs_own_name() {
    use super::super::{restore, retired};

    let folder = scratch("restore-second");

    save(&folder, "GBPUSD", Timeframe::Weekly, &[d("1.28")], 5).expect("saved");
    retire(&folder, "GBPUSD").expect("retired");
    save(&folder, "GBPUSD", Timeframe::Weekly, &[d("1.31")], 5).expect("saved again");
    retire(&folder, "GBPUSD").expect("retired again");

    assert_eq!(retired(&folder).len(), 2);

    restore(&folder, "GBPUSD-2").expect("restored the second set");

    assert_eq!(known(&folder), vec!["GBPUSD".to_string()]);
    let back = load_pair(&folder.join("GBPUSD.toml")).expect("readable");
    assert_eq!(back.levels[0].price, d("1.31"));
}
