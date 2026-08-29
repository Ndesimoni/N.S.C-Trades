# The database

Designed 27 August 2026. **Nothing is built.** This is the shape, the reasons,
and the two decisions that need him.

---

## What it is for

Three questions the bot cannot answer today, and one it will need to.

| | |
|---|---|
| **"Why did nothing fire last week?"** | Nothing is written down. A quiet week and a broken bot look identical |
| **"Does the level actually save it?"** | Rung 3 is a test with nothing to measure it against |
| **"What happened after this setup?"** | Signals vanish the moment they are sent |
| **Phase 4** | A model needs the *don't take this* examples as much as the good ones |

**It is a record, not a cache.** What price is doing right now belongs in
memory. What the bot *decided*, and what the market did about it, belongs here
and can never be recreated.

---

## What must not change

- **`nsc-core` never touches it.** No `sqlx` in that manifest — it is the
  manifest that enforces this, not a rule anybody remembers. A rule that needs
  a row gets handed the row.
- **`nsc-strategy` never touches it either.** Same reason, and it is the one
  that would break the backtest if it did.
- **SQL lives in `nsc-data::store` and nowhere else.** Queries hand back
  `nsc-core` types, never raw rows, so a table change stays in one folder.
- **Everything `TIMESTAMPTZ`, always UTC** — including `TZ` on the container.
  A server on local time shifts the daily candle, which shifts every band,
  which changes every signal, and it looks like a strategy problem.
- **Never `float`.** `NUMERIC` everywhere a price appears. `0.1 + 0.2` is not
  `0.3` in floating point, and a level at 4520.00 that stores as 4519.9999998
  answers "did price touch it" with *no* while his eye says yes.

---

## The ten tables

```
  candles ─────────── the history everything else is measured against
  backfills ───────── what was downloaded, so a half-done one is visible

  levels ──────────── every level he EVER drew, with the day it appeared
                      └── the backtest asks: which existed on this date?

  signals ─────────── what the bot saw, and sent
    ├── signal_outcomes ── what the market did next
    └── signal_labels ──── what HE thought of it

  rejections ──────── what was refused, and by which layer
  news_events ─────── the calendar as it stood that week

  runs ────────────── when the bot was UP, and when the line was down
                      └── without it, silence has two causes and no answer

  rule_sets ───────── what a rules_version hash actually WAS
```

---

### 1. `candles`

The history. Years of it, read forward, streamed.

| column | type | |
|---|---|---|
| `symbol` | `TEXT` | `XAU/USD` — as `config/pairs` writes it |
| `interval` | `TEXT` | canonical, never the feed's spelling — see below |
| `opened_at` | `TIMESTAMPTZ` | **when it opened**, not when it ended |
| `open` `high` `low` `close` | `NUMERIC(18,8)` | |
| `tick_volume` | `BIGINT NULL` | |
| `source` | `TEXT` | `ibkr`. Two feeds disagree and you must be able to tell which one you have |

**Primary key `(symbol, interval, opened_at)`**, and that key *is* the index
the backtest reads on. Forward in time, one pair, one timeframe — exactly the
order the PK stores.

**Writes are `ON CONFLICT DO UPDATE`.** Running a download twice repairs the
history instead of duplicating it. That matters more than it sounds: a
backfill that dies halfway is the normal case, not the exception, and the fix
has to be "run it again".

**`interval` is stored canonically, not as the feed spells it.** This project
already paid for that once — the timeframe travelled around as a `&'static
str` and two spellings of the same thing became two different keys, so the
same candle reported twice. One spelling, written by `nsc-core`.

**`opened_at`, and it is not obvious.** On the old feed an hourly stamp was
the candle's *open* and a daily stamp was the date it *ended on*. That is
still unchecked on IBKR. Storing the open means the conversion happens once,
on the way in, where it can be tested.

---

### 2. `backfills`

What was downloaded, over what range, and whether it finished.

| column | type | |
|---|---|---|
| `symbol` `interval` | `TEXT` | |
| `from_at` `to_at` | `TIMESTAMPTZ` | the range asked for |
| `finished_at` | `TIMESTAMPTZ NULL` | null means it died partway |
| `candles_written` | `INTEGER` | |

**Why this is not just a query over `candles`.** A gap during market hours is
detectable — the trading calendar says when the market was open. But a gap you
*have not tried to fill* and a gap you tried and failed to fill look identical
in the candle table, and they need opposite responses.

