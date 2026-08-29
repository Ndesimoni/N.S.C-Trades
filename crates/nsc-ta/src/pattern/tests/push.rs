//! His own pattern, on runs that actually printed.
//!
//! **Every candle here is real.** Swept out of live IBKR data on 21 August
//! 2026 across all five pairs and every timeframe from 30 minutes up.

use nsc_core::candle::Bar;
use rust_decimal::Decimal;

use super::rules;
use super::runs::*;
use crate::pattern::{Pattern, ending_at};

fn read(run: (Vec<Bar>, Decimal)) -> Option<Pattern> {
    let (bars, normal) = run;
    let borrowed: Vec<&Bar> = bars.iter().collect();

    ending_at(&borrowed, normal, &rules())
}

/// The two candles he circled on his own chart.
#[test]
fn his_own_gold_pair_is_found() {
    assert_eq!(read(his_gold()), Some(Pattern::Push { up: true }));
}

/// And the same shape upside down.
#[test]
fn a_push_down_met_by_a_tail_up_is_found() {
    assert_eq!(read(bear_gold()), Some(Pattern::Push { up: false }));
}

/// **A body of exactly zero must not divide by zero, and must still count.**
///
/// The tail-to-body test cannot score this candle. What lets it through is the
/// nose cap: with no body and no nose, everything left is tail.
#[test]
fn a_pin_with_no_body_at_all_is_found() {
    assert_eq!(read(no_body_pin()), Some(Pattern::Push { up: false }));
}

/// **The largest body the rule can ever admit.**
///
/// `config/candles.toml` names this candle `plain`, because its `body.small`
/// is 0.33 and this body is 0.3333. The pattern keeps its own cap at 0.3334
/// so it is not at the mercy of a setting shared with every other candle.
#[test]
fn a_pin_body_of_exactly_one_third_is_found() {
    assert_eq!(read(exactly_a_third()), Some(Pattern::Push { up: false }));
}

/// **The rule that matters most.** A tail pointing the same way as the push is
/// the push being refused, not the pullback — a different thing, and not this.
#[test]
fn a_tail_pointing_with_the_push_is_not_the_pattern() {
    assert_eq!(read(tail_with_the_push()), None);
}

/// **Shape is not size.** Both candles are the right shape and the push moved
/// less than a normal candle, so nothing was pushed.
#[test]
fn a_push_smaller_than_a_normal_candle_is_not_the_pattern() {
    assert_eq!(read(push_too_small()), None);
}

/// **A pullback that moved further than the push is not a pullback.**
///
/// Every other test of his passes on this run — the push is 62% body and
/// reached 1.6 normal candles, and the pin has a long tail the right way. It
/// is refused because the pin covers nearly three times the ground.
///
/// **It is not nothing, though, and that is worth pinning.** The same two
/// candles are a textbook engulfing, and once his own pattern steps aside the
/// engulfing is what gets reported. Asserting `None` here would be asserting
/// something false about the run.
#[test]
fn a_pin_bigger_than_its_push_is_not_his_pattern() {
    let found = read(pin_bigger_than_push());

    assert!(
        !matches!(found, Some(Pattern::Push { .. })),
        "the pin covers three times the push and must not be his pattern"
    );

    assert_eq!(found, Some(Pattern::Engulfing { up: true }));
}

/// **The same run, judged against a bigger market.** Nothing about the candles
/// changes; the yardstick does, and the push stops being a real move.
///
/// This is the test that would have caught handing over today's normal candle
/// for a run from two years ago.
#[test]
fn the_yardstick_alone_can_turn_it_off() {
    let (bars, normal) = his_gold();
    let borrowed: Vec<&Bar> = bars.iter().collect();

    assert_eq!(
        ending_at(&borrowed, normal, &rules()),
        Some(Pattern::Push { up: true })
    );

    // Twice the normal candle, and the same push no longer reaches.
    assert_eq!(ending_at(&borrowed, normal * Decimal::TWO, &rules()), None);
}
