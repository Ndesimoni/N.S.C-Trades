# config/

Your settings live here, not in the code. You will change these hundreds of
times while testing, and rebuilding the program for each tweak would be
unbearable.

| File | What it controls |
|------|------------------|
| `app.toml`      | What the bot watches, how often, and which parts are switched on. |
| `symbols.toml`  | Which pairs to watch and the facts about each one. |
| `strategy.toml` | **Your trading rules.** The six layers. Read this first. |
| `risk.toml`     | Position size, exposure limits, losing-streak brakes. |
| `ta.toml`       | How sensitive the chart reading is. |

One warning about `ta.toml`: the swing sensitivity is the single most
influential number in the whole system. Every level, trendline, Fibonacci
anchor and trend reading is built from swing points. Change it and
**everything** downstream changes. Test it properly. Never nudge it casually.
