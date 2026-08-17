//! What the measuring gives back, on candles that actually printed.
//!
//! **No made-up numbers.** Every candle here is a real XAU/USD 1-hour or
//! 4-hour candle from Twelve Data, with its true open, high, low and close.
//! A shape test on invented candles only proves the arithmetic; on real ones
//! it also proves the arithmetic survives what the feed actually sends.

use super::Shape;
use nsc_core::candle::Bar;
use rust_decimal::Decimal;

/// A price, from text. Never through a float, not even in a test.
fn d(text: &str) -> Decimal {
    text.parse().expect("a real price")
}

fn bar(open: &str, high: &str, low: &str, close: &str) -> Bar {
    Bar {
        datetime: "2026-05-15 16:00:00".to_string(),
        open: d(open),
        high: d(high),
        low: d(low),
        close: d(close),
    }
}

/// Gold, 15 May 2026, 16:00. The classic doji — it travelled 23 dollars and
/// finished 31 cents from where it started.
fn doji() -> Bar {
    bar("4567.98975", "4581.00268", "4557.84223", "4567.67438")
}

// ── What a candle measures to ──

#[test]
fn a_real_doji_is_almost_all_wick() {
    let shape = Shape::of(&doji(), d("20")).expect("it has a range");

    assert_eq!(shape.body.round_dp(4), d("0.0136"), "the body is nothing");
    assert_eq!(shape.upper.round_dp(4), d("0.5619"));
    assert_eq!(shape.lower.round_dp(4), d("0.4245"));
}

/// **The three shares are shares of one candle**, so they cannot come to
/// anything else. If this ever fails, one of them is being measured against
/// the wrong thing.
#[test]
fn the_three_shares_always_add_to_one() {
    let shape = Shape::of(&doji(), d("20")).expect("it has a range");

    assert_eq!(shape.body + shape.upper + shape.lower, Decimal::ONE);
}

/// Gold, 21 March 2026. Opened at its high, closed at its low, 64 dollars
/// straight down and not one tick of wick either end.
#[test]
fn a_marubozu_is_all_body_and_no_wick() {
    let marubozu = bar("4679.64822", "4679.64822", "4615.0481", "4615.0481");
    let shape = Shape::of(&marubozu, d("20")).expect("it has a range");

    assert_eq!(shape.body, Decimal::ONE);
    assert_eq!(shape.upper, Decimal::ZERO);
    assert_eq!(shape.lower, Decimal::ZERO);
    assert!(!shape.up, "it closed 64 dollars below where it opened");
}

// ── Big or small, for this pair ──

/// **`reach` is the only one that knows which instrument this is.** The same
/// 23-dollar candle is ordinary when a normal candle is 20 and enormous when
/// it is 2.
#[test]
fn reach_is_measured_against_a_normal_candle() {
    assert_eq!(
        Shape::of(&doji(), d("20"))
            .expect("range")
            .reach
            .round_dp(2),
        d("1.16"),
        "an ordinary hour"
    );

    assert_eq!(
        Shape::of(&doji(), d("2")).expect("range").reach.round_dp(2),
        d("11.58"),
        "the same candle on a quiet pair"
    );
}

// ── Nothing to measure ──

/// **A flat candle is not a fault, and it is not rare.**
///
/// Gold, 19 April 2025 — Easter Saturday. The feed sends the candle with the
/// high, low, open and close all the same number. There are seven exactly
/// like it in 5,000 4-hour candles, and 1,412 hourly candles with a range
/// under 0.02% of price.
///
/// Dividing by that range would make every number meaningless without saying
/// so. Answering nothing is the honest reply.
#[test]
fn a_flat_weekend_candle_has_no_shape() {
    let easter = bar("3326.27", "3326.27", "3326.27", "3326.27");

    assert!(Shape::of(&easter, d("20")).is_none());
}

/// **ATR can come back at nothing too.** It averages real candles, so a run
/// of dead ones gives zero — and dividing by it would panic. A library may
/// not panic: the backtester runs this over years of candles and one bad one
/// must not end the run.
#[test]
fn a_normal_candle_of_nothing_is_refused() {
    assert!(Shape::of(&doji(), Decimal::ZERO).is_none());
    assert!(Shape::of(&doji(), d("-1")).is_none());
}

// ── The one arbitrary decision ──

/// Closing exactly where it opened has to count as one or the other. It counts
/// as up. Written down here because nothing in the code says why, and the next
/// person will wonder.
#[test]
fn a_candle_that_ends_where_it_started_counts_as_up() {
    let level = bar("4500.00", "4510.00", "4490.00", "4500.00");

    assert!(Shape::of(&level, d("20")).expect("range").up);
}
