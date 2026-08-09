---
name: db-migration
description: Use when changing the database — adding tables or columns, changing types, adding indexes — or when writing queries in nsc-data::store.
---

# Changing the database

## Migrations

Numbered files in `migrations/`, applied in order. **Never edit one that has
already been applied** — write a new one instead. Editing an applied migration
means your development database and your live one quietly end up different,
and you find out weeks later when a query fails.

Add rather than remove. New columns that allow empty values, new tables. A
destructive change to `signals`, `signal_outcomes` or `signal_labels` destroys
training data you cannot recreate, because it recorded decisions you made in
the moment.

## What the tables are protecting

Every signal has to stay connected to what the market did and what you thought
of it. Those three things together — what the bot saw, what happened, what you
said — are the Phase 4 dataset. Treat those three tables as add-only.

## The features column

`signals.features` holds everything the bot saw at that moment, saved exactly
as it saw it.

Never work it out again later. Recalculating against updated chart-reading
code trains the model on inputs the live bot never actually produced. Nothing
detects that — both sides keep working and only the scores are wrong.

If the shape of what gets saved changes, add a version number to it. Old rows
stay valid at their old shape and the training script filters by version.

## Query rules

- SQL lives in `nsc-data::store` and nowhere else.
- Queries return proper types from `nsc-core`, never raw rows, so a table
  change stays inside one folder.
- Candle writes are safe to repeat, so running a download twice repairs your
  history instead of duplicating it.
- Backtest reads must **stream**, not load everything at once. A settings
  sweep runs the same period hundreds of times, and memory use decides whether
  sweeps are comfortable or painful.

## Indexes

Add one with a note about which query it serves. The three that exist cover:
reading candles forward in time (backtest), recent signals by status
(dashboard), and stopping duplicates.

Before adding another, check an existing one does not already cover it.

## Timestamps

Always with a timezone, always UTC — including the `TZ` setting on the Docker
container. A server on local time shifts your daily candles and therefore your
levels, and it looks like a strategy problem.
