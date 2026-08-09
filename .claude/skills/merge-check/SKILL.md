---
name: merge-check
description: Use before merging a change — reviewing a diff, finishing a feature, or checking work against the specific ways this project breaks.
---

# Checking a change before it lands

General code review advice is not the point here. These are the ways **this**
system breaks, and every one of them is silent.

## Blocking — do not merge

**A clean crate got dirty.** `nsc-ta` or `nsc-strategy` gained a database,
internet, async, a clock call, or global state. Check the `Cargo.toml` diff
first — it is the fastest tell.

**Code that asks "am I backtesting?"** This breaks the one promise the whole
design exists to make.

**Using data from the future.** A swing used before it was confirmed. An
unfinished candle read. Reading candle 101 while analysing candle 100. A
bigger-timeframe candle handed out before its period ended.

**A hardcoded pip number** anywhere outside `config/`. It works on the pair
you tested and quietly stops working elsewhere.

**`unwrap`, `expect` or `panic!` in a library crate.** A settings sweep runs
this code over years of candles. One bad candle must not destroy hours of work.

**Secrets.** `.env`, keys, broker logins in the diff.

**Something failing quietly.** Especially in `nsc-news`. A calendar that fails
to load and returns an empty list switches off your news blocking while every
log line still looks normal.

## Worth asking before approving

- Does a new number belong in `config/` rather than the code?
- Do the error types tell the caller *retry or give up*, or are they lumped
  together?
- Are rejected setups recorded with the layer that rejected them, or dropped?
- If the chart-reading code changed: did anyone actually read the golden diffs,
  or just regenerate them?
- If rules changed: was `[meta] version` bumped?
- If the AI layer changed: can it now approve a setup, or still only block one?
- Can every signal this produces still be explained in one sentence?

## Mechanical checks

```sh
cargo fmt --all --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

## About backtest numbers in a pull request

A change claiming an improvement needs before and after **over the same period
with everything else identical**. Comparing across a change to the
chart-reading code is comparing two different systems.

Look for a patch of settings that work, not a single peak. And check the trade
count before reading any percentage.
