---
name: debug-live
description: Use when the running bot misbehaves — no signals arriving, duplicate signals, levels that do not match your chart, stale data, or the bot has gone quiet.
---

# Working out what is wrong with the running bot

Start from this: **the usual failure is not a crash.** A crash is obvious and
gets restarted. The real failure is a program that keeps running while
something inside it stopped, and silence looks exactly like a quiet market.

## "The bot has gone quiet"

Check in this order — cheapest and most likely first.

**1. Is the feed alive?** `/health` shows the last candle received for each
pair. A stale timestamp on a weekday is the answer most of the time.

**2. Has a brake tripped?** A run of losses or hitting a daily limit pauses
signalling. It should have told you — if it did not, check the brake state in
Redis.

**3. Is it paused?** `/pause` survives restarts on purpose.

**4. Are setups being found and then blocked?** Look for recent rows with a
blocked status. If they exist, the engine is working and something is
filtering — the record says which layer.

**5. Are setups not being found at all?** Then it is the rules. Backtest the
last two weeks and look at which layer rejects most. The usual cause is rules
tightened one at a time until nothing gets through.

## "It sent the same signal twice"

Nearly always a restart between sending and saving. On restart the bot looks
at that candle again, finds no record of having sent it, and sends again.

Check that shutdown is saving pending work, and that the cooldown key actually
got written to Redis. The database has a uniqueness rule as a last defence —
if that fired, sending worked and saving did not.

## "The levels do not match my chart"

Almost always time, not analysis. In order:

1. Is the server clock UTC? Is the container set to UTC?
2. Does the daily close time match your broker's day?
3. Did the feed drop candles? Run the gap check for that period.

A missing hour shifts a swing, which shifts a level. Nothing errors.

## "It sent something obviously wrong"

Use `/why <signal_id>` for the full reasoning and everything the bot saw.

Then work out which of these it is:

- **The reasoning is sound and you disagree** → a rule is missing, usually a
  skip rule. Write it in the worksheet, then add it to the settings.
- **The reasoning is wrong** → a bug in the chart-reading code. Turn it into a
  golden-file test at that exact candle *before* fixing anything.

The second is rarer than it feels. Most "wrong" signals are your rules
faithfully doing something you never wrote down.

## Reproducing anything

Every signal saves its pair, timeframe and candle time. Replay to that candle
in the backtester and the engine must produce an identical setup.

If it does not, backtest and live have drifted apart. That is a blocking bug,
not a curiosity — it invalidates every backtest result you have.
