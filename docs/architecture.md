# How the code is put together

## The rule that matters most

Two crates — `nsc-ta` and `nsc-strategy` — are not allowed to touch the
outside world. No database. No internet. No checking the clock. Numbers go
in, answers come out.

Here is why that matters.

The backtester and the live bot both run this code. Not a copy of it — the
same code. So when a backtest says "this setup would have won", it is talking
about the exact thing that will run on Monday.

The moment you write "if we're backtesting, do it this way instead", that
promise is gone. Your backtest now tests something different from what runs
live.

And you will not catch it by looking at the results. When these two drift
apart, the backtest looks **better**, not broken. That is what makes it
dangerous.

## Where the backtester and the live bot meet

```
backtester ─┐
            ├─→ "bar closed: EURUSD, H1, [prices]" ─→ everything else
live bot   ─┘
```

The backtester reads old candles from the database and feeds them through as
fast as the computer allows. The live bot gets new candles from your broker.

Everything downstream cannot tell which one is feeding it. It is not allowed
to find out.

## Which crate is allowed to use which

```
nsc-core                    just types. almost no dependencies.
   ↑
nsc-ta                      reads charts. no outside world.
   ↑
nsc-strategy                applies your rules. no outside world.
   ↑
nsc-backtest   nsc-live     the two things that drive it
                  ↑
      nsc-data nsc-risk nsc-news nsc-ai nsc-chart nsc-telegram
```

Read it bottom to top. The crates at the bottom talk to databases, brokers
and Telegram. The ones at the top do not, and are not allowed to.

If `nsc-ta` ever needs a row from the database, something else fetches that
row and hands it over. `nsc-ta` never reaches down and gets it itself.

## Why Rust and not Python

Not because the live bot needs to be fast. A bot watching 1-hour candles has
an hour to think. Speed is irrelevant there.

It is the testing.

You will not run one backtest. You will run thousands — trying different
swing sensitivities, different stop sizes, different targets, across 20 pairs
and 5 years of candles.

In Python each of those is a coffee break. In Rust it is seconds. That gap
decides whether you actually explore your options or just guess at them.
Testing speed is the real bottleneck in this project, not trading speed.

Python shows up once, offline, in `research/`, to train the scoring model in
Phase 4. It never touches the live bot.
