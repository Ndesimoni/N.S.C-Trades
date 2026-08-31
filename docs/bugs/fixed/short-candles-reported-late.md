# 🟠 A quarter of 4-hour candles were reported late — fixed 31 Aug 2026

**Where** `watch/closes/look.rs`, via `nsc-core::candle::Bar::finished_by`
**Found** 31 Aug 2026, hunting for bugs · **Fixed** the same day

## What happened

A candle was judged finished by a stopwatch:

```rust
opened_at + minutes <= now
```

That assumes every 4-hour candle lasts four hours. **A quarter of them do
not.**

IBKR ends its forex day at **17:15 New York** and prints short candles around
the boundary. Measured on 30,000 AUD/USD 4-hour candles:

```text
    240 minutes   21,446 candles
    165 minutes    2,832      -> reported 75 min late
     75 minutes    2,275      -> reported 165 min late
    105 minutes    1,419      -> reported 135 min late
    135 minutes    1,137      -> reported 105 min late
```

**7,675 of 30,000 — 25%.** The 1-hour has the same shape at 4%.

So the close card for those candles arrived **up to two and three-quarter
hours late**, twice a day, on every pair. Rung 3 with it.

## Why nobody saw it

**It was late, never early.** An early read would have been price the market
had not printed — the one mistake this project has a rule against, and the one
that would have been caught. Lateness has no symptom: the card arrives, the
words are right, the candle is real. Only the clock on the message was wrong,
and nobody was holding a stopwatch.

It also cannot be seen in the data. `data/history/*.csv` has the short candles
in it and always did; nothing about them looks unusual until you subtract two
adjacent stamps.

## The fix

**A later candle is the proof.** The feed does not open a candle until the one
before it is done, so anything with a candle after it has finished — the
broker's own answer, needing no arithmetic. That is the same principle the
whole project already rests on: *the broker's chart is the truth.*

Only the newest candle in a reply has nothing after it, and there the clock is
all there is. It stays conservative there, for the same reason as before:
better a candle reported late than one reported before it closed.

## What it did not affect

**Not the stored candles.** `--bin history` writes what the feed says; the
stamps were always right.

**Not the charts.** Those are drawn from what IBKR hands over.

**Not lookahead.** The error was always in the safe direction.

## Related, and still open

`config/when.toml` says the trading day ends at **17:00 New York**, measured on
Twelve Data. **IBKR ends it at 17:15** — the same measurement that turned this
bug up. That one is still unfixed and it shifts every band.
