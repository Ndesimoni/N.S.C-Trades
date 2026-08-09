---
name: data-source
description: Use when adding or fixing a broker connection — OANDA, MetaTrader bridge, Twelve Data, CSV — or when candle timestamps, daily candle boundaries, spreads or missing candles look wrong.
---

# Connecting a broker

Every broker hides behind `nsc-data::source::MarketDataSource`. Broker details
must never escape `nsc-data::sources` — that is what makes your broker the one
decision in this project you can undo cheaply.

## What every connection must guarantee

- **Timestamps in UTC**, marking when the candle **started**. Storing the close
  time instead causes off-by-one-candle bugs that are painful to spot.
- **Unfinished candles flagged**, never quietly included.
- **Mid prices** for reading the chart, with the spread kept separately for the
  skip check.
- **Candle boundaries** matching `config/app.toml`, not the provider's default.

## The daily close will catch you out

In forex the day does not end at midnight UTC. It ends at the time set in
`config/app.toml` — usually 5pm New York. Providers disagree about this and
some let you specify it per request.

Get it wrong and the levels your bot draws will not match the levels on your
own chart. That destroys trust faster than a losing trade, and it looks like a
strategy problem when it is a clock problem.

## What each option costs you

**OANDA** — works on macOS with nothing extra, free with an account. Names
timeframes differently, so they get mapped. History comes in pages with a cap,
so downloading works in chunks. Set the daily close explicitly.

**MetaTrader bridge** — MT5's programming interface only runs on Windows, so
this talks to a script inside the terminal over a socket. That means a Windows
server and a second program to babysit, and the terminal must stay logged in.
Broker server time is usually not UTC and varies by broker — configure the
offset, never assume it. Worth it only if you specifically need your own
broker's exact prices.

**Twelve Data** — clean, works on macOS, costs money. Prices will not match
your broker exactly. Fine for higher-timeframe structure, not fine for
anything spread-sensitive.

**CSV** — the workhorse for building and testing, not a fallback. Input that
never changes is what makes the chart-reading tests worth anything.

## When things fail

The error kinds exist to answer one question: **retry, or give up?**

| What happened | What to do |
|---|---|
| timeout or connection dropped | retry, backing off |
| rate limited | retry slower, respect what they told you |
| bad API key | **give up.** Retrying forever looks exactly like a dead feed |
| unexpected response format | **give up and shout.** Guessing corrupts your history |

## After any work on the feed

Run `nsc-data::gaps`. Bad data does not fail loudly — a missing hour shifts a
swing, which shifts a level, which changes every signal after it, and the
backtest still finishes and prints a believable number.

And when the connection comes back, **fill in what you missed before carrying
on**. Resuming without filling the gap leaves a hole that affects analysis for
days with nothing reporting an error.
