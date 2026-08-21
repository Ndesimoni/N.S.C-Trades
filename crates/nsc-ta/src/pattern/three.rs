//! What three candles do together — the star, and the march.

use nsc_core::candle::Bar;

use super::body::Body;
use super::{Pattern, Rules};

/// A long move, a moment where nothing happened, then a long move back.
///
/// **The gap is reported, not required.** The classic Japanese star gaps away
/// from the candle before it and the one after. Spot forex runs Sunday evening
/// to Friday evening without a break, so a candle's open IS the last one's
/// close — insist on the gap and this can only form at the Sunday open, once a
/// week.
///
/// When the gaps ARE there and the middle is a doji, it is the pattern the
/// textbooks call an abandoned baby. That is the best-evidenced shape on the
/// whole list at around 70%, and on spot forex it will almost never appear.
/// Both of those are true at the same time, which is why it is a flag on this
/// pattern rather than a detector of its own.
pub(super) fn ending_at(first: &Bar, middle: &Bar, last: &Bar, rules: &Rules) -> Option<Pattern> {
    // **They cannot both be true.** A star turns — its first and third candles
    // disagree. A march does not — all three go the same way. So the order
    // between them decides nothing, and neither can steal the other's candles.
    star(first, middle, last, rules).or_else(|| marching(first, middle, last, rules))
}

/// A long move, a pause, then a long move back.
fn star(first: &Bar, middle: &Bar, last: &Bar, rules: &Rules) -> Option<Pattern> {
    let (one, two, three) = (Body::of(first), Body::of(middle), Body::of(last));

    // The outer two must be real moves, or the shape is three shrugs in a row.
    if one.share(first) < rules.star.min_outer_body || three.share(last) < rules.star.min_outer_body
    {
        return None;
    }

    // **The middle is the whole pattern.** A third push is not a stall.
    if two.share(middle) > rules.star.max_middle_body {
        return None;
    }

    // They have to turn: down then up, or up then down.
    if one.up == three.up {
        return None;
    }

    // And the last one has to give back a real share of the first.
    let into = one.size() * rules.star.min_close_into_body;

    let gave_back = if three.up {
        last.close >= one.bottom + into
    } else {
        last.close <= one.top - into
    };

    if !gave_back {
        return None;
    }

    let abandoned = gapped_clear(first, middle, last);

    if rules.star.require_gap && !abandoned {
        return None;
    }

    Some(Pattern::Star {
        up: three.up,
        abandoned,
    })
}

/// Did the middle candle's WHOLE RANGE clear both neighbours?
///
/// **Range, not body.** A body that clears its neighbours' bodies is an
/// ordinary star. An abandoned baby is the strict one: nothing about the
/// middle candle overlaps what sits either side of it, wicks included.
fn gapped_clear(first: &Bar, middle: &Bar, last: &Bar) -> bool {
    let below = middle.high < first.low && middle.high < last.low;
    let above = middle.low > first.high && middle.low > last.high;

    below || above
}

/// Three candles marching the same way, each closing beyond the last.
///
/// **Three white soldiers going up, three black crows going down.**
///
/// The textbook adds "each opens inside the last one's body". That test is
/// FREE on spot forex — a candle opens exactly where the last one closed, so
/// it is always on the boundary and always passes. It is left out rather than
/// kept as decoration, and the wick test does the work instead: a long wick
/// against the move means they were pushed back and came again, which is a
/// fight rather than a march.
fn marching(first: &Bar, middle: &Bar, last: &Bar, rules: &Rules) -> Option<Pattern> {
    let three = [first, middle, last];
    let bodies: Vec<Body> = three.iter().map(|bar| Body::of(bar)).collect();

    let up = bodies[0].up;

    // All three the same way, and every one a real move.
    for (bar, body) in three.iter().zip(&bodies) {
        if body.up != up || body.share(bar) < rules.soldiers.min_body {
            return None;
        }
    }

    // Each one has to finish beyond the last, or they are marking time.
    for pair in three.windows(2) {
        let beyond = if up {
            pair[1].close > pair[0].close
        } else {
            pair[1].close < pair[0].close
        };

        if !beyond {
            return None;
        }
    }

    // And none of them may have been pushed far back on the way.
    for (bar, body) in three.iter().zip(&bodies) {
        let range = bar.high - bar.low;

        if range <= rust_decimal::Decimal::ZERO {
            return None;
        }

        let against = if up {
            bar.high - body.top
        } else {
            body.bottom - bar.low
        };

        if against / range > rules.soldiers.max_wick_against {
            return None;
        }
    }

    Some(Pattern::Marching { up })
}