**This is the table I am least sure earns its place.** If a completeness scan
over `candles` plus the calendar turns out to be fast enough and clear enough,
this one can go. Worth building the scan first and seeing.

---

### 3. `levels` — and this one is a lookahead trap

Every level he has ever drawn, with the day it appeared and the day it went.

| column | type | |
|---|---|---|
| `pair` | `TEXT` | |
| `timeframe` | `TEXT` | weekly / daily / h4 |
| `price` | `NUMERIC(18,8)` | |
| `drawn_at` | `TIMESTAMPTZ` | when it first appeared in `config/pairs` |
| `removed_at` | `TIMESTAMPTZ NULL` | null means still live |

**`config/pairs/*.toml` stays the source of truth for what is live now.** The
file is why a pair is watched; delete it and the pair stops. That model is
simple, inspectable, and he edits it from his phone. Nothing here changes it.

**This table is the HISTORY of that file, and it exists for one reason.**

> Backtesting rung 3 with today's levels is lookahead, one level up.

A level he drew in August 2026 did not exist in March. Replaying March's
candles against today's zones asks "would this have fired at a level I had not
drawn yet" — and like every lookahead mistake it does not error, it makes the
results look **better**.

Without this table there is no honest backtest of rung 3 at all, because rung
3 is *entirely* about where the level was.

**How rows appear:** the bot notices `config/pairs` changed — it already does
this, that is `watch/reload.rs` — and appends. Nothing is ever updated in
place except `removed_at`.

---

### 4. `signals`

What the bot saw and sent. **Add-only, forever.**

| column | type | |
|---|---|---|
| `id` | `BIGINT` identity | |
| `at` | `TIMESTAMPTZ` | when it fired |
| `symbol` `interval` | `TEXT` | |
| `candle_opened_at` | `TIMESTAMPTZ` | which candle completed the shape |
| `spans_from` | `TIMESTAMPTZ` | where the shape STARTS |
| `shape` | `TEXT` | `nsc-bull`, `bullish engulfing`, `head and shoulders`, … |
| `shape_kind` | `TEXT` | `candlestick` or `chart` |
| `band_timeframe` `band_price` | `TEXT` / `NUMERIC` | the zone it printed at |
| `placing` | `TEXT` | `inside` / `just above` / `just below` |
| `broke_out` | `BOOLEAN` | did the candle close outside the band |
| `sentence` | `TEXT` | the one line `reasons.rs` wrote, **as it was sent** |
| `features` | `JSONB` | everything the bot saw at that moment |
| `features_version` | `SMALLINT` | |
| `rules_version` | `TEXT` | a hash of the config that produced it |
| `sent_at` | `TIMESTAMPTZ NULL` | null means Telegram refused it |

**`features` is saved exactly as the bot saw it, and never worked out again.**

Recalculating it later against updated chart-reading code trains a model on
inputs the live bot never produced. **Nothing detects that** — both sides keep
working and only the scores are wrong. If the shape of what gets saved
changes, `features_version` goes up; old rows stay valid at their old shape
and the training script filters.

**`sentence` is stored, not regenerated.** Same reason. It is what he actually
read on his phone, and the wording will change.

**`rules_version` is a hash of the config files.** Without it, "these signals
came back at 38%" is unanswerable — 38% *under which thresholds?* The bot
already refuses to reload settings while running, precisely so you can say
which rules produced which signals. This is that promise, written down.

**Unique on `(symbol, interval, band_price, candle_opened_at)`** — one shape,
one candle, one zone, one row. It also stops a restart re-sending.

---

#### Three timestamps, and they are not the same thing

**A candlestick pattern is one to three candles. A chart pattern is forty.**
Head and shoulders, a double top, a triangle — they span a stretch of chart,
and the design above quietly assumed a shape was a candle.

So a signal carries three moments:

| | |
|---|---|
| `spans_from` | where the shape **starts** — the left shoulder, the first touch of the trendline |
| `candle_opened_at` | the candle that **completed** it |
| `at` | when the bot could first **know** it |

**On a candlestick pattern all three collapse to the same candle.** On a chart
pattern they are days apart, and the gap between the last two is a lookahead
trap of its own.

> A right shoulder that prints on day 40 is not a swing until price proves it
> — sometimes day 42, sometimes day 55. `nsc-ta::swings` already refuses to
> call a peak until price has done the proving.

**The signal fires when the pattern became knowable, not when its last point
printed.** Record the wrong one and every backtest enters days early, at a
price nobody could have traded. It will not error. It will look **better**.

