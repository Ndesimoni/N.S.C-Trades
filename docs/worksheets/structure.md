# Worksheet — Structure

When a higher high is really a higher high. Started 12 Aug 2026.

Swings are the points. Structure is what you read off them: higher highs and
higher lows, lower highs and lower lows, and the moment one turns into the
other.

This becomes `nsc-ta::structure`. It decides nothing about trading — it says
what the sequence is, and the rules decide what to do about it.

**The picture:** [when a higher high is really a higher high →](https://claude.ai/code/artifact/45d262ab-5f17-49e3-bbde-9b7d2a5e96f5) — the
rule, what it refuses, and the calls made inside the code.

---

## What you told me

### Option 1 — taking the high out is not enough on its own

Price has to **take out the previous high**, and then **make at least 40% to
50% of the previous run** before you agree a higher high is in.

You called this option 1, so there is at least one other way you would accept
it. That one is not captured yet.

### It is a floor, not a target

Confirmed 12 Aug 2026. 40 to 50% is the least that will do. Price running much
further past the old high is the same answer arrived at more convincingly — it
does not stop being a higher high for overshooting.

So in code it is one comparison, `>=`, and not a band to land inside.

Worth remembering later: **how far past it went is information**. A break that
carries 200% of the previous run is a different-strength event from one that
scrapes 45%, and the rules layer may well want to know which it was. That is a
confidence input for `nsc-strategy`, not a second threshold here.

### What that means with numbers

A run from 1900 to 2100 is a 200-point run. The old high is 2100.

Price pulls back, turns, and pushes through 2100. Under this rule that is not
yet a higher high — it has to carry on to somewhere between **2180 and 2200**,
which is 40% to 50% of that 200-point run past the old high.

Poke through by 5 points and stall? Not a higher high. The high was touched,
not taken.

### Where the measurement starts — confirmed

**From the old high, not from the pullback low.**

Your words: when it takes out the previous high, we want to see price make a
run of about 40–50% of the previous run — measured from the take-out.

That matters because the other reading would have been nearly useless.
Measured from the pullback low, the new leg has already covered 38–50% of the
run simply by climbing back to the old high, so the test would pass every time
it was asked. Measured from the old high, it asks for something real.

    previous run          L1 1900 ──► H1 2100          200 points
    take-out              price clears 2100
    what has to follow    2180 to 2200                 40–50% of 200

---

### Lower lows work the same way

Confirmed 12 Aug 2026. The rule is symmetrical, just upside down.

Price has to break the previous low, and then carry at least 40 to 50% of the
previous run below it, measured from the break.

One rule, applied in both directions — so it is one piece of code with a
direction passed in, not two detectors that could drift apart. A downtrend
being read by different rules from an uptrend is how a bot ends up bearish and
bullish about the same chart.

---

## Why this rule exists at all

A high that gets nudged by a few points and then fails is the most common trap
on a chart. It looks like a breakout, it brings in buyers, and price turns
straight back down.

Demanding real follow-through is what separates *price went above the line*
from *the market went somewhere*.

---

## The in-between case: record it as a failed attempt

Answered 12 Aug 2026.

Price crosses the old high, stalls at 20% of the run, and comes back under.
That is **not** a higher high — and it is not nothing either. It gets recorded
as a failed attempt.

Your reasoning, and it is the same reasoning as the rejected-setups rule in
CLAUDE.md: **there will be a time when a strategy uses those failed attempts
along with other factors.** They cannot be collected later. The chart does not
remember what nearly happened, so if the bot does not write it down when it
happens, that evidence is gone.

### What happens next at that same high

If price later comes back and does reach 40%, the high is taken as normal —
the failure did not use it up. And the point price reached becomes a swing high
in its own right once a retracement is made there, which is simply the swing
finder doing its job.

So one high can produce: a failed attempt, then a break, then a new swing.
All three are true and all three get recorded.

### What a failed attempt carries

Which extreme was attempted, the run behind it, **how far past it got**, when
the push started and when it was over.

How far it got is the number to sort them by later. One that reached 35% and
failed is a near miss. One that reached 5% barely happened, and lumping the two
together would waste the data.

**A failed attempt never moves the trend.** It is evidence, not a direction.

### When a push ends

Normally when a whole candle fails to get past the extreme at all — price is
back under, so the push is over.

It also ends if a **newer swing replaces the extreme being watched** before
price ever comes back under the old one. The push still failed, so it is
recorded then rather than dropped. Losing it because the chart moved on would
lose exactly the evidence these are collected for.

---

## Built 12 Aug 2026

`nsc-ta::structure` now reads this rule, and `ta.toml` holds one setting for
it: `min_follow_through = 0.4`. Fifteen tests, plus nine on the types.

`bos_atr_multiple = 0.3` is gone. It measured the same thing against normal
candle size, and one question with two settings is how they end up
disagreeing.

The fraction-of-the-run version fits the rest of the system better. Every other
rule you have given is relative to the move in front of you rather than to an
average of recent candles, and it is the same reasoning that made the run floor
relative in `swings.md`.

---

## Still open

1. **Option 2**, and any others — the other ways you would accept a higher
   high.
2. ~~Whether the same applies to lower lows~~ **ANSWERED 12 Aug 2026** — it
   does, exactly mirrored.
3. ~~What happens between — price takes the high but stalls at 20%~~
   **ANSWERED 12 Aug 2026** — see below.
