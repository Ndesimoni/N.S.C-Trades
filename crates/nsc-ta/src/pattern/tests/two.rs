//! Two candles together, on runs that actually printed.

use nsc_core::candle::Bar;

use super::rules;
use super::runs::*;
use crate::pattern::{Pattern, ending_at};

fn read(bars: &[Bar], normal: rust_decimal::Decimal) -> Option<Pattern> {
    let borrowed: Vec<&Bar> = bars.iter().collect();

    ending_at(&borrowed, normal, &rules())
}

/// The clearest bullish engulfing in three years of gold.
#[test]
fn a_real_bullish_engulfing_is_found() {
    assert_eq!(
        read(&engulf_up(), normal_2024()),
        Some(Pattern::Engulfing { up: true })
    );
}

/// **The same real engulfing, on a busier day, is not one.**
///
/// Its candle reaches 22.60 points. Against the 18-point normal candle of
/// February 2024 that is 1.26 of one and it counts; against a 30-point normal
/// it is 0.75 and it does not.
///
/// **Nothing about the shape changed** — both body tests are shares of the
/// first candle and both still pass. What changed is whether anything
/// happened, and that is the half `min_reach` exists to ask about.
#[test]
fn an_engulfing_that_did_not_move_is_not_one() {
    use rust_decimal::Decimal;

    // The day it printed: a real reversal.
    assert_eq!(
        read(&engulf_up(), normal_2024()),
        Some(Pattern::Engulfing { up: true })
    );

    // The same candles where an ordinary candle is 30 points: a shrug.
    //
    // **It does not go quiet — it comes back a TWEEZER.** `tweezer` is tried
    // after engulfing and it has no size test of its own, so a shape refused
    // for being small can still be named something weaker. That costs nothing
    // today because rung 3 does not trade tweezers, and it is pinned here so
    // nobody discovers it by surprise the day one is added.
    assert!(
        !matches!(
            read(&engulf_up(), Decimal::from(30)),
            Some(Pattern::Engulfing { .. })
        ),
        "an engulfing reaching 0.75 of a normal candle was still called one"
    );
}

/// And the bearish one.
#[test]
fn a_real_bearish_engulfing_is_found() {
    assert_eq!(
        read(&engulf_down(), normal_2024()),
        Some(Pattern::Engulfing { up: false })
    );
}

/// **One run, two true statements — and only one answer.**
///
/// The 7 June 2024 candles are the clearest bearish engulfing AND the clearest
/// tweezer top in the whole history. Engulfing wins because it is the stricter
/// statement: it says where the bodies are, not only where the highs landed.
///
/// Reporting both would let a backtest count one setup twice, which is the
/// mistake this project has written down twice already.
#[test]
fn a_run_that_is_two_things_comes_back_as_one() {
    let bars = engulf_down();

    assert!(
        (bars[0].high - bars[1].high).abs() < normal_2024(),
        "these two really do share a high",
    );

    assert_eq!(
        read(&bars, normal_2024()),
        Some(Pattern::Engulfing { up: false })
    );
}

/// Two candles resting on the same low, with no engulfing to outrank it.
#[test]
fn a_real_tweezer_bottom_is_found() {
    assert_eq!(
        read(&tweezer_bottom(), normal_2026()),
        Some(Pattern::Tweezer { top: false })
    );
}

/// An up candle, then one closing back below its middle.
#[test]
fn a_real_dark_cloud_cover_is_found() {
    assert_eq!(
        read(&dark_cloud(), normal_2026()),
        Some(Pattern::DarkCloudCover)
    );
}

/// **The engulfing rule that is not in the textbook.**
///
/// Spot forex does not gap: the second candle opens EXACTLY where the first
/// closed — 2030.36 on both, in the run below. So the textbook "second body
/// covers the first" is half free every single time, and the pattern collapses
/// into "closed past the first candle's open".
///
/// Left alone that found one engulfing every six candles on gold. The second
/// body has to be bigger by a real margin.
#[test]
fn the_second_body_must_actually_be_bigger() {
    let bars = engulf_up();

    assert_eq!(bars[0].close, bars[1].open, "spot forex leaves no gap");

    // The real one is 2.5x and passes. Shrink the second candle's close so it
    // only just covers, and it stops being a reversal worth the name.
    let mut barely = bars.clone();
    barely[1].close = bars[0].open + rust_decimal::Decimal::ONE;

    // **Not "nothing" — not an ENGULFING.** It falls through and comes back a
    // tweezer bottom, because those two candles really do share a low. That is
    // the running order doing its job, not a miss.
    assert!(
        !matches!(
            read(&barely, normal_2024()),
            Some(Pattern::Engulfing { .. })
        ),
        "a bare cover is not an engulfing",
    );
}

/// One candle cannot be a pattern, and asking is not an error.
#[test]
fn one_candle_is_never_a_pattern() {
    let bars = engulf_up();

    assert_eq!(read(&bars[..1], normal_2024()), None);
}

// ── the harami, which had no tests at all until 29 August 2026 ─────────────

/// Gold, 21 April 2022: a real one.
#[test]
fn a_harami_is_found() {
    use rust_decimal::Decimal;

    assert_eq!(
        read(&harami_down(), Decimal::from(4)),
        Some(Pattern::Harami { up: false })
    );
}

/// **The big candle has to be big, and `min_first_body` does not ask that.**
///
/// It only says the first candle is mostly body — which a five-point candle in
/// a dead hour also is. This one reaches 11.61 points: 2.90 normal candles on
/// the day it printed, and 0.58 of one where an ordinary candle is 20.
///
/// **The small candle is left alone deliberately.** Its smallness IS the
/// pattern, and a floor under it would ask the pause not to be a pause.
#[test]
fn a_harami_whose_big_candle_is_not_big_is_not_one() {
    use rust_decimal::Decimal;

    // As with the engulfing, it does not fall silent — it is renamed to
    // something with no size test. What matters is that it is no longer a
    // harami, so it can never reach rung 3.
    assert!(
        !matches!(
            read(&harami_down(), Decimal::from(20)),
            Some(Pattern::Harami { .. })
        ),
        "a harami whose big candle reached 0.58 of a normal candle was still \
         called one"
    );
}

/// **Every size rule here is a FLOOR, and bigger is always fine.**
///
/// Confirmed by him on 29 August 2026: *"it should not be exactly like that, it
/// could have a minimum, greater or equal to."*
///
/// The same real harami against a tiny normal candle reaches 11 of one, and it
/// is still a harami. Nothing in `min_first_reach` or `min_reach` has an upper
/// end, and nothing ever should — a shape does not stop being itself by being
/// bigger than asked.
#[test]
fn bigger_than_the_minimum_is_always_still_the_shape() {
    use rust_decimal::Decimal;

    for normal in [1u32, 2, 3, 4] {
        assert_eq!(
            read(&harami_down(), Decimal::from(normal)),
            Some(Pattern::Harami { up: false }),
            "a harami reaching {} normal candles was refused — the floor has \
             grown a ceiling",
            11.61 / f64::from(normal)
        );
    }
}
