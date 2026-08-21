# Where IBKR puts its candle boundaries

Measured **20 August 2026**, market open, against live IBKR paper data.
Run it yourself: `cargo run -p nsc-work-man --bin candles -- XAU/USD`

Nothing here was read off documentation. Every number comes from matching a
big candle's **open price** against the opens of the candles one step below
it — the same tick, written down twice — so the smaller candle that shares the
number is where the bigger one began.

---

## The answer

| | starts | which is | how sure |
|---|---|---|---|
| **XAU/USD** day | **22:00 UTC** | 17:00 **Chicago** | 6 of 6 candles agree |
| **EUR/USD** day | 21:15 UTC | 17:15 **New York** | **2 of 6** — not settled |
| both, week | **Monday** | Sunday evening at the roll above | 6 of 6 agree |

---

## `config/when.toml` does not match either of them

It says the trading day ends **17:00 America/New_York**. In August that is
**21:00 UTC**.

```
    what the setting says      21:00 UTC
    what gold actually does    22:00 UTC     one hour later
    what EUR/USD does          21:15 UTC     fifteen minutes later
```

**Nothing errors when this is wrong.** The candles come back perfectly well.
They are simply not the candles on his chart — a different open, a different
high, a different range. Band thickness is 0.46 of a normal daily candle, so
every daily band changes size, every daily level moves, and the first thing he
would notice is an alert firing in a place he did not draw.

## And one setting cannot serve both

This is the part that is not a config edit.

Gold rolls on the **metals** clock and EUR/USD on the **forex** clock, and
they are 45 minutes apart. `when.toml` has a single `day_ends`, so whichever
number goes in it is wrong for the other instrument.

Two ways out, and neither has been chosen:

1. **Per-pair.** `config/pairs/*.toml` already exists and already carries
   per-pair overrides — `approach_pips` does exactly this. `day_ends` could
   join it, with `when.toml` holding the default.
2. **Stop deriving it.** The bot never works a boundary out for itself
   anyway — it reads the feed's own stamps. If nothing downstream actually
   needs `day_ends` for a candle, the setting only governs the CALENDAR (which
   day is silent, when the heartbeat is due) and can stay on New York.

**Which of those is right depends on what reads `day_ends`, and that has not
been traced yet.**

---

## What is NOT proven

- **EUR/USD is not settled.** Only 2 of 6 daily candles could vote — the other
  four opened on a price that two or three hourly candles shared, which is
  ordinary in a quiet hour on a five-decimal pair. Two agreeing is not a
  measurement. 21:15 UTC is what those two said, and it matches IBKR's
  published forex convention of 17:15 New York, but **it wants re-running on a
  busier stretch.**

- **Whether these hours move with the clock.** Every candle sampled falls in
  August 2026 — one side of the daylight-saving change. A fixed 22:00 UTC and
  a local 17:00 Chicago look identical in summer and are an hour apart in
  winter. **Re-run this in November.** Getting it wrong is a silent one-hour
  error for half the year, which is precisely the mistake `when.toml`'s own
  comments were written to prevent.

- **Only two instruments.** GBP/USD and USD/CAD are unmeasured. GBP/USD should
  match EUR/USD if the split really is forex-versus-metals.

---

## Two other things the run turned up

**A daily candle's stamp is its trading DATE, not its open time.** The candle
stamped `2026-08-20` began at 22:00 UTC on the 19th. So `Bar::opened_at()` on
a daily or weekly candle gives a time that is neither its open nor its close.
Nothing reads it today — `finished_by` is only used on the 1-hour and 4-hour —
but anything that starts drawing daily candles by their stamp will be wrong.

**Weekly candles are stamped with the FRIDAY, not the Monday.** The week that
opened Monday 17 August is stamped `2026-08-20`. Same trap, one level up.
