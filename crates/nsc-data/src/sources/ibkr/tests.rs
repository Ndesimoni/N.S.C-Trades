//! How far back a request reaches, and how it has to be written.

use crate::source::Interval;

use super::candles::span;

fn asking(interval: Interval, count: usize) -> String {
    span(interval, count).to_string()
}

/// **The one that was broken.**
///
/// Sixty weekly candles is the number the band sizing asks for. Written as 64
/// weeks, IBKR refused the whole request — *"durations longer than 52 weeks
/// must be made in years"* — and no pair with weekly levels could be sized at
/// all. It failed on the first real run.
#[test]
fn sixty_weekly_candles_is_asked_for_in_years() {
    assert_eq!(asking(Interval::Week, 60), "2 Y");
}

/// Just inside the limit still goes as weeks, because weeks are exact and a
/// year is a rounding.
#[test]
fn under_a_year_of_weeks_stays_in_weeks() {
    // 48 + 4 room = 52, which is the most IBKR will take.
    assert_eq!(asking(Interval::Week, 48), "52 W");

    // One more, and it has to change units.
    assert_eq!(asking(Interval::Week, 49), "2 Y");
}

/// The chart drawing asks for 150 weekly candles — nearly three years.
#[test]
fn a_chart_of_weekly_candles_reaches_back_far_enough() {
    // 154 weeks is 1,078 days, which rounds UP to three years.
    assert_eq!(asking(Interval::Week, 150), "3 Y");
}

/// **Rounded up, never down.**
///
/// Rounding down asks for less history than was wanted, and too few candles
/// does not error — it averages a "normal candle" over a shorter run, which
/// changes every band width on the pair.
#[test]
fn years_are_rounded_up() {
    // 53 weeks is 371 days: over a year, so two rather than one.
    assert_eq!(asking(Interval::Week, 49), "2 Y");
}

/// Daily candles: five a week, so seven days buys five, plus a fortnight for
/// holidays. 150 of them still fits inside a year.
#[test]
fn daily_candles_allow_for_the_weekend() {
    assert_eq!(asking(Interval::Day, 60), "98 D");
    assert_eq!(asking(Interval::Day, 150), "224 D");
}

/// A daily request that runs past a year has to change units too.
#[test]
fn more_than_a_year_of_days_is_asked_for_in_years() {
    // 300 days wanted becomes 434 with the weekend and holiday room.
    assert_eq!(asking(Interval::Day, 300), "2 Y");
}

/// Intraday, in whole days, doubled for the weekend and never less than one.
#[test]
fn intraday_asks_in_whole_days() {
    // Three hourly candles is nothing, but the market may have been shut.
    assert_eq!(asking(Interval::H1, 3), "2 D");

    // Sixty 4-hour candles is ten days of trading, doubled.
    assert_eq!(asking(Interval::H4, 60), "22 D");
}

/// **Asking for nothing still asks for something.**
///
/// A count of nought would otherwise become a duration of nought, and IBKR
/// refuses that outright — so a caller that asked for no candles would take
/// the whole request down instead of getting an empty list.
#[test]
fn asking_for_nothing_still_asks_for_a_real_span() {
    assert_eq!(asking(Interval::H1, 0), "2 D");
    assert_eq!(asking(Interval::Week, 0), "5 W");
}
