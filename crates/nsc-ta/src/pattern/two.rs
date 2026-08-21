//! What two candles do together.

use nsc_core::candle::Bar;
use rust_decimal::Decimal;

use super::body::Body;
use super::{Pattern, Rules};

/// Every two-candle pattern, tightest first.
///
/// **A bearish engulfing can also be a tweezer top** — your own note from June
/// found the 7 June 2024 candle being both. Engulfing wins because it is the
/// stricter statement: it says where the bodies are, not only where the
/// extremes landed.
pub(super) fn ending_at(
    first: &Bar,
    second: &Bar,
    normal: Decimal,
    rules: &Rules,
) -> Option<Pattern> {
    engulfing(first, second, rules)
        .or_else(|| piercing(first, second, rules))
        .or_else(|| harami(first, second, rules))
        .or_else(|| tweezer(first, second, normal, rules))
}

/// A body swallowing the one before it whole.
fn engulfing(first: &Bar, second: &Bar, rules: &Rules) -> Option<Pattern> {
    let (one, two) = (Body::of(first), Body::of(second));

    // **Without this, almost anything engulfs a doji** and the word stops
    // meaning anything at all.
    if one.share(first) < rules.engulfing.min_first_body {
        return None;
    }

    // They have to disagree. Two up candles in a row, the second bigger, is a
    // trend and not a turn.
    if one.up == two.up || !two.covers(one) {
        return None;
    }

    // **The covering is half free on spot forex**, because the second candle
    // opens exactly where the first closed. So the second body has to be
    // bigger by a real margin, or every ordinary retrace reads as a reversal.
    if two.size() < one.size() * rules.engulfing.min_second_of_first {
        return None;
    }

    Some(Pattern::Engulfing { up: two.up })
}

/// A big candle, then a small one hiding inside its body.
fn harami(first: &Bar, second: &Bar, rules: &Rules) -> Option<Pattern> {
    let (one, two) = (Body::of(first), Body::of(second));

    if one.share(first) < rules.harami.min_first_body {
        return None;
    }

    if one.up == two.up || !two.inside(one) {
        return None;
    }

    // Small ENOUGH. A body two-thirds of the one before it is inside it and is
    // still a second push rather than a pause.
    if two.size() > one.size() * rules.harami.max_second_of_first {
        return None;
    }

    Some(Pattern::Harami { up: two.up })
}

/// Two candles reaching the same extreme.
///
/// **The tolerance is in ATR**, because two candles never share a high to the
/// tick and five points is nothing on gold and a day on the euro.
fn tweezer(first: &Bar, second: &Bar, normal: Decimal, rules: &Rules) -> Option<Pattern> {
    if normal <= Decimal::ZERO {
        return None;
    }

    let room = normal * rules.tweezers.tolerance_reach;

    if (first.high - second.high).abs() <= room {
        return Some(Pattern::Tweezer { top: true });
    }

    if (first.low - second.low).abs() <= room {
        return Some(Pattern::Tweezer { top: false });
    }

    None
}

/// A candle closing well into the body of the one before it — but not past it.
///
/// **Past it would be an engulfing**, and the two must never both fire on one
/// candle. Engulfing is tested first, so anything reaching here has already
/// failed to cover.
fn piercing(first: &Bar, second: &Bar, rules: &Rules) -> Option<Pattern> {
    let (one, two) = (Body::of(first), Body::of(second));

    if one.share(first) < rules.piercing.min_first_body || one.up == two.up {
        return None;
    }

    let into = one.size() * rules.piercing.min_close_into_body;

    if two.up && second.close >= one.bottom + into && second.close <= one.top {
        return Some(Pattern::PiercingLine);
    }

    if !two.up && second.close <= one.top - into && second.close >= one.bottom {
        return Some(Pattern::DarkCloudCover);
    }

    None
}