`at` is the honest one and it is the one an outcome must be measured from.

#### What a chart pattern needs that nothing has yet

**This is a build item, not just a column.** `nsc-ta` names eight shapes and
every one is a candlestick pattern:

```
    engulfing · harami · tweezers · piercing · dark cloud
    star · the march · nsc-bull and nsc-bear
```

A chart pattern needs a **neckline, a trendline, or a pair of them** — and
`nsc-ta` has no trendlines at all. The chain is:

```
    swings ──── BUILT, and tested
      └── trendlines ──── NOTHING
            └── chart patterns ──── NOTHING
```

**The hard part is already done.** A trendline is two swing points and a rule
for when it is broken, and finding a swing honestly — without counting
candles, without reading price the market had not printed — is the part that
took the work.

The geometry itself goes in `features`: the anchor swings, the neckline price,
the measured target. Not its own table — it is what the bot *saw*, and the
whole point of `features` is that what it saw is saved once and never worked
out again.

---

### 5. `signal_outcomes`

What the market did next. One row per signal per horizon.

| column | type | |
|---|---|---|
| `signal_id` | `BIGINT` → `signals` | |
| `candles_ahead` | `SMALLINT` | 1, 3, 5, 10 |
| `moved_for` `moved_against` | `NUMERIC` | in normal candles, never points |
| `reached_target_first` | `BOOLEAN NULL` | |
| `ambiguous` | `BOOLEAN` | |
| `settled_at` | `TIMESTAMPTZ` | |

**`ambiguous` is a `CLAUDE.md` rule with a column.** When the stop and the
target were both hit inside one candle, you cannot know which came first
without tick data. Guessing in your own favour is the single easiest way to
make a backtest lie. The row is written, marked, and **left out of the
numbers**.

**Measured in normal candles, not points.** 20 points is a big move on the
euro and nothing on gold. Every threshold in this project is a share of
something and the outcomes have to match, or you cannot compare pairs.

**Nullable `reached_target_first`** because "neither, within ten candles" is a
real answer and it is not a loss.

---

### 6. `signal_labels`

What *he* thought. The half no measurement can supply.

| column | type | |
|---|---|---|
| `signal_id` | `BIGINT` → `signals` | |
| `verdict` | `TEXT` | `took it` / `skipped it` / `would have skipped` |
| `note` | `TEXT NULL` | in his own words |
| `at` | `TIMESTAMPTZ` | |

**This is the table that cannot be recreated.** Candles can be re-downloaded
and outcomes recomputed. What he thought of a setup on the afternoon it
printed exists nowhere else the moment he forgets it.

**A signal can have several**, because he is allowed to change his mind and
the change is itself information.

The natural way to fill it is two buttons under every signal card. That is not
built and it is the cheapest thing on this page.

---

### 7. `rejections`

What was refused, and by which layer. A `CLAUDE.md` rule with nothing behind
it yet.

| column | type | |
|---|---|---|
| `at` | `TIMESTAMPTZ` | |
| `symbol` `interval` | `TEXT` | |
| `candle_opened_at` | `TIMESTAMPTZ` | |
| `layer` | `TEXT` | `shape` / `place` / `skip` / `direction` / `trigger` |
| `why` | `TEXT` | the specific test that failed |
| `features` | `JSONB` | the same shape as `signals.features` |
| `features_version` | `SMALLINT` | |

**The layer is the whole point.** "Nothing fired this week" and "forty things
fired and every one was thrown out at the place test" are completely different
problems, and today they are the same silence.

**It will be far bigger than `signals`** — most candles reject. Partition by
month, or prune rejections older than a year once the Phase 4 set is built.

**`features` matches `signals.features` exactly.** They are the two halves of
one dataset: what to take and what not to. A different shape on each side
makes them unusable together, which is the only use either has.

---

### 8. `news_events`

The economic calendar as it stood.

| column | type | |
|---|---|---|
| `at` | `TIMESTAMPTZ` | when it prints |
| `currency` | `TEXT` | `USD`, `All` |
| `title` | `TEXT` | |
| `impact` | `TEXT` | High / Medium / Low |
| `forecast` `previous` | `TEXT NULL` | as sent — they are not always numbers |
| `fetched_at` | `TIMESTAMPTZ` | |

**Why store something already public.** ForexFactory serves **this week
only** — there is no next-week file and no archive. A week not saved is a week
gone, and it is the thing that answers whether a setup fired in front of a
rate decision.

