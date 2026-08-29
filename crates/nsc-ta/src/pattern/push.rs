//! His own pattern: a push, then a pin that gets refused.
//!
//! **This is the only pattern in this folder that is not from a textbook.**
//! Say so out loud wherever it is discussed — the `nsc-` prefix does NOT mark
//! it, because that prefix is the house namespace and textbook patterns he has
//! adopted wear it too.
//!
//! Two candles:
//!
//! 1. **The push.** Mostly body, so one side plainly won, **and at least the
//!    size of a normal candle**. Shape alone is not momentum — a quiet candle
//!    in a dead hour can be nearly all body and mean nothing.
//! 2. **The pin.** A tail at least twice its own body, pointing **against**
//!    the push, with little or nothing at the other end — and **no bigger
//!    than the push**, because a pullback that covered more ground than the
//!    move it answers is not a pullback.
//!
//! **The tail must oppose the push, and that is the whole rule.** A tail
//! pointing the same way as the push is a different animal — that is the push
//! being refused, not the pullback. Ignore which way it points and this fires
//! on twice as much and means half as much.

use nsc_core::candle::Bar;
use rust_decimal::Decimal;

use super::body::Body;
use super::{Pattern, Rules};

/// A push, then a pin whose tail opposes it.
///
/// `normal` is how big a normal candle was **at that moment**. It is what
/// turns "mostly body" into "mostly body AND a real move".
pub(super) fn ending_at(
    first: &Bar,
    second: &Bar,
    normal: Decimal,
    rules: &Rules,
) -> Option<Pattern> {
    let push = Body::of(first);

    // ── candle 1: did one side plainly win, and was it a real move? ────────
    if push.share(first) < rules.push.min_push_body {
        return None;
    }

    // **Both halves of "momentum", and the second is the one that gets
    // forgotten.** Measured across five pairs on 21 August 2026, testing the
    // shape alone let through 67 pairs whose push was SMALLER than a normal
    // candle — 45% of everything found, the smallest a fifth of normal.
    if normal <= Decimal::ZERO || first.high - first.low < normal * rules.push.min_push_reach {
        return None;
    }

    // ── candle 2: is it a pin, and does its tail oppose the push? ──────────
    let range = second.high - second.low;

    if range <= Decimal::ZERO {
        return None;
    }

    // **A pullback that moved further than the push is not a pullback.** The
    // pin answers the push; if it covers more ground than the move it is
    // answering, the push was the smaller event.
    //
    // **This is a size test, not an engulfing one.** The pin may still poke
    // past the push's high or low — on a continuation setup, price making a
    // new extreme and being thrown back is the strength, not a fault. Insist
    // on the push COVERING the pin and the clearest example he has, gold's
    // daily on 20 August 2026, is refused for a high 16.70 points too far.
    if range > (first.high - first.low) * rules.push.max_pin_of_push {
        return None;
    }

    let pin = Body::of(second);
    let body = pin.size();

    // **A third is the ceiling, and it is arithmetic rather than taste.** With
    // no nose the body and the tail are the whole candle, so a tail of twice
    // the body leaves the body at most one third. Setting this above a third
    // would admit nothing the tail test does not already refuse.
    if body > range * rules.push.max_pin_body {
        return None;
    }

    // Push up, the tail points down. Push down, it points up.
    let (tail, nose) = if push.up {
        (pin.bottom - second.low, second.high - pin.top)
    } else {
        (second.high - pin.top, pin.bottom - second.low)
    };

    if tail < body * rules.push.min_tail_of_body {
        return None;
    }

    // **What keeps indecision out.** A small body with real wick on BOTH sides
    // is a spinning top — nobody won — and it means close to the opposite of a
    // refusal. He struck those out by name, twice, while this was being drawn.
    if nose > range * rules.push.max_nose {
        return None;
    }

    Some(Pattern::Push { up: push.up })
}
