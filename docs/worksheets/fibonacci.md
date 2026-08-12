# Worksheet — Fibonacci

Captured 12 Aug 2026. Four levels, and each one has a different job — which is
why they could never be treated as one zone with lines in it.

---

## The levels you use, and what each is for

    0.382    a reading: the pullback was shallow, so the trend is strong
    0.5      ┐ the golden zone. The most important two. Where you look
    0.618    ┘ to get in.
    0.786    where you look to put stops — but not always

That is the whole list. Nothing else gets drawn.

### The golden zone is 0.5 to 0.618

Your words: those two are your golden levels, **the most important ones**, and
price sitting between them is what gets your attention.

So it is a zone, not two separate lines. The pair works together, and it is
where you look to get in.

### 0.786 is where stops go

Captured 12 Aug 2026. **Not always** — you look at other factors too, and the
level on its own does not place the stop.

That split matters for the code. `nsc-ta` draws the level; where the stop
actually goes is the invalidation layer in `nsc-strategy`, weighing this
against whatever else is on the chart. A stop placed by one line, every time,
is a stop everybody can see.

### 0.382 says the trend is strong

**It is not an entry level. It is a reading.**

A pullback that shallow means the market barely paused, and that is what a
strong trend looks like. So when price turns at 0.382 rather than reaching the
zone, the information is not "here is my entry" — it is "this move is
powerful".

A strong move often never reaches 0.5 at all, so waiting for the zone in a
market like that means standing aside for the best moves.

Which is why the same number is already the shallow swing threshold in
`swings.md`. One belief written once.

---

## What was wrong in the config

`config/ta.toml` said:

    retracement_zone = [0.382, 0.618]
    golden_pocket    = [0.618, 0.705]

The golden pocket was written as 0.618 to 0.705 — a band from a different
school of thought, where yours is 0.5 to 0.618. And 0.382 was sitting inside
the everyday zone, when it is really the strong-trend level.

Fixed on 12 Aug 2026 to `golden_zone = [0.5, 0.618]`,
`strong_trend_level = 0.382` and `deep_level = 0.786`.

`extensions = [1.272, 1.618]` is still the textbook pair and still unconfirmed
by you. It is marked as such in the file.

---

## Where this already had an effect

**0.382 is also the shallow swing threshold.** Settled 12 Aug 2026.

The swing rule needed a depth for its shallow case — a pullback that stops
short of half and then runs on. That is the same situation this level
describes: a strong move that barely pauses.

So it is one belief written once. If it ever changes, it changes in both
places, because it is the same number doing the same job. See `swings.md`.

---

## What each level does, in one table

| Level | What it is for | Which layer |
|---|---|---|
| 0.382 | reading trend strength from a shallow pullback | `nsc-ta`, then confidence |
| 0.5 – 0.618 | the golden zone — where you look to get in | the location layer |
| 0.786 | where you look to put a stop, not always | the invalidation layer |

Three different jobs, which is exactly why they could not be treated as one
zone with three lines in it.

---

## Built 12 Aug 2026

`nsc-ta::fibonacci` draws the four levels over the last completed leg — the two
most recent confirmed swings, which is the same run the swing finder measured
to confirm them. Thirteen tests, plus eight on the type.

Three settings checks came out of writing it down, and each catches a config
nobody would notice was wrong: the strong-trend level has to sit shallower than
the zone or it says nothing new, the stop level has to sit beyond the zone or
it would be hit by the entry it protects, and the shallow edge of the zone
comes first.

---

## Still open

1. **Which timeframe the move is measured on**, and what happens when a bigger
   move is still running inside a smaller one. Right now it measures whatever
   swings it is handed.
2. **Extensions.** `ta.toml` has the textbook 1.272 and 1.618 for targets and
   you have not confirmed them.
3. **What the other factors are** that decide whether the stop actually goes
   at 0.786.
4. Whether any of this differs by timeframe.
