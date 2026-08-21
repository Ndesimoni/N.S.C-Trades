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
