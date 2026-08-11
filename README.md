# nsc_trades

Reads forex charts, applies your trading rules, and sends you signals on
Telegram. **Version 1 does not place trades.** Trading comes in version 2.

## What it does, in one paragraph

Your rules are written down as settings. The bot reads the chart, finds setups
that match those rules, and sends them to you. Before sending, it checks a few
things — is there news coming, does a trained model think this is one of your
better setups, does an AI spot anything odd. Every signal it sends gets
tracked until it hits your stop or your target. You press 👍 or 👎 on each one.
Over time that builds a record of which setups you take and which win — and
that record is what teaches the model. The AI never invents a setup. It only
filters the ones your rules already found.

## The two rules that must never break

**1. Two crates never touch the outside world.**

`nsc-ta` and `nsc-strategy` have no database, no internet, no clock. Numbers
in, answers out.

That is what lets the backtester and the live bot run the *same* code. The
moment they run different code, your backtest is describing something you are
not going to run — and you will not notice, because the mismatch always makes
backtests look better.

**2. Never use data the market hadn't printed yet.**

A swing high is not *known* to be a swing high until several candles later.
If any part of the system uses it earlier than that, the results are fiction.

This is why candles are fed through one at a time instead of being processed
all at once. See `crates/nsc-backtest/src/guards.rs`.

## Where everything lives

| Folder | What's in it |
|--------|--------------|
| `crates/nsc-core`     | The basic types everything else shares. |
| `crates/nsc-ta`       | Reads the chart. The most important crate. |
| `crates/nsc-strategy` | Your rules. |
| `crates/nsc-data`     | Broker connection, database, downloading history. |
| `crates/nsc-risk`     | Position size, exposure limits, losing-streak brakes. |
| `crates/nsc-news`     | Economic calendar and headlines. |
| `crates/nsc-ai`       | The trained model and the AI second opinion. |
| `crates/nsc-chart`    | Draws the chart image for Telegram. |
| `crates/nsc-telegram` | Sends signals, collects your 👍/👎. |
| `crates/nsc-backtest` | Replays history, tests settings, reports results. |
| `crates/nsc-live`     | The bot that actually runs. |
| `crates/nsc-api`      | Web endpoints: Telegram callbacks and admin. |
| `config/`             | Your settings. Change these without rebuilding. |
| `ci/`                 | The checks that run on every push. |
| `migrations/`         | Database tables. |
| `docs/`               | Start with `strategy-worksheet.md`. |
| `research/`           | Python, for training the model. Phase 4. Offline only. |

## The build order

- **Phase 0** — download prices, store them, build the backtester
- **Phase 1** — read the chart: swings, levels, trend, Fibonacci, candlesticks
- **Phase 2** — your rules, signals to Telegram, no trading
- **Phase 3** — run it live for 6–8 weeks and press 👍/👎 on everything
- **Phase 4** — train a model on what you collected
- **Phase 5** — news filter and AI second opinion
- **Phase 6 (v2)** — actually place trades, crypto first

See `docs/phases.md` for how you know each phase is finished.

## Running it

```sh
cp .env.example .env      # fill in broker, Telegram and database details
docker compose up -d      # starts Postgres and Redis
cargo run -p nsc-data --bin backfill   # download history — nothing works without this
cargo run -p nsc-backtest              # check your rules against history
cargo run -p nsc-live                  # only once the backtest makes sense
```

Note: the last three commands are the Phase 0 target. They are not built yet.

## Before you push

```sh
./ci/rules.sh                                          # under a second, needs nothing installed
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

GitHub runs exactly these on every push and pull request. `ci/rules.sh` is the
project's own nine checks — a clean crate gaining a database, a pip number
baked into code, a file grown past 250 lines. `ci/README.txt` says what each
one is for and what breaks without it.
