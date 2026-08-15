use rust_decimal::Decimal;

use super::{Band, Timeframe};

fn d(text: &str) -> Decimal {
    text.parse().expect("a number")
}

// ── The band he actually draws ──

// Measured off his own gold chart. He drew a weekly band from 4055.913 to
// 4132.020 — middle 4093.97, thickness 76.11 — when a weekly gold candle was
// 220.42.
//
// So the rule has to reproduce roughly that from the line alone.
#[test]
fn a_weekly_line_becomes_the_band_he_drew_by_hand() {
    let band = Band::around(Timeframe::Weekly, d("4094"), d("220.42"), d("0.35"));

    assert_eq!(band.thickness().round_dp(2), d("77.15"));

    // Within a point and a half of the 76.11 he drew.
    assert!((band.thickness() - d("76.11")).abs() < d("1.5"));
}

// The same rule on the second one, drawn months earlier at a very different
// price. 3383.480 to 3303.553 — thickness 79.93.
#[test]
fn and_the_one_he_drew_months_earlier_too() {
    let band = Band::around(Timeframe::Weekly, d("3344"), d("220.42"), d("0.35"));

    assert!((band.thickness() - d("79.93")).abs() < d("3"));
}

// His daily band was 32.28 thick against a daily candle of 70.36.
#[test]
fn a_daily_band_is_about_half_a_weekly_one() {
    let daily = Band::around(Timeframe::Daily, d("2984"), d("70.36"), d("0.46"));
    let weekly = Band::around(Timeframe::Weekly, d("4094"), d("220.42"), d("0.35"));

    assert!((daily.thickness() - d("32.28")).abs() < d("1"));
    assert!(daily.thickness() < weekly.thickness() / Decimal::TWO);
}

// ── The line is the middle ──

#[test]
fn the_line_he_drew_sits_in_the_middle() {
    let band = Band::around(Timeframe::Weekly, d("4094"), d("220.42"), d("0.35"));

    assert_eq!(band.price, d("4094"));
    assert_eq!((band.top + band.bottom) / Decimal::TWO, d("4094"));
}

// ── The question a level exists to answer ──

#[test]
fn price_inside_the_band_is_at_the_level() {
    let band = Band::around(Timeframe::Weekly, d("4094"), d("220.42"), d("0.35"));

    assert!(band.holds(d("4094")), "the line itself");
    assert!(band.holds(band.top), "the very top counts");
    assert!(band.holds(band.bottom), "and the very bottom");
    assert!(!band.holds(band.top + Decimal::ONE), "just above does not");
    assert!(!band.holds(band.bottom - Decimal::ONE), "nor just below");
}

// A share of a candle, never a price. 78 points is a normal week on gold and
// about a year on EURUSD — a fixed number would be right on one pair and
// absurd on every other.
#[test]
fn the_same_rule_gives_a_sensible_band_on_a_currency_pair() {
    let gold = Band::around(Timeframe::Weekly, d("4094"), d("220.42"), d("0.35"));
    let euro = Band::around(Timeframe::Weekly, d("1.1567"), d("0.0180"), d("0.35"));

    assert!(gold.thickness() > d("70"), "gold in points");
    assert!(euro.thickness() < d("0.01"), "the euro in its own money");

    // And the same share of their own normal candle.
    assert_eq!(
        (gold.thickness() / d("220.42")).round_dp(2),
        (euro.thickness() / d("0.0180")).round_dp(2)
    );
}

// ── His colours, which are not ours to choose ──

#[test]
fn each_timeframe_keeps_his_colour() {
    assert_eq!(Timeframe::Weekly.colour(), "black");
    assert_eq!(Timeframe::Daily.colour(), "blue");
    assert_eq!(Timeframe::H4.colour(), "yellow");
}

// ── Saving what he sends ──

use super::{digits_for, known, save, with_slash};

/// A scratch folder of its own, so tests cannot tread on each other.
fn scratch(name: &str) -> std::path::PathBuf {
    let folder = std::env::temp_dir().join(format!("nsc-levels-{name}"));
    let _ = std::fs::remove_dir_all(&folder);
    folder
}

#[test]
fn a_pair_that_has_never_been_seen_gets_its_own_file() {
    let folder = scratch("new-pair");

    let saved = save(&folder, "EURUSD", Timeframe::Weekly, &[d("1.0850")], 5).expect("saved");

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
    .expect("saved again");

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
    let saved = super::load_pair(&folder.join("GBPUSD.toml")).expect("read back");

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

    let left = super::undo(&folder, "EURUSD", 1).expect("undone");

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

    let left = super::undo(&folder, "EURUSD", 3).expect("undone");

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
    let left = super::undo(&folder, "EURUSD", 9).expect("undone");

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
    super::undo(&folder, "EURUSD", 1).expect("undone");

    let text = std::fs::read_to_string(folder.join("EURUSD.toml")).expect("readable");
    assert!(text.contains("THIS FILE IS WHY THE PAIR IS WATCHED"));
}
