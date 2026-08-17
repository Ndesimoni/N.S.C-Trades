//! What the caption says about a chart that holds none of his levels.

use std::path::PathBuf;

use nsc_core::levels::{Pair, Timeframe};

use super::asked::caption;
use crate::review::Drawn;

fn pair() -> Pair {
    Pair {
        symbol: "USD/CAD".into(),
        digits: 5,
        nightly_break_minutes: 0,
        approach_pips: None,
        levels: Vec::new(),
    }
}

fn drawn(on_it: usize, altogether: usize) -> Drawn {
    Drawn {
        picture: PathBuf::from("preview/asked-for.png"),
        on_it,
        altogether,
    }
}

#[test]
fn every_level_on_the_chart_says_nothing_extra() {
    let words = caption(&pair(), Timeframe::Weekly, &drawn(3, 3));

    assert_eq!(words, "📈 <b>USD/CAD</b> — the weekly chart.");
}

/// **The one this exists for.** A 4-hour chart is twenty-five days wide and his
/// weekly levels are years apart, so it draws perfectly and comes out empty.
/// Saying nothing would leave it looking like the bands broke.
#[test]
fn an_empty_chart_says_so_rather_than_looking_broken() {
    let words = caption(&pair(), Timeframe::H4, &drawn(0, 3));

    assert!(
        words.contains("None of your 3 levels reach this far in"),
        "{words}"
    );
    assert!(words.contains("4-hour"), "{words}");
}

#[test]
fn one_level_off_the_chart_is_not_called_levels() {
    let words = caption(&pair(), Timeframe::H4, &drawn(0, 1));

    assert!(words.contains("Your level is outside"), "{words}");
}

#[test]
fn some_on_it_says_how_many() {
    let words = caption(&pair(), Timeframe::Daily, &drawn(1, 3));

    assert!(words.contains("1 of your 3 levels is on it"), "{words}");
}

#[test]
fn more_than_one_on_it_reads_as_plural() {
    let words = caption(&pair(), Timeframe::Daily, &drawn(2, 3));

    assert!(words.contains("2 of your 3 levels are on it"), "{words}");
}

/// **A pair with nothing drawn on it gets a plain chart.** Nought of nought
/// used to fall into the "none of them reached" branch and tell him that none
/// of his 0 levels were on screen.
#[test]
fn a_pair_with_no_levels_is_not_told_none_reached() {
    let words = caption(&pair(), Timeframe::Weekly, &drawn(0, 0));

    assert_eq!(words, "📈 <b>USD/CAD</b> — the weekly chart.");
}