**`forecast` and `previous` stay text.** `-911K`, `4.3%`, `51.0`. Parsing them
to numbers means deciding what `-911K` means, and that decision belongs where
it can be tested, not in a column type.

---

### 9. `runs` — so silence has one explanation, not two

When the bot was up, and when the price line was not.

| column | type | |
|---|---|---|
| `started_at` | `TIMESTAMPTZ` | |
| `stopped_at` | `TIMESTAMPTZ NULL` | null means it is running, or it was killed |
| `rules_version` | `TEXT` | which settings this run used |
| `stopped_why` | `TEXT NULL` | `clean` / the trouble that ended it |

And a row per outage of the price line: `from_at`, `to_at`, `why`.

**This is the table the whole database exists for, and I left it out.**

The first question on the list is *"why did nothing fire last week?"* — and
the answer has two shapes:

```
    the market was quiet          nothing to say, and that is correct
    THE BOT WAS DOWN              everything to say, and nobody heard it
```

**Without this table those are the same empty week.** Worse than unanswered:
a quiet stretch caused by an outage reads as evidence the rules are too tight,
and the natural response is to loosen them — which is the exact mistake
`CLAUDE.md` calls fitting the rule to the wish.

The bot already knows all of it. `watch/trouble.rs` sends a card when the line
goes down and another when it comes back; it simply throws the fact away
afterwards.

---

### 10. `rule_sets` — so `rules_version` can be looked up

| column | type | |
|---|---|---|
| `version` | `TEXT` primary key | the hash |
| `files` | `JSONB` | every `config/*.toml`, as it read at the time |
| `first_seen_at` | `TIMESTAMPTZ` | |

**A hash you cannot resolve is a hash that tells you nothing.** `signals`
carries `rules_version` so that "these came back at 38%" can be answered with
*under which thresholds* — and that answer needs the thresholds themselves,
not a fingerprint of them.

One row per distinct configuration, written on startup. They are small and
there will be very few, because a restart with unchanged settings reuses the
row.

**It also dates a change he cannot otherwise date.** `reach_of_band` moving
from 0.5 to 0.4 is invisible in every other table; here it is a new row with a
timestamp, and every signal after it points at that row.

---

## Indexes, and what each is for

Add one **only** with a note about which query it serves.

| index | serves |
|---|---|
| `candles (symbol, interval, opened_at)` — the PK | the backtest reading forward |
| `signals (at DESC)` | "what has fired recently" |
| `signals (symbol, interval, band_price, candle_opened_at)` unique | stopping duplicates |
| `rejections (at DESC, layer)` | "why did nothing fire this week" |
| `levels (pair, drawn_at, removed_at)` | "which zones existed on this date" |
| `runs (started_at DESC)` | "was the bot even up that week" |

Six. Before adding a seventh, check one of these does not already cover it.

---

## Enums are `TEXT` with a `CHECK`, not Postgres enums

`shape`, `layer`, `verdict`, `impact`, `interval`, `timeframe`.

**A Postgres enum cannot have a value removed and cannot be reordered.** These
lists will grow — `shape` gains one every time he adds a pattern, `layer`
gains one for every strategy layer that gets built. `TEXT` with a `CHECK`
constraint is one migration to widen and one to narrow.

The values themselves are already named in Rust, and `nsc-data::store` is the
one place that writes them.

---

## What does NOT go in here

**The throwaway state — that is Redis:** which zones price is already sitting
in, which candles have been reported, where he is in a Telegram conversation.
All of it is lost on restart today and none of it is worth keeping past one.

**The bank consensus.** Quarterly targets from research desks, redrawn
monthly. Interesting to read, not something a backtest can use, and the free
tier gives averages rather than the per-bank rows that would be worth a table.

**The live level files.** `config/pairs` stays the truth for what is watched
now. Only its history comes here.

---

## Two decisions that need him

- [ ] **How long do rejections live?** They will outnumber signals by a wide
      margin. A year, then pruned? Kept forever and partitioned? It is cheap
      to decide now and awkward to change once there are millions.

- [ ] **Does he want the two buttons under a signal card** — *took it* /
      *skipped it*? It is the cheapest thing on this page and it fills the one
      table nothing else can. Without it `signal_labels` stays empty and
      Phase 4 has half a dataset.

## And one thing to build first

**The completeness scan, before any of this.** A query that says whether the
candle history for a pair and timeframe is whole — using the trading calendar
to tell a real gap from a shut market. It decides whether `backfills` needs to
exist at all, and nothing should read history until something can say the
history is sound.
