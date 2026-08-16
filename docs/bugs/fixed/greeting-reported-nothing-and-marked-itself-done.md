# 🔴 The report of where price stands never came

**Where** `watch/line.rs`, `watch/resumed/awake.rs`
**Found** 16 Aug 2026 · **Fixed** 16 Aug 2026

## What happens

The session opens. Price is already sitting in a zone. The card saying so
never arrives — not late, never, for the whole session.

The greeting asks which zones price is resting in *before* the price is
recorded. It finds nothing, sends nothing, and marks the session done.

## Fix

Feed the price in first, and skip any pair no price has arrived for. Both, so
it does not depend on two lines staying in order.
