# Progress

Where the project actually is, as of **14 August 2026**.

Updated whenever a piece of work finishes — that is a rule in `CLAUDE.md`. A
progress file that is out of date is worse than none, because the next decision
gets made against it.

```
[x]  done
[~]  started, not finished
[ ]  not started
```

**245 tests passing · clippy clean · ~7,000 lines of real code**

One thing to know before reading: **most of the work so far is Phase 1, not
Phase 0.** That was deliberate — reading a chart is the part that could have
gone wrong. Storage and replay are plumbing.

---

## Phase 0 — the boring foundations

*Finished when you can replay a year of 15-minute candles and the lookahead
checks stay quiet.*

### Download price history — [~]

- [x] cTrader exporter, and the trailing-newline bug that buried the files
- [x] CSV reader — any column order, UTC, 10 tests
- [x] 62,384 real gold 15-minute candles, Dec 2023 to Aug 2026 — no
      impossible candles, spacing all accounted for
- [ ] Broker connection — no IBKR contracts filled in, nothing connects
- [ ] `backfill.rs` — automatic download
- [ ] `gaps.rs` — spotting holes that are not weekends

Exports are done by hand. Enough for now.

### Store it — [ ]

- [x] 7 migrations written
- [ ] Migrations have never been run
- [ ] `store/candles.rs` — empty
- [ ] `store/signals.rs` · `outcomes.rs` · `labels.rs` · `backtests.rs` — empty
- [ ] No database query anywhere in the codebase

Every run re-reads a 4 MB CSV.

### Build the backtester — [ ]

- [ ] `events.rs` — `BarClosed`, the one place the backtester and bot meet
- [ ] `replay.rs` — walk a history one candle at a time
- [ ] `harness.rs` · `metrics.rs` · `report.rs` · `sweep.rs` — all empty
- [x] `bin/chart` — reads a file and prints everything the analysis sees

The chart tool does most of what replay needs. The gap is doing it **one candle
at a time with the guards watching**.

### Lookahead checks — [~]

Half done, and it is the half that matters.

- [x] `Swing::new` refuses a swing knowable before its own candle
- [x] `Level::new` refuses a level built from unconfirmed swings
- [x] Incomplete candles refused by ATR, swings and levels
- [x] The swing finder **cannot** emit an unconfirmed swing — none exists to misuse
- [x] Tested throughout
- [ ] `guards.rs` — the run-level watcher that kills a whole backtest

Nothing to guard yet, because nothing replays.

---

## Phase 1 — reading the chart

Nearly finished, and well ahead of Phase 0.

### nsc-core — the shared types — [x]

- [x] `error.rs` · `price/` · `timeframe/` · `symbol/` · `candle/`
- [x] `swing/` · `level/` · `structure/` · `fib/` · `pattern/`
- [ ] `session.rs` — London, New York, Tokyo, Sydney. Still empty.
- [ ] `signal.rs` — a finished trade idea. Waits on Phase 2.
- [ ] `trendline.rs`

### nsc-ta — the chart reading — [~]

- [x] `swings/` — the run-and-pullback rule, no candle counting
- [x] `levels/` — bands, across timeframes, crowding and covering
- [x] `structure/` — higher highs with follow-through
- [x] `candles/` — pin bar, engulfing, doji, belt-hold, tweezers, star, inside bar
- [x] `fibonacci/` — the four levels and the golden zone
- [x] `indicators/atr/` — Wilder's smoothing, matches TradingView
- [x] `aggregate/` — proven against the broker's own daily candles
- [x] `config/` — every setting, checked at startup
- [ ] `snapshot.rs` — everything about one moment, in one object
- [ ] `context.rs` — what a timeframe hands down to smaller ones
- [ ] `trendlines.rs`
- [ ] `patterns/` — double, flag, head and shoulders, triangle. Deferred on purpose.
- [ ] `indicators/moving_average.rs` · `rsi.rs`

---

## Phase 2 — your rules — [ ]

- [x] `config/strategies/` — reversal, breakout and trend files, all settings
      commented out until answered
- [~] `docs/worksheets/` — swings, levels, candles, structure and fibonacci have
      real answers in them; reversal is part done; breakout and trend are empty
- [ ] `nsc-strategy` — 11 lines. Nothing built.
- [ ] `nsc-telegram` — nothing built

---

## Phases 3 to 6 — [ ]

Collecting labels, the model, news and AI checks, and trading. Nothing started,
and nothing should be.

---

## Work that is not in any phase

### Levels drawn by hand — [~]

- [x] `config/levels/XAUUSD.toml` — 6 weekly and 2 daily, with the date drawn
- [x] `nsc-data/levels/` — reads the file into real `Level` values, 8 tests
- [x] `Level` carries an `Origin` — `Found` or `DrawnByHand`
- [x] A hand-drawn level has **no touch count**. Asking gives `None`, not a
      made-up number that would poison every later comparison.
- [x] `from` enforced — a level does not exist before the day it was drawn
- [x] Nothing thinned. The crowding and covering rules do not run on his
      levels, because he already thins them while drawing.
- [x] `bin/chart --levels config/levels/XAUUSD.toml`
- [x] `config/levels/USDCAD.toml` — 4 weekly and 2 daily
- [x] Band thickness **settled**: `drawn_weekly_atr = 0.35`,
      `drawn_daily_atr = 0.60`, same on every instrument
- [x] Levels hold **one price**, not a band. The band is worked out from a
      normal candle when the level loads.
- [ ] The other 50-odd pairs

Why one price: he draws with one pen width, so the band looks identical on
every chart. That reads as consistent but cannot be computed — a bot has no
screen. A share of a normal candle can be, so that is what is built. Also
practical: reading one centre line off a screenshot is accurate to about 3
points, reading two edges was accurate to about 15.

Decided along the way: the bot trades **your** levels, not found ones. The
finder stays only as something to score against them.

Why: the finder was tried against his own gold chart. The best any setting
managed was four of his eight, and three could never be found at all. His
levels are where a big move **ended**; the finder looks for prices where swings
**cluster**. Different definitions, and no band width bridges them.

### Recording your decisions — [ ]

- [ ] `snapshot.rs`, then a decisions file — your words plus what the code saw
      at that exact moment
- [ ] Pattern-finding across them. **Not until there are hundreds** — anything
      found in a handful of examples is a coincidence.

### Settings — [x]

- [x] `app.toml` · `symbols.toml` (13 instruments) · `ta.toml` · `risk.toml`
- [x] `strategy.toml` and the three strategy files
- [x] `brokers/ibkr.toml` — shape written, contracts still blank

### Documents and pictures — [x]

- [x] `docs/map.md` — which file a given change belongs in
- [x] 13 diagrams in `docs/diagrams/`, all with source in the repo and
      listed in its README
- [x] 9 worksheets in `docs/worksheets/`
- [x] 12 skills in `.claude/skills/`
- [x] `README.txt` in all 20 folders that have code

---

## What is left in Phase 0, in order

| | Size |
|---|---|
| `events.rs` — `BarClosed` | small |
| `replay.rs` — one candle at a time | medium |
| `guards.rs` — kill the run on lookahead | small |
| `gaps.rs` — holes that are not weekends | small |
| `store/candles.rs` — Postgres in and out | medium |
| Run the migrations | trivial |

Four of the six are small. Storage can come last — the CSV works and nothing is
blocked on the database.

**`snapshot.rs` sits outside this list** but is worth doing early, because it is
what turns each screenshot you send into a record that can be searched later.
