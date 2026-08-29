---
name: ta-primitive
description: Use when adding or changing anything in nsc-ta — naming a single candle, or what a run of two or three of them does. Also use when a shape the bot names does not match what he sees on his chart.
---

# Adding something to the chart-reading engine

`nsc-ta` is the most valuable crate and the most dangerous one to change.
Everything the bot says about a chart comes from here, and one loosened
threshold changes every signal downstream at once.

**It is smaller than it was.** On 29 August 2026 swings, chart patterns,
trendlines, Fibonacci and the indicators were all removed at his word: *"we
would only work with candlesticks and I will do my analysis and draw my levels
and send them."* What is left is two things.

```
candle/    what ONE candle is — 13 names
pattern/   what a RUN of two or three does — 8 shapes, 4 of them traded
```

## Before writing anything

Work out which of the two it belongs to. `pattern/` reads `candle/`; nothing
reads the other way. If it needs anything else — a level, a zone, a trend — it
does not belong in `nsc-ta` at all. It belongs in `nsc-strategy`, and it gets
handed what it needs as an argument.

**There is no trend in this project and that is deliberate.** A hammer and a
hanging man are the same candle; only what came before separates them. Without
swings, `nsc-ta` can honestly say `long lower wick` and nothing more. Say
exactly that rather than guessing.

## The four rules

**1. No outside world.** No database, no internet, no async, no clock. Candles
in, answers out. `Cargo.toml` has no `tokio` and no `reqwest`, so the compiler
enforces it rather than a person remembering.

**2. Never use data you did not have yet.** `ending_at` takes the candles up to
and including the one being judged. There is no argument that could let it see
forwards — the safety is the shape of the function, not a discipline.

**3. Measure in normal candles, never pips.** Every distance, size and
tolerance is a multiple of normal candle size. Write a pip number and it works
on the pair you tested and quietly stops working on the next one.

**4. The numbers live in `config/patterns.toml` and `config/candles.toml`.**
Anything a trader would tune goes in the settings file, never as a constant.

## Shape is not size, and size is the half that gets forgotten

**Every shape needs both tests.** A rule about proportions says one side won;
it says nothing about whether anything happened. A quiet candle in a dead hour
can be 90% body and move a fifth of a normal candle.

Measured 29 August 2026 over 270,000 candles: **39% of engulfings and 38% of
haramis reached less than one normal candle** before floors went in. `[push]`
had found the same hole a week earlier.

So each traded shape carries a reach floor, on whichever candle carries the
move:

```
push        min_push_reach      the push candle
engulfing   min_reach           the engulfing candle
harami      min_first_reach     the BIG first candle, never the small one
marching    none, on purpose    96% already clear one
```

**Marching has no floor and that is the point.** A setting that refuses almost
nothing is worse than no setting, because the next person believes it is
protecting them.

**Every size rule is a floor, and bigger is always fine.** Confirmed by him on
29 August. The only ceilings are where the shape *requires* smallness — the
harami's small candle, the pin's body and nose, the soldiers' wick against.
A test pins this; do not grow a ceiling on a floor.

**Reach, not body, for anything that reverses at a level.** A rejection has a
wick by nature. His own AUD/USD trigger of 25 August reached 2.47 normal
candles with a 37% body, and a "mostly body" test would have thrown away the
exact candle he pointed at.

## Detectors report, they do not decide

A detector says *what it found*. It never says whether to trade it.

An engulfing is a fact. Whether it is worth anything depends on where it
printed, and that is `nsc-strategy`'s call. Mixing the two is how the rules
stop living in the rules crate.

**`ending_at` returns ONE pattern, longest first.** Three candles beat two, and
his own push beats the textbook ones because it is the stricter statement.
Reporting two names for one run would let a backtest count one setup twice.

## Refusing a shape does not silence it

**A shape that fails its size floor comes back named something weaker.** The
two-candle detectors are tried in order — engulfing, piercing, harami, tweezer
— and `tweezer` has no size test at all. So an engulfing refused for being
small can still be reported as a tweezer top.

That costs nothing today because rung 3 does not trade tweezers. It is pinned
in a test so nobody discovers it by surprise the day one is added.

## Testing — genuinely not optional here

Every addition needs a test on candles that **actually printed**. The fixtures
in `pattern/tests/runs/` are real runs off his charts, and the README there
says so — do not put invented candles in that folder.

Made-up candles are fine for showing what a rule *does*. They are worthless as
evidence that the rule is any good, because they were drawn to make the point.

The check that matters most:

> **A test must fail without the fix.** Check that it does. A test that passes
> either way pins nothing.

A neat way to test a size floor without inventing candles: take a real shape
and judge it against a **bigger normal candle**. Nothing about the shape
changes; only whether anything happened. That is exactly the half the floor
exists to ask about.

Also check that the same shape scores the same on EURUSD and on gold — which is
what proves the thresholds really are in normal candles and not pips in
disguise.

## When a shape does not match what he sees

He is the specification. If the bot names something he would not, or misses
something he would take, **the rule is wrong and the chart is right.**

Find a real example on his chart, measure it, and argue the change from what
the pattern *is*. Never from how many it yields — a low count is information,
not a problem to solve.
