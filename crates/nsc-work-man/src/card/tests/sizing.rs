//! **How many candles a chart draws, and where that is decided.**
//!
//! One place: [`render`] cuts to [`RUN`], [`render_ringed`] cuts to
//! [`CONTEXT`], and neither can be talked out of it.
//!
//! It used to be decided by each caller. Four of them slice for themselves and
//! one did not — `review/picture.rs`, the chart he gets when he ASKS for one.
//! It asks IBKR for 150 candles, IBKR reads that as a span of days, and
//! fourteen days of hourly forex came back as over three hundred. He saw it on
//! AUD/USD on 1 September 2026.
//!
//! **A rule every caller has to remember is a rule one of them will forget.**

use nsc_core::candle::Bar;
use rust_decimal::Decimal;

use super::super::fill::newest;
use super::super::sizes::{CONTEXT, RUN};

/// A run of candles that all look the same. Only how many there are matters.
fn many(count: usize) -> Vec<Bar> {
    (0..count)
        .map(|which| Bar {
            datetime: format!("2026-09-01 {:02}:00:00", which % 24),
            open: Decimal::ONE,
            high: Decimal::TWO,
            low: Decimal::ONE,
            close: Decimal::TWO,
        })
        .collect()
}

#[test]
fn the_wide_chart_never_draws_more_than_the_run() {
    let plenty = many(RUN * 3);
    let handed: Vec<&Bar> = plenty.iter().collect();

    assert_eq!(newest(&handed, RUN).len(), RUN);
}

#[test]
fn the_close_up_never_draws_more_than_its_context() {
    let plenty = many(500);
    let handed: Vec<&Bar> = plenty.iter().collect();

    assert_eq!(newest(&handed, CONTEXT).len(), CONTEXT);
}

/// **It takes the NEWEST, not the oldest.** Cutting from the wrong end would
/// draw a chart that stops before the candle the card is about.
#[test]
fn it_keeps_the_end_of_the_run() {
    let plenty = many(300);
    let handed: Vec<&Bar> = plenty.iter().collect();

    let drawn = newest(&handed, RUN);

    assert_eq!(
        drawn.last().map(|bar| &bar.datetime),
        handed.last().map(|bar| &bar.datetime),
        "the newest candle must survive the cut"
    );
    assert_eq!(drawn.len(), RUN);
}

/// Fewer than asked for is not an error. A pair with a short history draws
/// what it has.
#[test]
fn a_short_history_draws_all_of_itself() {
    let few = many(12);
    let handed: Vec<&Bar> = few.iter().collect();

    assert_eq!(newest(&handed, RUN).len(), 12);
    assert!(newest(&[], RUN).is_empty());
}

/// **The close-up is the smaller of the two.** They were 400 and 100, then 150
/// and 45, then 200 and 45 — and through all of it the run has to be the wide
/// one or the ring lands on a chart with no room around it.
#[test]
fn the_run_is_wider_than_the_close_up() {
    // Read through a variable so the compiler cannot fold it away — a check it
    // optimises out is a check that stops being one.
    let (wide, near) = (RUN, CONTEXT);

    assert!(wide > near, "the run is the wide picture: {wide} vs {near}");
}
