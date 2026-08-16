# 🔴 The report of where price already stands never came

**Found** 16 August 2026, checking what would run that evening.
**Fixed** 16 August 2026 — `watch/line.rs` and `watch/resumed/awake.rs`.

## What he would have seen

The session opens. He waits for the card saying which zones price is already
sitting in — the whole reason the opening hours are held quiet.

Nothing arrives. Not late: never, for the entire session.

## Why

The greeting reports which zones price is **resting in**. Nothing is resting
anywhere until a price has been fed in — a fresh watcher has every band down
as *Away* and no last price at all.

It was asked **first**, before the price was recorded. On the very first price
of a session it found nothing, sent nothing, and **marked the session as
reported**.

## What it cost

**Silence that lied.** Price sits in his zone as the session opens, nothing
arrives, and silence in this bot means *nothing is near your levels*. He would
have believed it. That is the third 🔴 case — he stops being able to trust the
quiet, and the quiet is the whole design.

Filed 🟠 at first, on the reasoning that no *wrong* message was sent. That was
the wrong test. **A message that should have come and did not is exactly as
misleading as a wrong one**, when the rule is that nothing arriving means
nothing happened.

## The fix

Two guards, deliberately.

1. `line.rs` feeds the price in before asking the greeting anything.
2. `awake.rs` skips any pair no price has arrived for.

Staying true should not depend on two lines staying in one order.

Two tests in `nsc-core` pin the fact underneath: nothing rests before the
first price, and the first price makes it rest without counting as an arrival.

## The lesson worth keeping

**Order is behaviour.** Two calls that look independent are not, when one of
them reads what the other writes — and nothing in either signature says so.
