# Map — where everything lives

A navigation file. When you want to change something and cannot remember
where it goes, start here.

---

## "I want to set a rule" — which file?

| A rule like… | Goes in |
|---|---|
| "Only buy when the daily is bullish" | `config/strategies/<name>.toml` → `[bias]` |
| "Price has to be at a level I'd have marked" | `config/strategies/<name>.toml` → `[location]` |
| "I need an engulfing candle to confirm" | `config/strategies/<name>.toml` → `[trigger]` |
| "Stop goes past the swing, with a bit of room" | `config/strategies/<name>.toml` → `[invalidation]` |
| "Minimum 2:1 or I don't take it" | `config/strategies/<name>.toml` → `[target]` |
| "Nothing after Friday lunchtime" | `config/strategies/<name>.toml` → `[veto]` |
| "Stay out around big news" | `config/strategies/<name>.toml` → `[veto]` |
| "Nothing at all on Mondays and Tuesdays" | `config/strategy.toml` → `[trading_window]` |
| "Five signals a day maximum" | `config/strategy.toml` → `[limits]` |
| "Don't send me the same idea twice" | `config/strategy.toml` → `[deduplication]` |
| "If two strategies disagree, do nothing" | `config/strategy.toml` → `[conflict]` |
| "Which strategies are switched on" | `config/strategy.toml` → `[registry]` |
| "Risk 1% per trade" | `config/risk.toml` → `[account]` |
| "One position per pair, two per correlated group" | `config/risk.toml` → `[exposure]` |
| "Stop me after four losses in a row" | `config/risk.toml` → `[brakes]` |
| "Which instruments to watch" | `config/symbols.toml` |
| "Skip this pair if the spread is too wide" | `config/symbols.toml` → `max_spread_pips` |
| "Which timeframes exist and which ones fire" | `config/app.toml` → `[engine]`, `[schedule]` |
| "When the trading day and week end" | `config/app.toml` → `[engine]` |
| "How big a swing has to be before it counts" | `config/ta.toml` → `[swings]` |
| "What makes a higher high real" | `config/ta.toml` → `[structure]` |
| "How many touches makes a level worth trading" | `config/strategy.toml` → `[levels]` |
| "How close counts as *at* the level" | `config/ta.toml` → `[proximity]` |
| "Which parts of the system are switched on" | `config/app.toml` → `[features]` |
| "Send signals at 7pm" / "don't message me at night" | `config/app.toml` → `[delivery]` |

### The rule of thumb

- About **this setup** → the strategy file.
- About **you and your account** — how much, how often, when to stop → `risk.toml`.
- About **the instrument** → `symbols.toml`.
- About **what the bot can see on a chart** → `ta.toml`.
- About **more than one strategy at once** → `strategy.toml`.

If a rule seems to fit two files, it belongs in the narrower one. A rule in
`risk.toml` applies to everything forever; a rule in a strategy file applies
only to that setup. Putting a setup-specific rule in `risk.toml` silently
changes strategies you were not thinking about.

---

## `config/` — everything a trader tunes

| File | What it controls |
|---|---|
| `app.toml` | Which timeframes exist, which fire signals, when the day and week end, which features are on. |
| `symbols.toml` | Which instruments, their pip size, spread limits, sessions, correlation groups. Broker-neutral. |
| `brokers/ibkr.toml` | How those names map to real IBKR contracts, plus connection, pacing and reconnect. The only file outside `nsc-data::sources` that mentions IBKR. |
| `strategy.toml` | The registry. Which strategies are on, what happens when two fire at once, daily limits. **No trading rules.** |
| `strategies/reversal.toml` | Catching a turn once the trend runs out of strength. The only one that trades against the bigger trend. |
| `strategies/breakout.toml` | Price escapes a range and you go with it. |
| `strategies/trend.toml` | The trend is running; you buy the pullback. |
| `risk.toml` | Position size, exposure limits, losing-streak brakes. |
| `ta.toml` | How sensitive the chart reading is. |

Two things to know about `config/`:

**`ta.toml` swing sensitivity is the most influential setting in the project.**
Every level, trendline, Fibonacci anchor and trend reading is built from swing
points. Change it and *everything* downstream changes. Never nudge it because
one chart looks nicer.

Those settings are all **shares of a move** rather than distances — how much of
a run gets given back, how big a run is next to recent ones. That is why one
set of numbers works on the 4-hour and the daily, and on gold and EURUSD.
`worksheets/swings.md` says where each came from.

**Strategy settings start commented out, and that is on purpose.** Commented
out means "not decided yet", and a strategy with undecided settings refuses to
load. A file full of sensible-looking defaults is dangerous — you read `2.0`,
assume somebody chose it, and end up backtesting a stranger's strategy.

---

## `docs/` — why things are the way they are

