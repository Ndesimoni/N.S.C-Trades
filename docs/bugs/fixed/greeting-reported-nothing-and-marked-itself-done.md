# 🟠 The report of where price already stands never came

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

One card a session, the one he was waiting for. Nothing wrong was sent, so
🟠 rather than 🔴 — but it was the feature working exactly backwards.

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
