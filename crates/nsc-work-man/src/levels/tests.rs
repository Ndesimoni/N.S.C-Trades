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
