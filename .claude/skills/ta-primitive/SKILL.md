---
name: ta-primitive
description: Use when adding or changing anything in nsc-ta — swing detection, support and resistance, trendlines, Fibonacci, trend direction, candlestick or chart patterns, indicators. Also use when the levels or signals on a chart look wrong and the cause might be in the chart-reading code.
---

# Adding something to the chart-reading engine

`nsc-ta` is the most valuable crate and the most dangerous one to change.
Everything is built on swing points, so changing how swings are found moves
every level, every trendline, every Fibonacci anchor and every trend reading
all at once.

## Before writing anything

Work out which layer it belongs to:

```
candles → swings → levels / trendlines / fibonacci / trend → snapshot
```

If it needs something from further right, the design is wrong. If it needs
something from outside `nsc-ta` altogether, it belongs in `nsc-strategy`.

## The four rules

**1. No outside world.** No database, no internet, no async, no clock.
Candles in, answers out. If you need a database row, take it as an argument.

**2. Never use data you did not have yet.** Analysing candle 100, you may read
candles 1 to 100 and swings confirmed by candle 100. Nothing else. A swing at
candle 100 is not knowable until a few candles later, however obvious it looks
on the chart.

**3. Measure in ATR, never pips.** Every distance, size and tolerance is a
multiple of normal candle size. Write a pip number and it works on the pair
you tested and quietly stops working on the next one.

**4. Put the numbers in `config/ta.toml`.** Anything a trader would tune goes
in the settings file, not as a constant in the code.

## Detectors report, they do not decide

A detector says *what it found* and *how good it looks*. It never says whether
to trade it.

An engulfing candle is a fact. Whether it is worth trading depends on where it
happened, and that is `nsc-strategy`'s call. Mixing the two is how the rules
stop living in the rules crate.

## Testing — genuinely not optional here

Every addition needs a golden-file test, and one test matters more than the
rest:

> **One candle at a time must give the same answer as all at once.** Feed the
> candles in one by one, the way the live bot does, and the result must be
> identical to processing the whole series in one go.

Any difference means you used data from the future. This one check catches the
whole category of bug before the backtester is even involved.

Also check: no swing used before it was confirmed, unfinished candles ignored,
and identical setups on EURUSD and GBPJPY scoring the same — which is what
proves your thresholds really are in ATR and not pips in disguise.

## After changing swing detection

Every golden file downstream will change. That is expected.

**Read the diff before regenerating.** Ask which levels moved and whether you
agree with the new ones. An unread regeneration is a silent change to the part
of the system everything else depends on.

Then rerun the backtest and compare. A big change in results from a small
change in sensitivity means your strategy was fitted to the old setting — not
that the new setting is better.

## About chart patterns

Left until later on purpose. Trend direction plus levels plus Fibonacci plus
candlesticks is most of the edge for a fraction of the work.

When you do build them: match against the **sequence of swings**, never
against raw prices. Raw price matching ends in an unmaintainable pile of
special cases. And a pattern is only valid once its last swing is confirmed —
looking back, it was obvious several candles earlier, which is exactly the
trap.
