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

**One crate · 28 tests · clippy clean · levels arrive from his phone and stick.**

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
crates/nsc-work-man/src/
  main.rs       the flow, and nothing else
  settings.rs   pair, timeframe, digits — step 2 replaces this with config
  candle/       one candle, and whether it has finished. 8 tests
  feed.rs       asking Twelve Data
  card/         filling in a template, letting Chrome draw it. 7 tests
  bin/listen.rs the live price stream, proved to work
  message.rs    the caption — the notification banner, not the message
  telegram.rs   sending, as a media group

assets/card/
  chart.html     the candle chart. Carries its own open, high, low and range
  readout.html   where price sat inside the candle
```

**Every file is under 140 lines. Every folder with code in it has a
`README.txt`.**

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
| **1** | price touches a level he drew | an alert. One line, no picture. May fire on a candle still forming — it is only a heads-up |
| **2** | a candle closes inside that zone | the candlestick. What it actually did there |
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
- [ ] Nothing watches them yet. That is the price watcher

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