| File | What it's for |
|---|---|
| `map.md` | This file. Where everything lives. |
| `strategy-worksheet.md` | The six questions every strategy answers. An index to the per-strategy worksheets. |
| `worksheets/reversal.md` | Your reversal rules, in words. Becomes `strategies/reversal.toml`. |
| `worksheets/breakout.md` | Your breakout rules, in words. Becomes `strategies/breakout.toml`. |
| `worksheets/trend.md` | Your trend rules, in words. Becomes `strategies/trend.toml`. |
| `worksheets/levels.md` | How you draw support and resistance, and which round numbers matter. Becomes `nsc-ta::levels`, not a strategy file. |
| `worksheets/swings.md` | How a peak proves itself — the pullback, not a count of candles. Becomes `nsc-ta::swings`. **Read before touching that folder.** |
| `worksheets/structure.md` | When a higher high is really a higher high, and what happens to the pushes that fail. Becomes `nsc-ta::structure`. |
| `worksheets/candles.md` | The six candlestick patterns used, with textbook measurements. Becomes `nsc-ta::candles`. |
| `worksheets/fibonacci.md` | The four levels used. Deliberately thin — what each one is FOR is not captured yet. |
| `worksheets/to-collect.md` | Screenshots still needed to pin down vague rules. Grows as each strategy is worked through. |
| `diagrams/` | Pictures built to settle a question — swings, levels, structure. Live links in [diagrams/README.md](diagrams/README.md). |
| `architecture.md` | Why the code is split up the way it is. Read before moving code between crates. |
| `phases.md` | What gets built when, and how you know a phase is done. |
| `pitfalls.md` | The ways this kind of system breaks without telling you. Read before believing any result. |

**Words before numbers.** A worksheet gets answered in plain English first,
and the config file is the translation. Rules written straight into settings
look precise and usually mean nothing.

---

## `crates/` — the code

Twelve crates. The important thing is not what they do but **who is allowed to
use whom**:

```
nsc-core  ←  nsc-ta  ←  nsc-strategy  ←  nsc-live / nsc-backtest
```

The clean crates never reach down into the messy ones. If `nsc-ta` needs a
database row, you hand it the row.

| Crate | What it does | Touches the outside world? |
|---|---|---|
| `nsc-core` | The shared vocabulary — candle, level, swing, signal, timeframe. | No |
| `nsc-ta` | Reading the chart — swings, levels, trendlines, Fibonacci, patterns, indicators. | **Never** |
| `nsc-strategy` | Your rules, in six layers. The only place rules are applied. | **Never** |
| `nsc-data` | Brokers, the database, and checking the data is sound. | Yes |
| `nsc-risk` | Position size, exposure, brakes. | No |
| `nsc-news` | Economic calendar, headlines, news blackouts. | Yes |
| `nsc-ai` | The checking layer. May warn, lower confidence or block — never approve. | Yes |
| `nsc-chart` | Turning a setup into a picture. | No |
| `nsc-telegram` | Sending signals and collecting your verdict. | Yes |
| `nsc-live` | The bot. | Yes |
| `nsc-backtest` | Replaying history and testing settings. | Yes |
| `nsc-api` | The web endpoints. Never applies rules. | Yes |

**`nsc-ta` and `nsc-strategy` never touch the outside world.** No database, no
internet, no async, no reading the clock. That is what lets the backtester and
the live bot run the *same* code. If either gains `tokio`, `sqlx` or `reqwest`
in its `Cargo.toml`, the change is wrong.

**The one meeting point** is `BarClosed` in `nsc-data::events`. The backtester
and the live bot both go through it.

---

## Everything else

| Where | What |
|---|---|
| `migrations/` | Database tables. Seven of them — symbols, candles, signals, outcomes, labels, news, backtests. |
| `fixtures/` | Saved candle data and golden test files. |
| `research/` | Offline Python model training. Nothing here runs in production. |
| `deploy/` | Nginx config and server setup. |
| `.claude/skills/` | Task-specific guides — backtesting, data sources, migrations, debugging live, deploying, strategy rules, TA primitives, testing, and more. |
| `CLAUDE.md` | The project rules. Overrides everything else. |

---

## Known gaps

Things referred to by name that have no file behind them yet.

**Session times are not defined anywhere.** `symbols.toml` says
`sessions = ["London", "NewYork"]` and the strategy files say
`blocked_sessions = ["Sydney"]`, but nothing in `config/` says when London
starts or ends. Those names currently point at nothing. Needs a file.

**No IBKR contracts are filled in.** `config/brokers/ibkr.toml` has the shape
but every contract is commented out, because a plausible guess is worse than a
blank here — the wrong contract does not error, it just downloads a different
instrument's history and everything looks fine.

**The daily close is wrong for the two indices.** `app.toml` sets one anchor
for everything: 17:00 New York, the forex convention. That is correct for the
majors, the metals and oil — they all settle there. It is wrong for US30 and
SPX500, which settle an hour earlier. Their daily candles will not match your
chart, so every daily level, trendline and Fibonacci anchor on those two is
built on the wrong boundaries. Needs a per-class anchor, which does not exist
yet. Until then, prefer H4 and below on US30 and SPX500 — and check it
yourself by putting one of our daily candles next to your chart's.

**Spread limits on the six non-forex instruments are guesses.**
`max_spread_pips` for the metals, indices and oil are typical values, not
measured ones. Watch a normal London morning and set them from what you see.

**No instrument has volume data.** Cash forex has none and CFDs have none, so
no rule anywhere — strategy files, `nsc-ta`, or the Phase 4 model — can use
it. Not a preference; it is not in the data.
