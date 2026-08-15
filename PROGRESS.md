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

**Two crates · 94 tests · clippy clean · it watches his levels and says when price arrives.**

```
nsc-core        what the bot knows      no reqwest, no tokio — it CANNOT reach
nsc-work-man    everything that reaches
```

---

## What this is

A signal bot. Phase one sends signals to Telegram and places no trades.

**The design is a page you can click through** —
[Broker to Telegram](https://claude.ai/code/artifact/1093ff9f-f3b3-4af7-afd5-6377629ea1dd).
The two lanes, what sits in each stage, where your rules live, and the build
order. Everything else visual is indexed in [docs/README.md](docs/README.md).

**Forex pairs and gold, nothing else.** Trades are executed on the 4-hour and
the 1-hour; the weekly and daily are there for levels.

**The broker's chart is the truth.** Every timeframe comes from the feed
finished. Nothing is built from anything else — that assumption is what got the
last version of this project cleared out on 14 August 2026.

---

## Step 1 — the thin pipe — [x]

*Done when a candle closes and the phone buzzes.*

- [x] Connected to **Twelve Data**, key in `.env`, never in the code
- [x] **Gold works on the free plan** — `XAU/USD` comes back as
      `"type": "Precious Metal"`. That was the one thing we could not know
      without asking, because "commodities" is a paid feature on their pricing
      page
- [x] **The finished-candle rule, proved against the clock.** At 18:19 it sent
      the 17:00 candle and skipped the 18:00 one, because 18:00 plus an hour
      has not happened. Not asserted in a comment — demonstrated on real data
- [x] Telegram bot made, channel made, bot is an admin
- [x] A card is drawn and sent, with a one-line caption
- [x] **The finished-candle rule is pinned by tests**, including the one that
      shows why "skip the first candle in the list" is wrong: ask at 18:00:02
      and the finished candle is first or second depending on whether a price
      has landed yet

### What it is made of

```
crates/nsc-core/          WHAT IT KNOWS. No reqwest, no tokio — the manifest
                          is what stops it, not a rule anybody has to remember
  candle/       one candle, and whether it has finished.  9 tests
  levels/       his lines, the bands round them, the
                watching, what a candle did at one, and
                what to say about it.                    53 tests
  when/         whether the bot may speak at all.        10 tests
  error/        retry or give up — Answer and Knows.      3 tests
  settings.rs   pair, timeframe, digits — step 2 replaces this with config
  message.rs    the caption

crates/nsc-work-man/      EVERYTHING THAT REACHES
  feed/         asking Twelve Data.                       5 tests
  telegram/     sending — words, pictures, media groups.  3 tests
  card/         filling a template, letting Chrome draw.  9 tests
  retry/        trying again. Lives here BECAUSE IT SLEEPS 3 tests
  main.rs       the hourly chart card
  review.rs     one pair's levels, drawn
  bin/inbox/    levels arriving from his phone
  bin/watch/    the price watcher — rungs 1 and 2
  bin/alert.rs  draw any zone card without waiting for the market
  bin/levels.rs draw a pair's bands without waiting for one to be touched
  bin/listen.rs the raw price stream, kept as proof it works

assets/card/
  chart.html     the candle chart. Carries its own open, high, low and range
  readout.html   where price sat inside the candle
  alert.html     price at one of his zones, with the zone drawn
  close.html     a finished candle drawn inside the zone it touched
```

**Every folder with code in it has a `README.txt`.** Every file is inside the
250-line limit; `bin/watch/main.rs` at 206 and `when/tests.rs` at 195 are the
ones to watch.

**The design lives in HTML, not in Rust.** Open the file, change it, and the
next message picks it up — no rebuild. Chrome draws it headlessly.

Every run leaves the card in `preview/` as both a picture and a web page. Open
the page in Chrome, edit the template, refresh — that is the design loop.
Everything worth looking at is listed in [docs/README.md](docs/README.md).

The cost: **whatever machine runs this needs Chrome.** Fine on a Mac. A real
dependency on a server, and worth remembering before it goes anywhere else.

---

## What the feed actually does — [x]

All of it measured, none of it read off their documentation. The detail is in
`docs/worksheets/twelve-data.md`.

- [x] **Their day ends at 17:00 New York.** Checked by matching the daily
      candle's open against every hourly candle for thirty hours — exactly one
      matched. Their daily chart is your daily chart
- [x] **Their week opens Sunday 17:00 New York.** The forex week
- [x] **The newest candle is always still forming**, and skipping the first one
      in the list is not the fix — position is right most of the time, which is
      worse than being wrong always
- [x] **The datetime field means two different things.** An hourly stamp is the
      candle's open time. A daily stamp is the date it *ends* on
- [ ] **Weekend daily candles exist and are noise.** Saturday and Sunday come
      back with ranges of 0.57 and 1.32 against 60–200 on a real day. Your chart
      has five daily candles a week; this feed gives seven. **Not handled in
      code yet** — the rule is to drop any daily stamped Saturday or Sunday

### The limit that shapes the design

**8 requests a minute.** Not the 800 a day — that is plenty.

Our requests all want to happen at the same instant, on the hour. Eight pairs at
a 4-hour close is eight 1-hour candles plus eight 4-hour candles: **sixteen
requests in one second against a limit of eight.**

So the fetching has to spread itself over the minute or two after a close. Cheap
to build in now, annoying to retrofit.

---

## Still open

- [ ] **OANDA** — waiting on them, about 24 hours from 14 August. Worth having
      because it marks each candle finished or not, so the guessing stops
- [x] **The websocket works, and it changes the cost of everything.** Tested
      14 August: the line opens, gold is allowed on the trial plan, and prices
      arrive about one a second. So price watching costs **0 requests**, and a
      request only happens when price reaches a level — not once per candle
      close. See `crates/nsc-work-man/src/bin/README.txt`
- [ ] **Gold specifically has not been watched ticking** — it was shut for the
      weekend, so the test ran on BTC/USD. Change one word back on Sunday
- [ ] **`--card-height` is measured by hand.** Each template says how tall it
      is and Rust reads that line, but the number comes from measuring the page
      once and typing it in. It will go stale when the design changes
- [ ] **Everything was measured on gold only.** The majors almost certainly
      behave the same. Almost is not checked

---

## What the bot is allowed to say — [x] decided, [ ] built

**Silence is the default.** Nothing arrives on a quiet hour. Send something
every hour and by the second week he stops opening them, and then he misses
the one that mattered.

| | When | What arrives |
|---|---|---|
| **1** | price touches a level he drew | an alert **card** — the zone drawn, with price on it. May fire on a candle still forming; it is only a heads-up |
| **2** | a candle that **touched** the zone finishes | a close card — the candle drawn inside the band. **Once per candle**, not once per visit: while price is at a zone he wants to watch it candle by candle |
| **3** | it closed there **and** a strategy matched | the chart and the candlestick, with entry, stop, target and the sentence |
| **·** | morning and evening | a heartbeat — still running, pairs watched, zones touched, signals sent |

**Rung 2 is the point.** Price arriving at a level says nothing; it may cut
straight through. The *close* says whether it was a rejection. So rungs 2 and 3
never fire on a candle still forming.

The heartbeat exists because silence has one problem: after three quiet days
you cannot tell whether nothing happened or the bot died. Twice a day rather
than once — twelve hours of silence is believable, twenty-four is not.

Needs levels loaded, which is step 5. Until then the hourly message stays as
the only sign of life.

---

## His levels — [~]

**A level is a band, not a line.** Price does not stop at a number, it turns
somewhere near one — so "price is at the level" means price is inside a band.

- [x] **Weekly bands are 0.35 of a weekly candle.** Measured off his own gold
      chart from two bands drawn months apart at 4,094 and 3,343 — 76.11 and
      79.93 points, giving 0.35 and 0.36. Two independent draws landing on the
      same number is why this one is trusted.
- [x] **Daily bands are 0.46 of a daily candle.** 32.28 points, measured the
      same day. An older note said 0.60; that came from USDCAD before the
      reset, and his own hand wins.
- [x] **The same on every pair**, whatever he sends. In `config/levels.toml`,
      with the evidence in `docs/worksheets/levels.md`
- [ ] **The 4-hour thickness has never been measured** — 0.55 is a guess
      sitting between the other two
- [x] **He sends them from Telegram and they save themselves.** Tap the pair,
      tap the timeframe, send the numbers. A pair he has never sent creates its
      own file, with `digits` and the nightly break worked out from the name and
      marked as unchecked
- [x] **The buttons are the files in `config/pairs/`**, not a list in the code —
      that was the `settings.rs` mistake, and two lists always disagree
- [x] **They draw**, and they land where he drew them.
      `cargo run -p nsc-work-man --bin levels -- GBPUSD`
- [x] `XAUUSD` has 3, `GBPUSD` has 4
- [ ] **Five gold levels are still missing** — they are pixel estimates off a
      screenshot and stay out until he reads them off
- [x] **They are watched.** `cargo run -p nsc-work-man --bin watch` holds the
      price stream open for every pair with a file and says when price arrives
      in one of his bands. Rung 1 of the ladder
- [x] **It says price is *approaching*, not only that it has touched.**
      `approach_pips = 4.0` in `config/levels.toml`. A pip comes from each
      pair's `digits` — gold 0.10, the euro 0.0001 — so one setting is
      meaningful everywhere without a number per pair
- [x] **Any pair can overrule it** with its own `approach_pips`. Four pips is
      about two minutes of gold and about an hour of euro, so gold is the one
      likely to want more. Commented example in `config/pairs/XAUUSD.toml`
- [x] **And no wider than that, because the band is already the early
      warning.** Its outer edge is about 3 hours of movement from his line on
      gold and 6 on the pound. A first attempt added a quarter of a band on
      top and fired *nine hours* early on the pound.
      [`docs/diagrams/how-close.html`](docs/diagrams/how-close.html) has the
      measurements
- [x] It fires **once per touch, not once per price** — prices come about once
      a second and barely move, so without that rule one visit becomes twenty
      alerts. The first price never fires, and hovering on a band's edge does
      not fire repeatedly
- [x] **Leaving is measured differently from arriving** — a tenth of the
      band's thickness, about 8 points on gold. Easy to trigger, hard to
      reset. One pip each way would make a single visit an afternoon of alerts
- [x] **The alert goes as a card, not a line of text.** Telegram gives text no
      colour, no size and no layout, so approaching and arriving read the
      same. On a card the state is a chip, and **the zone is drawn** — his
      band with price marked on it and a dashed line where the alert fires.
      Three numbers in a message have to be compared in his head; a band with
      a dot on it does not
- [x] **`--bin alert` draws any of them on demand**, so a design can be
      changed without waiting for the market to do anything
- [x] **Rung 2 — a candle that touched a zone reports at its close.** Inside,
      above or below, and **a wick counts**: a candle that only reached in and
      closed back out is the rejection he is waiting for, and treating that as
      a miss would throw it away
- [x] **A 4-hour candle does not exist until its last hour has closed.** Three
      hourly closes can pass with the 4-hour silent; the fourth is when it
      speaks. `Bar::finished_by` is the one place that decides
- [x] **Rung 2 costs nothing when nothing is happening.** Only pairs with
      price at a zone are ever fetched
- [x] **There is no "a candle opened in the zone" message.** Spot forex runs
      without a break, so an open *is* the last close — it would repeat what
      arrived a minute earlier. Only a **gap** into a zone carries anything

### The calendar — [x]

- [x] **Monday is silent, and it means nothing at all** — no prices checked,
      no candles fetched, no queue to dump on him on Tuesday
- [x] **The trading week is not the calendar week.** It opens Sunday 17:00 New
      York, so Sunday evening is already Monday's session and Monday evening
      is Tuesday's. Read off the UTC calendar, Monday's silence would land
      three hours into Tuesday and miss Sunday entirely
- [x] **17:00 New York is not a fixed UTC time** — 21:00 in summer, 22:00 in
      winter. `config/when.toml` holds the New York time and the zone, never a
      UTC clock time
- [x] **Three states, not two.** `Anything` / `WatchOnly` / `Silence`. "Do not
      trade" and "do not speak" are different: the first four hours of a day
      report what is happening and suggest nothing
- [x] **Friday reports but opens nothing new**
- [x] **Nothing in `when/` reads the clock** — `now` is handed in, which is
      what lets the backtester run these exact rules over 2019
- [x] **Tuesday says what it FOUND**, not what arrived. Price can walk into a
      zone during Monday's silence, and a card saying "arrived" would put a
      Monday move on a Tuesday clock
- [x] **It costs no requests to run.** The candles that size the bands are
      fetched once at startup; after that every price is free

---

## Failing properly — [~]

Every failure now answers one question: **is it worth trying again?**

- [x] `error/` — every trouble this crate can have, in one place. Each answers
      `Answer::TryAgain(how long)` or `Answer::GiveUp`, and `keep_trying`
      respects it while still stopping after a few goes
- [x] The feed and Telegram have **named troubles**, not one catch-all. A
      dropped line waits 3 seconds; being told to slow down waits a minute; a
      wrong key stops on the first go rather than looking like a dead
      connection for a minute
- [x] Both feeds **refuse politely** — Twelve Data answers 200 with
      `{"code": 401}` in the body, Telegram answers 200 with `ok: false`. Both
      are read out of the body, and both are tested
- [x] **The whole library speaks named troubles** — `FeedError`, `SendError`,
      `CardError`, `LevelError`. The programs in `bin/` still use `anyhow`,
      which is right: a program with a person watching it only needs the trail
- [ ] Nothing survives a *restart* yet. Retrying handles a hiccup; a crash
      still loses the run

---

## Step 2 — every pair, behind one door — [ ]

**Designed:** [One Door, Eight Pairs](https://claude.ai/code/artifact/475ba411-e3c9-4d1e-8bb1-83591bd4e47e)

*Done when it runs a full trading day without missing a candle — including one
Friday 21:00 UTC, which is the moment that breaks it if anything will.*

**The burst is worse than 16.** At Friday 21:00 UTC the hour, the 4-hour, the
day and the week all end on the same second, because 21:00 UTC is 17:00 New
York. Eight pairs across four timeframes is **32 requests against a limit of
8 a minute** — and a refused request does not crash anything, it just leaves
the candle missing on the busiest close of the week.

- [ ] The pairs and their settings come from `config/`, not from constants at
      the top of `main.rs`
- [ ] Four timeframes — W1, D1, H4, H1
- [ ] Requests spread out after a close, to stay under 8 a minute
- [ ] One interface every feed hides behind, so adding OANDA is a config change
      rather than a rewrite. **Two feeds are already planned, so the door goes
      in now** — building it afterwards means unpicking it out of everything
      written in between

---

## Then, in order

- [ ] **Keep it** — Postgres, one table of candles, written as they arrive
- [ ] **The past** — download history per timeframe, and a scan that says
      whether it is complete before anything reads it
- [ ] **Read the chart** — swings, candle types, structure, and your hand-drawn
      levels loaded from config
- [ ] **The price watcher** — every tick against every level you drew. Alerts
      only. It may never produce a signal
- [ ] **The strategies** — one family first. Direction, place, trigger, stop,
      target, skip
- [ ] **Prove it** — replay the stored history through the same code the bot
      runs, with the lookahead guard on

---

## Rules this project has already paid for

Written down because each cost something.

**The lookahead rule got in through the drawing, not the analysis.** The first
chart quoted its headline price from the candle still forming — a picture with
a price on it gets believed exactly like a number does.

**Round to the instrument.** The feed sends gold as `4385.59525`. Gold is quoted
to two decimals. Printing all five is what makes a signal look like a debug
dump.

**A reply that parses is not a reply that worked.** Twelve Data refuses with a
normal-looking `{"code": 401}`. Telegram refuses with a polite `ok: false`. Both
in one afternoon, so it is a pattern rather than bad luck.
