//! The star, and the abandoned baby inside it.

use nsc_core::candle::Bar;

use super::rules;
use super::runs::*;
use crate::pattern::{Pattern, ending_at};

fn read(bars: &[Bar]) -> Option<Pattern> {
    let borrowed: Vec<&Bar> = bars.iter().collect();

    ending_at(&borrowed, normal_2026(), &rules())
}

/// Down hard, a pause where nothing happened, then back up.
#[test]
fn a_real_morning_star_is_found() {
    assert_eq!(
        read(&morning_star()),
        Some(Pattern::Star {
            up: true,
            abandoned: false
        }),
    );
}

/// The same shape, the other way up.
#[test]
fn a_real_evening_star_is_found() {
    assert_eq!(
        read(&evening_star()),
        Some(Pattern::Star {
            up: false,
            abandoned: false
        }),
    );
}

/// **No real morning star on gold is an abandoned baby, and none can be.**
///
/// The strict Japanese pattern needs the middle candle to clear both
/// neighbours outright. Spot forex runs Sunday evening to Friday evening
/// without a break, so a candle's open IS the last one's close — the gap has
/// nowhere to come from.
///
/// This is the single most important thing the flag records. Around 70%
/// accuracy, the best evidence on the whole list, and it will effectively
/// never fire on his instruments.
#[test]
fn a_real_star_never_comes_back_abandoned() {
    for bars in [morning_star(), evening_star()] {
        let Some(Pattern::Star { abandoned, .. }) = read(&bars) else {
            panic!("it should still be a star");
        };

        assert!(!abandoned, "spot forex cannot gap mid-week");
    }
}

/// Move the middle candle clear of both and the same code says so.
#[test]
fn a_star_that_really_gapped_is_an_abandoned_baby() {
    assert_eq!(
        read(&abandoned_baby_made_up()),
        Some(Pattern::Star {
            up: true,
            abandoned: true
        }),
    );
}

/// **The gap is a setting, and turning it on turns the pattern off.**
///
/// With `require_gap = true` the real morning star above stops being reported
/// at all — which is the honest answer for a market that gaps, and silence for
/// one that does not. It is `false` in `config/patterns.toml` for exactly that
/// reason.
#[test]
fn requiring_the_gap_silences_a_real_star() {
    let mut strict = rules();
    strict.star.require_gap = true;

    let bars = morning_star();
    let borrowed: Vec<&Bar> = bars.iter().collect();

    // **Not silence — not a STAR.** With the gap demanded it stops being one,
    // and the last two candles are then judged on their own merits. That is
    // the running order, and it is right: they are still two candles that did
    // something.
    assert!(!matches!(
        ending_at(&borrowed, normal_2026(), &strict),
        Some(Pattern::Star { .. }),
    ));

    // And it still finds the one that really gapped.
    let gapped = abandoned_baby_made_up();
    let borrowed: Vec<&Bar> = gapped.iter().collect();

    assert!(matches!(
        ending_at(&borrowed, normal_2026(), &strict),
        Some(Pattern::Star {
            abandoned: true,
            ..
        }),
    ));
}

/// **Three candles beat two.** A star whose last two candles also engulf is a
/// star — reporting both would count one setup twice.
#[test]
fn a_star_outranks_the_two_candles_inside_it() {
    let bars = morning_star();

    assert!(matches!(read(&bars), Some(Pattern::Star { .. })));

    // The last two on their own are a different question entirely.
    assert!(!matches!(read(&bars[1..]), Some(Pattern::Star { .. })));
}

/// 20 January 2026 — gold marching up through 4,665 to 4,733.
#[test]
fn a_real_three_white_soldiers_is_found() {
    assert_eq!(read(&soldiers()), Some(Pattern::Marching { up: true }));
}

/// 28 April 2026 — the same thing downhill.
#[test]
fn a_real_three_black_crows_is_found() {
    assert_eq!(read(&crows()), Some(Pattern::Marching { up: false }));
}

/// **A star and a march can never both be true**, so their order decides
/// nothing. A star turns — its first and third candles disagree. A march does
/// not — all three go the same way.
#[test]
fn a_star_and_a_march_are_never_the_same_run() {
    for bars in [morning_star(), evening_star()] {
        assert!(!matches!(read(&bars), Some(Pattern::Marching { .. })));
    }

    for bars in [soldiers(), crows()] {
        assert!(!matches!(read(&bars), Some(Pattern::Star { .. })));
    }
}

/// **Being pushed back is what stops it being a march.**
///
/// The textbook also asks each candle to open inside the last one's body, but
/// spot forex opens exactly ON the last close — that test passes every time
/// and asks nothing. The wick against the move is the one doing the work.
#[test]
fn a_long_wick_against_the_move_is_not_a_march() {
    let mut pushed = soldiers();

    // Stretch the last candle's high far above its close: it went up, got sold
    // hard, and closed well off the top. Not a march — a fight.
    pushed[2].high =
        pushed[2].close + (pushed[2].close - pushed[2].open) * rust_decimal::Decimal::TEN;

    assert!(!matches!(read(&pushed), Some(Pattern::Marching { .. })));
}
