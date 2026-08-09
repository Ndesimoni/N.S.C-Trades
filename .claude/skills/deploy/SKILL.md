---
name: deploy
description: Use when deploying to the server, setting the server up, configuring Nginx or the Telegram webhook, or preparing a release.
---

# Deploying

## Two forex-specific things people get wrong

**1. The server clock must be UTC.** Both the machine and the container. Every
candle timestamp is UTC and the daily close is applied deliberately in code. A
server on local time shifts your daily candles, therefore your levels — and it
looks like a strategy problem rather than a server problem.

**2. Put the server near your broker.** Not for speed — version 1 places no
trades — but a distant server reconnects more often, and every reconnect is a
chance to miss candles.

## Order of operations on a fresh machine

```sh
docker compose up -d              # postgres and redis
sqlx migrate run                  # create the tables
cargo run -p nsc-data --bin backfill   # download history
# check for gaps before going further
cargo run -p nsc-backtest         # sanity check against real data
# only now:
systemctl start nsc-live nsc-api
```

Download history before anything else. A bot started against an empty database
will happily analyse thirty candles and produce nonsense.

## Nginx

HTTPS is genuinely required — Telegram refuses to deliver to a plain http
address.

The webhook must check the secret header **inside the application**, not only
at the proxy. Anyone who finds the URL can otherwise send fake button presses
and poison your training data: silent, cumulative, and impossible to undo once
mixed in.

The admin pages are locked to your own IP. They change what the bot does, and
the backtest trigger will use every core on the machine.

## Changing settings

Restart. Never reload while running.

Changing rules under a running bot means signals within one session came from
different rules, and no analysis afterwards can untangle which. **The restart
is how you know.**

Bump `[meta] version` in the strategy file whenever behaviour changes.

## The feature switches are phase gates

The switches in `config/app.toml` exist so parts come online one at a time.
Turning on the news filter and the AI check together means you cannot tell
which one changed your results. Switch one on, watch it, then the next.

`execution` stays off. It is not built, and it is not a gap waiting to be
helpfully filled in.

## After deploying

Watch `/health` until every pair reports a recent candle. Then wait for the
first signal and check the whole path: the message, the chart image, and that
pressing 👍 actually saves a row.

Do that last check every time. A broken feedback button is invisible — signals
keep arriving, everything looks fine, and the dataset Phase 4 depends on
quietly stops growing.

## Backups

Dump the database on a schedule, and restore it at least once to prove the
backup works.

`signals`, `signal_outcomes` and `signal_labels` cannot be regenerated. They
recorded decisions made in the moment. Losing them costs months, not hours.
