# Worksheet — Fibonacci

**Started 12 Aug 2026. Deliberately incomplete.**

Only one thing is captured so far, written down now so it is not lost. The
rest comes when we work through Fibonacci properly.

---

## The levels you use

    0.382    when the trend is strong
    0.5      ┐ the zone that gets your attention
    0.618    ┘
    0.786    its own use, not described yet

That is the whole list. Nothing else gets drawn.

### 0.382 is the strong-trend level

Added 12 Aug 2026, after 0.382 had been struck off.

It is not a level you use all the time — it is the one that matters **when the
move is strong**. A powerful trend barely pauses. It turns back up shallow and
often never reaches 0.5 at all, so waiting for the zone in a market like that
means standing aside for the best moves.

So the depth is not only where price is. It also says something about how
strong the move is, which is why this level cannot simply be added to the zone
and forgotten.

### The golden zone is 0.5 to 0.618

Your words: those two are your golden levels, and **price sitting between them
is what gets your attention**.

So it is a zone, not two separate lines. The pair works together.

### 0.786 is on its own

The third level is used, for something different from the zone above. What
that is has not been described yet, and you will say when we work through
Fibonacci properly.

That is why nothing is being built. A level with no job attached is a line the
bot draws and nothing reads.

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

## Still open

Everything else. What each of the three levels is for, which move they get
drawn from, whether extensions are used for targets, and whether the levels
differ by timeframe.
