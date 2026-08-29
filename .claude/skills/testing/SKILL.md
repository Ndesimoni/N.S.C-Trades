---
name: testing
description: Use when writing or fixing tests, working with golden files or saved candle data, or when a test fails after changing the chart-reading code.
---

# Testing

## The kinds of test

**Unit tests** — inside the crate, for logic that does not touch the outside
world. `nsc-ta` and `nsc-strategy` should be full of these. They need no
database, no broker and no async runtime, which is the whole payoff for
keeping those crates clean.

**Golden files** — `crates/nsc-ta/tests/golden.rs`. Feed in saved candles,
compare against a saved answer.

**Rule tests** — `crates/nsc-strategy/tests/rules.rs`, using made-up snapshots.
A snapshot takes a few lines to build, so every layer can be tested on its own
without a single real candle.

## The test that matters most

> **One candle at a time must give the same answer as all at once.** Feed
> candles in one by one, the way the live bot does, and the result must be
> identical to processing the whole series in one go.

Any difference means you used data from the future. This one check catches the
whole category of bug before the backtester is involved, and it is cheaper
than every other defence.

## Saved candle data

Committed CSV files in `fixtures/candles/`, deliberately covering:

| File | What it proves |
|---|---|
| a trending quarter | the normal case works |
| a choppy range | simple support/resistance does not fall apart |
| a volatile pair | thresholds really are in ATR, not pips in disguise |
| a series with holes | the gap checker finds them |

A test fed by a live connection cannot prove anything, because its input
changes underneath it. That is why these files are committed.

## Golden files — regenerate deliberately

Change how a shape is detected and every golden file downstream changes. That is
expected.

**Read what changed. Ask which levels moved and whether you agree.**

An unread regeneration is a silent change to the part of the system everything
depends on — and it shows up as a green test suite.

## What to check

- No candle read before it closed
- Unfinished candles ignored completely
- The same setup on EURUSD and GBPJPY scores the same
- Each must-pass rule can reject on its own, and which layer rejected is
  recorded
- A nonsensical settings file fails when it **loads**, not on the first quiet
  week
- Adding a confluence never lowers the score

## Do not

- Do not fake the chart-reading engine in rule tests. Build a real snapshot —
  it is a few lines, and a faked one can describe a situation the engine could
  never actually produce, so the test passes for a setup that will never occur.
- Do not compare prices with exact equality. Allow a tolerance in pips.
