---
name: observability
description: Use when adding logging, metrics or health checks, or when deciding what a part of the system should report about itself.
---

# Logging and health

## The idea behind all of this

**Silence tells you nothing.** A quiet market and a dead feed look identical
from outside. Everything below follows from that.

## Which level to use

| Level | For |
|---|---|
| `error` | Something needs a person. Dead feed, unreadable calendar, tripped brake. |
| `warn` | Working, but not fully. AI check skipped, feed reconnected. |
| `info` | Signals sent, candles received per hour, jobs starting and stopping. |
| `debug` | Per-candle decisions. Switched on by environment variable. |

**A quiet market is not an error.** The most common mistake here is logging
"no setup found" as a failure. It is the normal outcome, by a huge margin.
Logging it as an error means thousands of lines a day, nobody reading the
logs, and a real problem going unnoticed.

## Log the decision, not the event

Useless: `"checked EURUSD H1"`.

Useful: `"EURUSD H1: no setup — place check failed, 1 of 2 needed (level yes,
harami yes), price 0.8 normal candles from the nearest level"`.

The second answers "why did nothing fire?" without rerunning anything. That
question comes up constantly in Phase 3, and it is the whole reason the
rejecting layer gets recorded on every blocked setup.

## "Alive" must mean "receiving candles"

A job reporting "still running" is not reporting health. The health job tracks
the **last candle received for each pair** and raises the alarm when a feed has
been quiet longer than the market can explain.

Handle weekends and holidays properly. An alert that fires every Saturday gets
muted, and a muted alert is the same as no alert.

## Use fields, not sentences

Log `symbol`, `timeframe`, `bar_time`, `signal_id` as separate fields. You
will want to filter by signal id when working out what happened six weeks ago,
and that is impossible if the id is buried in a sentence.

## Reference numbers

Every signal carries an id from the moment it is created through to delivery.
The web layer returns that id on errors, so a failure you see maps to a log
line without leaking internal details.

## What to count

Candles received per pair per hour, setups by which layer rejected them,
signals sent, delivery failures, AI checks skipped, and how far behind the
result tracker is.

The rejecting-layer count is the most useful number in the system. A sudden
shift in which layer rejects most is either the market changing or a bug —
and either way you want to know that week, not that quarter.
