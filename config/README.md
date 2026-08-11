# config/

Your settings live here, not in the code. You will change these hundreds of
times while testing, and rebuilding the program for each tweak would be
unbearable.

| File | What it controls |
|------|------------------|
| `app.toml`      | What the bot watches, how often, and which parts are switched on. |
| `symbols.toml`  | Which pairs to watch and the facts about each one. |
| `strategy.toml` | Which strategies are on, and what happens when two of them fire at once. No trading rules live here. |
| `strategies/`   | **Your trading rules.** One file per strategy. Read these first. |
| `risk.toml`     | Position size, exposure limits, losing-streak brakes. |
| `ta.toml`       | How sensitive the chart reading is. |

## `strategies/` — one file per strategy

| File | The trade |
|------|-----------|
| `reversal.toml` | Catching a turn, once the trend runs out of strength. The only one that trades against the bigger trend. |
| `breakout.toml` | Price escapes a range and you go with it. |
| `trend.toml`    | The trend is running; you buy the pullback. |

Each file has the same six layers: **direction, place, trigger, stop, target,
skip.** Three strategies in one file turn into one strategy with thirty
exceptions — and then you cannot switch one off, backtest one on its own, or
notice that two of them are quietly the same trade.

**Everything in these files starts commented out, and that is on purpose.** A
commented-out setting means "not decided yet", and a strategy with undecided
settings refuses to load.

Why not ship sensible defaults? Because you would read `2.0` in a config file
and assume somebody chose it. Nobody chose it. It came with the file, it
survived into your backtest, and now you are testing a stranger's strategy and
calling the results yours.

Write the words before the numbers. `docs/worksheets/` has one worksheet per
strategy, and those get answered first.

One warning about `ta.toml`: the swing sensitivity is the single most
influential number in the whole system. Every level, trendline, Fibonacci
anchor and trend reading is built from swing points. Change it and
**everything** downstream changes. Test it properly. Never nudge it casually.
