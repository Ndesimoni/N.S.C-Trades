# Progress

Where the project actually is, as of **29 August 2026**.

Updated whenever a piece of work finishes — that is a rule in `CLAUDE.md`. A
progress file that is out of date is worse than none, because the next decision
gets made against it.

```
[x]  done
[~]  started, not finished
[ ]  not started
```

**Five crates · 337 tests · clippy clean · it watches his levels, says what
happens at them, and tells him when it cannot.**

```
nsc-core        what the bot knows      no reqwest, no tokio — it CANNOT reach
nsc-ta          reading a chart         describes; never decides
nsc-data        where prices come from  IBKR, and nothing else knows its name
nsc-strategy    the rules, in one place  no tokio, no reqwest — it CANNOT reach
nsc-work-man    everything that reaches
```

> **CUT BACK ON 29 AUGUST, AT HIS WORD.** *"We would only work with
> candlesticks and I will do my analysis and draw my levels and send them."*
>
> **Gone:** the bank consensus and `/banks`; the chart patterns (`nsc-ta::chart`
> — the curvy); **swings**; the indicators and Fibonacci. About 2,000 lines and
> 70 tests, with their config, cards, worksheets, diagrams and binaries.
>
> **The swing-finder bug went with them.** A ratchet in `memory.rs` had left it
> blind — 51 swings in 30,000 candles, none at all on the Aussie 1-hour after
> March 2025 — and every chart pattern stood on it. Nothing needs swings now, so
> the biggest known defect in the project no longer has anything to break.
>
> **What it costs:** trend. A hammer and a hanging man are the same candle and
> only the trend before it separates them, so `nsc-ta` says `long lower wick`
> and stops there. That is the honest answer rather than a guess.
>
> **Rungs 1, 2 and 3 were never touched** — they read his levels and two-candle
> shapes, and no swing was ever in that path.

> **RUNG 3 TRADES FOUR SHAPES AS OF 29 AUGUST**, up from two. Harami and
> marching joined his push-then-pin and the engulfing, at his word.
>
> **And three of the four had no size test.** Only `[push]` asked whether the
> candle actually moved. Measured over 270,000 candles: **39% of engulfings and
> 38% of haramis reached less than one normal candle** — shapely, and nothing
> happened. `min_reach` and `min_first_reach` went in at 1.0, the same number
> and the same argument `[push]` already used.
>
> **Marching deliberately got no floor.** 96% already clear one, so the setting
> would refuse almost nothing — and a threshold that never says no is worse than
> none, because the next person believes it is protecting them.
>
> **The harami had no tests at all** before this. It has five now, on a real
> gold harami from 21 April 2022.
>
> **THE THREE TIERS LANDED 30 AUGUST**, and a signal now sends **two
> pictures**.
>
> ```text
>     Inside   in the zone           RED     the only one that asks him to act
>     Close    within half a band    amber   it almost touched and did not
>     Bold     no zone near it       plain   2x a normal candle, or silence
> ```
>
> **A level beats size, always.** A shape at a zone is a setup whatever its
> reach; one away from every zone is only ever a remark, and the card says so
> in words before it says it in colour.
>
> **The wide chart goes first** — a hundred candles, his levels on it, and a
> **red ring** round the shape. The setup card goes under it. The chart says
> where, the card says what. Both are drawn in one hop off the price loop, so
> two pictures still hold one thread rather than two.

> **RUNG 3 IS WIRED TO TELEGRAM — 25 AUGUST.** A shape he trades at one of
> his zones now draws a card and sends it: the two candles on the band, and
> the one sentence the rules wrote. It rides on the candles rung 2 already
> fetched, so it costs no extra request, and it **only ever fires on a
> finished candle**.

> **RUNG 3 EXISTS — 25 AUGUST.** `nsc-strategy` was an empty folder this
> morning. **One rule:** a shape he trades — `nsc-bull`, `nsc-bear`, or an
> engulfing either way — sitting inside a zone or within half a band of it.
> Three strategies were described and collapsed into one, because the *place*
> test was identical for all of them. Spec in
> `docs/worksheets/strategies.md`. **Nothing is wired to Telegram yet, and the
> stop is still unanswered.** See [Rung 3](#rung-3--a-shape-at-a-level--x-25-august).

> **IT WATCHES THE ECONOMIC CALENDAR AS OF 25 AUGUST.** A card thirty minutes
> before a high or medium impact release — red for high, orange for medium,
> ForexFactory's own spelling because that is the calendar he reads. It runs
> beside the price watcher on its own clock and needs no IBKR. **Drawn against
> the real feed and checked, but it has never fired on a live release.**
> See [The news](#the-news--x-25-august).

> **HIS FIRST OWN PATTERN LANDED ON 22 AUGUST.** `nsc-bull` and `nsc-bear` —
> a push that shows one side winning, then a pin whose long tail points
> *against* it. Everything else in `pattern/` is a textbook shape; this one is
> his, which is what the `nsc-` prefix is for. Settings live in a `[push]`
> block of their own so no threshold is shared with general candle naming.
> **Measured, and it did not continue:** followed for ten candles across five
> pairs it reached +1 normal candle before -1 in 29 of 75. That is 80 trades
> with no level under them, which is under the hundred this project's own
> backtest rule asks for — see `docs/diagrams/push-then-pin-found.html`.

> **THE FEED CHANGED ON 20 AUGUST.** Twelve Data is gone. Candles and live
> prices both come from Interactive Brokers now, through `nsc-data`.
> **Live prices confirmed on EUR/USD and on GOLD**, and candles come back.
> One line in `.env` has to be quoted before it can reach Telegram.
> See [The feed](#what-the-feed-actually-does--rewritten-20-august).

> **TOP PRIORITY, agreed 16 August:** draw cards off the price loop. Drawing
> one blocks a worker thread for 2–10 seconds. Fine on his Mac, **stalls the
> whole bot on a one-core cloud box** — and hosting is the plan. See
> [Next, in this order](#next-in-this-order). Do it once a live session has
> run clean.

---

## What this is

A signal bot. Phase one sends signals to Telegram and places no trades.

**The design is a page you can click through** —
[Broker to Telegram](https://claude.ai/code/artifact/1093ff9f-f3b3-4af7-afd5-6377629ea1dd).
Everything else visual is indexed in [docs/README.md](docs/README.md).

**Forex pairs and gold, nothing else.** Trades are executed on the 4-hour and
the 1-hour; the weekly and daily are there for levels.

**The broker's chart is the truth.** Every timeframe comes from the feed
finished. Nothing is built from anything else — that assumption is what got the
last version of this project cleared out on 14 August 2026.

---

## Running it

```sh
cargo run -p nsc-work-man                    # THE BOT — watcher and inbox both
cargo run -p nsc-work-man --bin cards -- …   # draw any card without waiting
cargo run -p nsc-work-man --bin levels -- GBPUSD
cargo run -p nsc-work-man --bin listen       # IBKR's raw ticks, EUR/USD
cargo run -p nsc-work-man --bin cards -- news       # what is coming up
cargo run -p nsc-work-man --bin cards -- news busy  # several releases at once
```

**TWS or IB Gateway has to be running and logged in** for any of them but
`--bin cards -- trouble` and `--bin cards -- news`. There is no feed without
it and no fallback. The news card is the exception because the economic
calendar is a plain web page with no key on it.

- [x] **The obvious command runs the real thing.** It used to run a leftover
      from step one that sent a gold card *every time it was called* — the
      exact opposite of silence-by-default
- [x] **The watcher is library code**, not a binary, so `main.rs` is four lines
      and the whole bot is reachable from a test
- [x] **The inbox runs inside it.** It was a second program: two terminals,
      remembering both, and if it was not up a level he sent went nowhere with
      nothing to say so
- [x] **Level files are written in one move** — to a file beside it, then
      renamed over the top. Two things read these files now, and a plain write
      is not one step

---

**What he can send it** — `/status`, `/help`, `/pairs`, `/chart`, `/level`,
`/remove`, `/restore` — is written up for him in
[docs/telegram.md](docs/telegram.md) — every command, what comes back, and
what turns up on its own.

---

## What it sends — [x] rungs 1 and 2, [ ] rung 3

**Silence is the default.** Nothing arrives on a quiet hour. Send something
every hour and by the second week he stops opening them, and then he misses the
one that mattered.

| | When | What arrives |
|---|---|---|
| **1** | price comes near a zone | an **alert card** — *approaching* |
| **1** | and again when it goes **in** | the same card — *in the zone* |
| **2** | a candle that **touched** a zone finishes | a **close card** — the candle drawn on the band, named by what it did |
| **3** | a shape he trades prints at one of his zones | a **setup card** — the two candles on the band, and one sentence. Never says buy |
| **·** | 30 min before a high or medium impact release | a **news card** — what prints, whose currency, forecast and previous, red for high |
| **?** | he sends `/news` and taps *Today* or *This week* | a **calendar card** — every release with its time, grouped by day, each marked *passed* or counting down |
| **·** | 07:00 UTC, only if nothing else was sent | a **heartbeat card** — every pair, its levels as dots in his colours, the nearest zone on each |
| **·** | he sends a level while it is running | an **armed card** — a tick, and the count |
| **·** | the line breaks, comes back, or it stops | a **trouble card** — severity in the colour |

**Everything it can say is a card**, and every one can be looked at on demand
without waiting for it to happen. That was the last plain-text message going.

---

## Rung 1 — price at a zone — [x]

- [x] **A level is a band, not a line.** Price does not stop at a number, it
      turns somewhere near one
- [x] **It fires on a touch** — `approach_pips = 4.0`, and a pip comes from
      each pair's `digits`, so one setting means the same everywhere
- [x] **Any pair can overrule it** with its own `approach_pips`. Four pips is
      two minutes of gold and an hour of euro
- [x] **And no wider, because the band is already the early warning.** Its
      outer edge is ~3 hours of movement from his line on gold, 6 on the pound.
      A first attempt added a quarter of a band on top and fired *nine hours*
      early. [`docs/diagrams/how-close.html`](docs/diagrams/how-close.html)
- [x] **Once per touch, not once per price.** Prices arrive about once a second
      and barely move; without the rule one visit is twenty alerts
- [x] **Leaving is measured differently from arriving** — a tenth of the band,
      about 8 points on gold. Easy to trigger, hard to reset
- [x] **The first price never fires.** It says where price *is*, not that it
      arrived — it may have been sitting there for hours
- [x] **It speaks when price gets DEEPER** — near, then in. Two messages per
      visit and never more. Entering used to say nothing at all: coming near
      marked the band as reached, so walking in was not a change, and he heard
      *"coming up on your zone"* then waited up to an hour for a candle
- [x] **Wobbling at the edge is still silent**, because it never gets deeper
      than it already was
- [x] **Three states, and the card says which**: *approaching*, *in the zone*,
      and *already in the zone* — the last for what it **found** on waking, so
      a Monday move never gets a Tuesday clock

## Rung 2 — what the candle did — [x]

- [x] **A wick counts.** A candle that only reached in and closed back out is
      the rejection he is waiting for; treating it as a miss throws it away
- [x] **The card names what happened** — *kissed it*, *pushed back*, *closed
      inside*, *cut through*. `kiss_depth` is where the graze ends
- [x] **Cutting through is never called a rejection.** Deep, and closed
      outside — the exact shape of a rejection, but the level *broke*
- [x] **Once per candle**, not once per visit. His decision: while price is at
      a zone he wants to watch it candle by candle
- [x] **ONLY A CLOSE OUTSIDE THE BAND — 26 August.** A candle that settled
      *inside* the zone no longer sends. He already got the alert when price
      arrived; a card saying it is still there is the same news twice, and a
      message he stops opening is one he misses when it matters.
      **The rejection survives** — a wick into the zone that closed back out
      finishes above or below the band. `only_breaks` in `config/levels.toml`
      turns the old behaviour back on
- [x] **A 4-hour candle does not exist until its last hour has closed.** Three
      hourly closes can pass with the 4-hour silent
- [x] **~~The twenty-minute look~~ — GONE, 27 August.** A third message per
      zone visit was a heads-up about a heads-up. Its going matters more than
      the card did: **nothing in this project reads an unfinished candle any
      more**, so "a candle still forming is invisible to the analysis" is now
      true everywhere rather than true with one exception
- [x] **No "a candle opened in the zone" message.** Spot forex runs without a
      break, so an open *is* the last close. Only a **gap** carries anything
- [x] **It costs nothing when nothing is happening.** Only pairs with price at
      a zone are fetched, and one request serves both the close and the look

---

## His levels — [~]

- [x] **Weekly bands are 0.35 of a weekly candle.** Measured off his own gold
      chart, two bands drawn months apart, giving 0.35 and 0.36
- [x] **Daily bands are 0.46 of a daily candle**, measured the same day
- [x] **The same on every pair**, in `config/levels.toml`, evidence in
      `docs/worksheets/levels.md`
- [x] **He sends them from Telegram and they save themselves.** Tap the pair,
      tap the timeframe, send the numbers. A new pair creates its own file
- [x] **The buttons are the files in `config/pairs/`**, not a list in the code
- [x] **They draw**, and they land where he drew them
- [x] **A level sent while it is running is picked up**, no restart. They used
      to be read once at startup — the inbox saved it, the file was right, and
      the level did nothing for days
- [x] **Checked by the clock on the files**, every ten minutes, on quiet days
      too: the weekend is when he does his chart work
- [x] **Only the changed pair costs a request.** An untouched pair keeps the
      `Watch` it had; rebuilt, it forgets which zones price is in
- [x] Currently: `EURUSD` 4, `GBPUSD` 4, `XAUUSD` 3, `USDCAD` 2
- [x] **The same price is one level, whatever chart it arrives on.** He sent
      three euro levels twice and got both copies — one line on his chart was
      two bands, two alerts, two closes, and a heartbeat claiming seven levels
      where he had drawn four
- [x] **Compared as numbers**, so 1.15 and 1.15000 are one level, and repeats
      inside a single message are dropped too
- [x] **The timeframe is not part of what makes it unique.** Re-sending a
      weekly line off the daily chart has not changed anything about it, and a
      62-pip band and a 29-pip band round one line fire twice as price passes.
      It keeps the chart he first drew it on
- [x] **A pair whose file will not read says so** before asking him to stop
      it. It said *"it has 0 levels on it"*, which is the one sentence that
      would make him tap yes without thinking
- [x] **He can stop a pair from his phone** — `/remove`, pick it, then confirm.
      **Two taps**, because it throws away every level he drew for that pair
      and the first tap happens while he is doing something else. The second
      one tells him how many levels are on it first
- [x] **`/status` answers "is it running, and is anything close?"** whenever
      he asks. The same card as the morning heartbeat, which only comes on a
      day nothing else did
- [x] **The commands are registered with Telegram**, so they appear in the
      tap-list beside the message box and he never types one. `/help` lists
      them too, because the file that describes them is on his Mac
- [x] **`/pairs` shows them all, and one at a time.** Tap a pair to see what
      it holds and what can be done to it — add, take one off, chart, stop
      watching
- [x] **`/chart` draws a pair on any of his three charts**, 17 August. Two
      doors — the command, and a 📈 button on the pair's page. It was only
      reachable by SAVING a level before, so seeing a chart meant adding
      something in order to see one
- [x] **A chart with none of his levels on it says so.** 150 four-hour candles
      is twenty-five days and his levels are years apart, so the daily and the
      4-hour often hold none. A correctly empty chart and one whose bands
      failed to draw are otherwise the same picture
- [x] **One level can be taken off.** Undo only ever reached what the last
      message added: fine for a typo, useless for *"that 1.15 from last week
      was wrong"*. Matched on the price as a NUMBER, and the comments in the
      file survive it
- [x] **`/restore` puts a stopped pair back**, one tap, under the pair's own
      name whatever the file is called
- [x] **It refuses to land on a pair he is already watching.** He may have
      stopped one, drawn it again, and then reached for the old set — which
      would replace the levels he is using and say nothing
- [x] **Stopped is moved, not deleted.** The file goes to
      `config/pairs/removed/` and comes back by being moved out. Retiring the
      same pair twice keeps both sets. The watcher notices within ten minutes
- [x] **The reply names what he already had, and which chart it is on** —
      *"1.15000 is already a weekly level"*. He may have expected it to move;
      silence would leave him thinking it had. And undo is told what was
      actually added, or it would cut levels he sent weeks ago
- [x] **The three already in `EURUSD` are gone**, comments untouched
- [ ] **The 4-hour thickness has never been measured** — 0.55 is a guess
- [ ] **Five gold levels are still missing** — pixel estimates off a
      screenshot, and they stay out until he reads them off
- [ ] **0.35 was only ever verified on gold.** 62 pips on the pound is
      unchecked — one screenshot settles it

## Rung 3 — a shape, at a level — [x] 25 August

**The crate exists and it is pure.** `nsc-strategy` had been an empty folder
since 16 August. Spec: `docs/worksheets/strategies.md`, and that file wins over
the code.

- [x] **One rule, not three.** He described three strategies; the *place* test
      was identical in all of them, so they are one rule with four shapes
- [x] **The shapes he trades**: `nsc-bull`, `nsc-bear`, bullish engulfing,
      bearish engulfing. `pattern/` names twelve and he trades four — the rest
      are on every candlestick page, not on his chart
- [x] **Inside the band, or within half a band of its edge.** His number,
      25 August. A share of that band's own thickness, never a distance
- [x] **NO TOUCH RULE.** Asked whether the pin had to touch he said it need
      not, and that touching was no problem either. Distance is the only test
- [x] **Measured from the tail tip** — the pin's low on a bull, its high on a
      bear. Argued from what the pattern *is*: the tail is a pullback that
      failed, so if it reached the level, the level is what stopped it
- [x] **The nearest zone wins**, not the first one loaded. Which zone a shape
      printed at is the whole content of the signal
- [x] **Breaking the band is reported, never required.** He was asked whether
      the break was the trigger and said the shape at the zone is a signal
      either way
- [x] **It cannot reach anything, and that is proved rather than promised.**
      Dropping a `reqwest::Client` into it fails to compile:
      `error[E0433]: use of unresolved module or unlinked crate`
- [x] **One sentence per signal**, and a test that pins it never saying *buy*,
      *sell*, *entry*, *target* or *stop*
- [x] **Eighteen tests**, on the two gold candles off his own screenshot. The
      one that matters: the same shape with the zone moved a thousand points
      away is **not** a signal

**Why this rung is a test rather than an answer.** `nsc-bull` and `nsc-bear`
were measured on 22 August: followed for ten candles they reached +1 normal
candle before -1 in **29 of 75, where a coin flip is 50%** — and **none of
those had a level under them**. This crate is the test of whether the level is
the missing half. If these come back at 38% too, the level does not save it,
and that is a finding rather than a failure.

- [x] **~~Nothing is wired to Telegram.~~** Done the same day. `setup.html`,
      `card/setup.rs`, `watch/closes/setups.rs`, and
      `--bin cards -- setup` to look at one without waiting
- [x] **It rides on rung 2's candles**, so a signal costs no extra request.
      The fetch went from three candles to twenty, because a normal candle is
      an average over fourteen — and IBKR paces on the number of requests, not
      on how many bars each asks for, so it is free
- [x] **ONLY EVER ON A FINISHED CANDLE.** A shape halfway through a candle is
      not a shape, and one that un-forms before the close would have been a
      message about something that never happened
- [x] **The history is cut at the candle being judged**, so nothing after it
      can be read — the lookahead rule as the shape of the call rather than a
      discipline
- [x] **Its own key**, so a candle worth both messages — what it did at the
      band, and the shape it completed — sends both
- [x] **Chrome runs off the price loop** via `spawn_blocking`, like the news
      card. The six older cards still block a worker
- [x] **Unreadable `config/strategy.toml` turns rung 3 off** and leaves the
      alerts and closes running, loudly
- [ ] **Where the stop goes — still unanswered.** Asked for since 16 August. A
      signal with no stop is a reading, not a trade, and version 1 says so
      plainly rather than implying an entry
- [ ] **What makes him skip one.** The `skip` layer has nothing in it. Every
      qualifying shape at a level will fire until he says what he leaves alone
- [ ] **Nothing has measured these against the 38%.** That needs
      `nsc-backtest`, which does not exist

---


## The news — [x] 25 August

**A level in front of a rate decision is not the same level as one on a quiet
Thursday.** Asked for on 16 August, built on the 25th.

- [x] **The source is ForexFactory's weekly file.** Free, no key, no signup —
      and it is the calendar he already reads, so the bot's week and his week
      are the same list. A source that disagreed with his screen would get
      distrusted the way levels that lag his chart do
- [x] **IBKR cannot do this.** Its API has six news calls and every one is
      headlines from a provider. No rate decisions, no payrolls, nothing
      scheduled. There is a Wall Street Horizon calendar and it is corporate
      earnings, for stocks. Checked in `ibapi` 2.12
- [x] **A card thirty minutes ahead**, `warn_minutes` in `config/news.toml`
- [x] **High and medium, every currency.** His call, and it is the right one:
      filtering by currency would go quietly blind the day he adds a pair —
      the pair watched and its news not, with nothing saying so
- [x] **Red is high, orange is medium.** ForexFactory's spelling, not a design
      choice. Red meaning anything else would make him translate the card
      every time, under time pressure
- [x] **The header's rule takes the heaviest rating on the card**, not the
      first one listed. One high and two mediums is a high-impact card
- [x] **One card per release, not per line.** Three Australian CPI numbers
      print in the same second and share a card. Apart, they buzz his phone
      three times for one event
- [x] **A window with two edges, and the far one is the point.** An event
      speaks from 30 minutes before until 5 after. Without the far edge a bot
      restarting at two in the afternoon finds the week's file full of this
      morning and sends every one of them at once
- [x] **Once per release, and it survives a re-read.** The file is downloaded
      every six hours and the same event is in every copy — so "already said"
      is kept by time, currency and title together. The time alone will not do
      and neither will the title
- [x] **It fails quiet.** Calendar down, rate-limited, config missing → say so
      in the terminal and carry on with what it had. Nothing about the news
      may stop the bot saying what price is doing at his levels
- [x] **The refusal that arrives looking like a success.** Over two downloads
      in five minutes ForexFactory sends an HTML page saying "Request Denied"
      under a normal 200. Handed to the JSON parser that reads as "the feed
      changed shape" — which is *give up* — and one busy afternoon would have
      retired the watcher for good. **Third time this project has met this**:
      Twelve Data's `{"code": 401}`, Telegram's `ok: false`, now this
- [x] **Chrome runs off the loop.** `spawn_blocking` from the start. The six
      older cards still block a worker and that is the top open bug; there was
      no reason to add a seventh
- [x] **It runs beside the price watcher**, spawned like the inbox. It needs
      no prices and no IBKR, and the price loop blocks for hours on the socket
      — a check living inside it would only run when the line dropped
- [x] **`--bin cards -- news`, `news busy`, `news today`, `news week`** draw
      every shape on demand, with no TWS at all. `busy` picks the busiest group
      of the week, which is the one the layout has to survive

**And `/news` asks for the list, whenever he wants it.** Added the same day.

- [x] **In the tap-list beside the message box**, registered with Telegram
- [x] **Two buttons.** *📅 Today* is the whole day; *🗓 This week* is what is
      left of it. Different questions — "what am I in for" and "what is coming"
- [x] **NOTHING IS LEFT OUT OF EITHER LIST — every row says which side of now
      it is on.** Gone ones read *PASSED* and are greyed; the rest carry how
      long they have: *in 45m*, *in 10h 53m*, *in 3d 10h*
- [x] **The header counts both halves** — *1 gone · 17 to come*. "18 releases"
      does not say whether the day is over
- [x] **The units shrink as it gets closer.** Two days out the minutes do not
      matter; forty minutes out they are the only thing that does — and "in 0h"
      reads as a card that failed to fill in
- [x] **The countdown is not coloured.** The stripe already carries impact and
      a second colour on the row would compete with it
- [x] **A list, not a release.** Its own card: every row carries its own time
      and the forecast is left off. Eighteen rows of numbers is a spreadsheet
      and he reads it on a phone
- [x] **The week grows a heading per day and today grows none** — a heading
      over a list that is all one day is a line saying nothing
- [x] **Same `config/news.toml` as the warnings**, so the list he pulls up and
      the cards that arrive unasked can never disagree about what counts
- [x] **Nothing on the calendar gets words, not a card.** Running Chrome for
      ten seconds to draw the word "nothing" is the mistake `/status` already
      made on a resting day
- [x] **It answers either way.** Chrome fails or Telegram refuses the photo and
      the caption still carries the answer. He asked outright, so no reply is
      indistinguishable from a dead bot
- [x] **Measured before it was designed.** A real week is 69 events, 18 of them
      high or medium, 1–6 a day. Both views fit one card comfortably — that was
      checked against the live file rather than guessed

**A bug this found, and it is the kind this project keeps meeting.** The card
went out headed *"4 releases"* with three on it. Chrome screenshots a
**window**, not a page, so the fourth was simply cut off — and it reads as a
quieter week, not as a fault. The row height had been copied from the
heartbeat, whose rows are one line where these are two. The heights are pinned
in `news.css` now and `card/tests/growing.rs` reads that file to check the two
still agree. It fails if either moves.

- [ ] **It has never fired on a live release.** Drawn against the real feed
      and checked by eye, both the one-release and four-release cards. Nothing
      has arrived on his phone from the watcher itself
- [ ] **The level and the news do not know about each other yet.** The whole
      point was a level *in front of* a rate decision, and that join needs
      rung 3. Today they are two separate messages
- [ ] **This week only.** There is no next-week file — that URL 404s. Friday
      evening the bot knows nothing about Monday until it rolls over

---

## The calendar — [x]

- [x] **The trading week is not the calendar week.** It opens Sunday 17:00 New
      York, so Sunday evening is already Monday's session and Monday evening is
      Tuesday's
- [x] **17:00 New York is not a fixed UTC time** — 21:00 in summer, 22:00 in
      winter. The config holds the New York time and the zone
- [x] **Monday is silent, and it means nothing at all** — nothing checked,
      nothing fetched, no queue to dump on him on Tuesday
- [x] **Saturday and Sunday too**, and not as a preference: the market shuts
      Friday 17:00 New York and opens Sunday 17:00, and on this calendar that
      closed stretch *is* the sessions called Saturday and Sunday
- [x] **Three states, not two.** `Anything` / `WatchOnly` / `Silence`. The
      first four hours of a day report what happens and suggest nothing
- [x] **Friday reports but opens nothing new**
- [x] **Nothing in `when/` reads the clock** — `now` is handed in, which is
      what lets the backtester run these exact rules over 2019

## The heartbeat — [x]

- [x] **Only on a day that said nothing else.** On a busy day it never fires
- [x] **07:00 UTC, before London opens** — knowing the bot works *before* the
      hours he trades beats a post-mortem after them
- [x] **Due at the first 07:00 after the session opened**, not "today at 07:00"
      — the session straddles midnight
- [x] **Once a session.** A heartbeat that repeats is worse than none
- [x] **It fires on Monday too**, the one message that does
- [x] **A card**: every pair, its levels as dots in his colours, and how far
      price is from the nearest zone on each — so a pair that quietly lost its
      daily levels shows as a missing blue dot
- [x] **Its height is worked out, not typed**, because it grows a row per pair.
      Left unfilled the card *fails* rather than clipping the last pair off

## When something goes wrong — [x]

- [x] **The line dropping is no longer the end of it.** The socket closing used
      to return `Ok` and the process exited *successfully* — and the heartbeat
      went with it, so a dead bot and a quiet day looked identical
- [x] **It reconnects**, keeping what it knew: which candles were reported,
      which zones were announced, when it last spoke
- [x] **On a silent day it does not open the line at all**
- [x] **Nothing is marked as said until it has actually gone.** That was wrong
      in four places at once — the alert, the close, the waking report and the
      heartbeat — and each lost its message for good on a single hiccup
- [x] **Every send tries again**, through the retry that `SendError` has known
      how to answer since the day it was written and was never asked
- [x] **A card that will not send does not drop the price line.** It said the
      feed was down when Telegram had hiccupped
- [x] **The last picture is cleared before drawing.** Chrome answers 0 whether
      it drew your card or its own error page, so "a file appeared" is the only
      check — and one was already there. A failed draw would have sent today's
      caption on yesterday's chart
- [x] **The waking report waits a minute between goes.** It is asked for on
      every price, and prices arrive about once a second — so leaving it
      unmarked after a failure turned one bad send into a request a second at
      Telegram. That one was introduced by the fix before it
- [x] **A reply that is not candles is cut short.** What comes back can be a
      whole web page, and that string becomes the error on a trouble card. Cut
      by characters, not bytes, or their `£` splits in half and panics
- [x] **A refused Telegram message is an error.** It printed the refusal to a
      terminal he is not watching and answered Ok, so everything upstream
      believed he had been replied to. He would have seen nothing and had no
      way to tell that from a dead bot
- [x] **What went wrong is escaped before it goes in a message.** Every one is
      parsed as HTML, so a `<` in an error is an unclosed tag and Telegram
      refuses the whole thing — the reply that says what broke is exactly the
      one that has to arrive
- [x] **The subscription reply is read.** They answer per symbol with a
      `success` list and a `fails` list, and nothing looked at it — a pair
      they will not serve is refused while the socket stays open, so no prices
      arrive for it and nothing errors. Every pair refused is now an error;
      some refused says so and watches the rest
- [x] **A line that opens and shuts without a price counts as broken.** A key
      over its quota does exactly that
- [x] **Quiet about hiccups, loud about outages** — nothing said for five
      minutes, then once, with a second message when it comes back
- [x] **Trouble it cannot recover from says so and stops.** Proved by hiding
      `config/when.toml`: the message arrived
- [x] **The secrets do not travel with the error.** `reqwest` puts the failing
      URL in its message and both secrets live in a URL, so *"could not reach
      Telegram"* printed the bot token in full. Stripped at the source in three
      places, and scrubbed again before anything reaches a card
- [ ] **A hard crash still loses the run.** Nothing can send a message if the
      process is killed outright — that needs a supervisor (`launchd`)

## Errors — [x]

- [x] Every failure answers one question: **is it worth trying again?**
      `Answer::TryAgain(how long)` or `Answer::GiveUp`
- [x] **Named troubles, not one catch-all.** A dropped line waits 3 seconds;
      being told to slow down waits a minute; a wrong key stops on the first go
- [x] **Both feeds refuse politely** — Twelve Data answers 200 with
      `{"code": 401}`, Telegram answers 200 with `ok: false`. Both read out of
      the body, both tested
- [x] **The library speaks named troubles**, the binaries use `anyhow`

---

## What it is made of

```
crates/nsc-core/          WHAT IT KNOWS. No reqwest, no tokio — the manifest
                          is what stops it, not a rule anybody remembers
  candle/       one candle, whether it has finished, and
                how long a timeframe is                    9 tests
  levels/       his lines, the bands round them, the
                watching, what a candle did at one, and
                what to say about it                      79 tests
  when/         whether it may speak, and the heartbeat   16 tests
  error/        retry or give up                           3 tests

crates/nsc-ta/            READING A CHART. It describes, it never decides
  candle/       what ONE candle is -- four numbers, measured
                before it is named, then TWELVE SHAPES it
                can be. Thresholds in config/candles.toml   12 tests
  pattern/      what a RUN of them does — engulfing, harami,
                tweezers, piercing, dark cloud, and the star
                with the abandoned baby inside it, and the
                march — soldiers and crows.
                AND HIS OWN: nsc-bull and nsc-bear, a push
                then a pin whose tail opposes it            30 tests
  source/       the question — the trait, what a timeframe
                is, what a live price is                    4 tests
  sources/ibkr/ the answer. Connecting, contracts, candles,
                and the live tick line                     19 tests

crates/nsc-work-man/      EVERYTHING THAT REACHES
  main.rs       four lines — it runs the watcher
  watch/        THE BOT. Rungs 1 and 2, the calendar, the
                heartbeat, reconnecting, and picking up a
                level he sends while it runs
  inbox/        the other side of Telegram — his levels,
                stopping a pair, /status and /help. Spawned
                beside the watcher                         6 tests
  card/         filling a template, letting Chrome draw   17 tests
  telegram/     sending — words, pictures, media groups    3 tests
  retry/        trying again. Lives here BECAUSE IT SLEEPS 3 tests
  places.rs     where things are — his inbox, the settings
                files, where cards are drawn. ONE COPY
  secrets.rs    reading .env, and saying so when it
                will not read                              3 tests
  review/       one pair's levels, drawn on whichever chart he asked for
  bin/cards/    draw ANY card without waiting for anything
  bin/levels.rs draw a pair's bands on demand
  bin/listen.rs IBKR's raw ticks — the window onto what
                actually arrives
  bin/read.rs   READ A CHART the way the code reads it — real
                candles, nsc-ta over them, the name the CODE
                gave each. Driven by the read-the-chart skill
  bin/after/    WHAT PRICE DID NEXT after each pattern, against
                the base rate and against noise. NOT a backtest
  bin/candles/  WHERE IBKR STARTS ITS DAY. Lines each daily
                candle's open up against the hourly opens —
                the hour that shares the number is the
                boundary. Refuses to answer on thin
                evidence                                    4 tests

assets/card/
  style.css      the palette, typefaces and page box — shared by all
  <name>.css     each card's own styling, and its height
  chart.html     the candle chart, with his levels as bands
  alert.html     price at one of his zones
  close.html     a finished candle at a zone, named by what it did
  heartbeat.html what is being watched, on a day nothing happened
  armed.html     a level he just sent is now being watched
  trouble.html   the bot itself has a problem — severity in the colour
  readout.html   NOT SENT by anything. Superseded by close.html

config/
  levels.toml    band thickness, how close counts, where a graze ends
  when.toml      the trading day, the silent days, the heartbeat
  pairs/*.toml   ONE FILE PER PAIR — the file is why the pair is watched
```

- [x] **Every folder with code has a `README.txt`**, and every file — Rust,
      HTML and CSS — is under the 250-line limit
- [x] **Checked by a script, 20 August**, against all four structure rules at
      once: nothing over 250, no file holding a type and its own tests, every
      `mod.rs` a front door with no types or logic in it, every code folder
      with a README. All four pass
- [x] **A second limit, added 20 August: no more than 170 lines of actual
      CODE** — blank lines and comments do not count. Two limits because they
      stop two different things: the 250 is how far you scroll, the 170 is how
      much is going on. A 240-line file with barely a comment in it passes the
      first and fails the second
- [x] **Explaining deliberately costs nothing.** A limit that counted doc
      comments would train everybody to delete them, and the first thing to go
      is always the paragraph saying why the obvious approach was wrong
- [x] **Nothing breaks it today.** The most code in any one file is 143 —
      `card/zone.rs`, then `levels/tests/watching.rs` and `levels/alert.rs` at
      141. So it costs nothing now and bites the next file that tries to do
      two jobs. `CLAUDE.md` carries the one-line command that measures it
- [x] **His chat id lived in three files and `config/pairs` in five.** None of
      them disagreed, which is the only reason nobody noticed — two copies of
      a string agree right up until one is changed. All of it is `places.rs`
      now
- [x] **`watch/README.txt` was 380 lines and `inbox/README.txt` 321.** Both
      were past the point where anybody reads them end to end, which is the
      thing the rule exists to stop. The detail moved down into the folder it
      describes; nothing was dropped
- [x] **Each card's styling lives beside it** as `<name>.css`. A card was a
      350-line file; the markup and script are what change, the CSS sits still
- [x] **The design lives in HTML.** Edit the template, redraw, no rebuild
- [ ] **`--card-height` is measured by hand** on every card but the heartbeat.
      It will go stale when a design changes
- [x] **Doc comments say `text` when they mean text.** An indented block in a
      doc comment is Rust to rustdoc, so a table and a list of commands were
      being compiled — and failing. See below
- [x] **Every file is inside the 250-line limit**, every `mod.rs` is a front
      door with no types or logic, and every folder with code has a
      `README.txt` that names the files actually in it with the right count
- [x] **No file holds a type and its own tests.** That is what earns a folder,
      and the folder is the module — `levels/` holds `band.rs` and `watch.rs`
      with `tests/` beside them, which is the shape `CLAUDE.md` draws
- [ ] Eight files sit between 200 and 250 and want watching: `close.html`,
      `card/tests.rs`, `levels/alert.rs`, `inbox/conversation.rs`,
      `chart.html`, `watch/closes.rs`, `levels/tests/watching.rs`,
      `card/zone.rs`

The cost of all this: **whatever machine runs it needs Chrome.** Fine on a
Mac. A real dependency on a server.

---

## What the feed actually does — rewritten 20 August

**The feed is Interactive Brokers.** Twelve Data is gone — candles and live
prices both come from IBKR, through `nsc-data`.

### What is built — [x]

- [x] **Candles.** `MarketDataSource::candles` — newest first, stamps in UTC,
      `MidPoint` prices, extended hours. Spot forex has no TRADES to ask for;
      there is no exchange to trade on
- [x] **The timezone fix is one line, and it is the whole fix.** IBKR stamps a
      bar in whatever timezone TWS was logged in with — his is Dubai. Every
      stamp goes through `unix_timestamp`, which is the same number in Dubai as
      in London, so nothing has to know what TWS was set to
- [x] **The timezone alias is registered before anything else.** TWS reports
      the machine's zone as *"Gulf Standard Time"*, a Windows name the library
      does not know, and without the alias connecting fails with an error that
      says nothing about timezones
- [x] **The live price line.** One subscription per pair, folded into one
      channel in `nsc-data`, so the watcher kept the loop it always had
- [x] **A price is the MIDDLE of the bid and the ask.** IBKR never sends a
      price — it sends a bid, and separately an ask. It has to be the middle,
      because the candles are mid prices: measured against a bid, a level looks
      reached when the candle says it never was. On the euro that is a fifth of
      a pip; on gold the spread is ~30 cents, which is most of a band edge
- [x] **A market that has not moved says nothing.** Only a middle that actually
      changed is passed on
- [x] **A refusal cannot be silent.** IBKR does not fail a subscription it will
      not serve — it sends one notice down a line that stays open and then
      never sends a price. That is turned into `Heard::Refused` and reaches the
      watcher, because otherwise it is indistinguishable from a quiet market
- [x] **Delayed prices are refused out loud.** An account without live data is
      served fifteen-minute-old prices *instead of nothing*. Dropped quietly
      the bot goes silent; acted on, it says price is at his level a quarter of
      an hour after it was
- [x] **The farm chatter is ignored.** "Market data farm connection is OK"
      arrives on every connection. Passed on it would report a healthy feed as
      refused every time the bot started
- [x] **A dead line reopens the whole connection.** TWS restarting leaves a
      client that refuses every subscription forever, and subscribing again on
      it fails identically. Only a fresh line fixes it
- [x] **`/chart` comes in on its own client id.** One connection per id at
      IBKR, and a second on the same id throws the first off — drawing a chart
      would have knocked the bot off the feed

### What the first live run showed — 20 August, market open

- [x] **The account gets live forex prices.** EUR/USD bid and ask both
      arriving, no notice, nothing delayed
- [x] **IT SERVES GOLD.** `XAU/USD` goes over as a commodity through SMART and
      the prices come back — bid 4461.39, ask 4461.73. This was the one that
      could have sunk the whole switch
- [x] **The spread on gold is 34 cents**, which settles the mid-price
      question with a real number. Alerting off the bid would have been a
      third of a band out on every gold level
- [x] **Bid and ask arrive as `PriceSize`**, not as separate `Price` ticks —
      both shapes are read, and the session High, Low, Close and Last that
      arrive alongside are ignored. Taking `Close` for a price would have
      alerted off yesterday's number
- [x] **Sixty weekly candles was refused**, and that was a real bug. IBKR
      will not take a duration over 52 weeks written as weeks — over that it
      must be years. **No pair with weekly levels could be sized at all.**
      Fixed, and pinned by eight tests
- [x] **`.env` was silently half-loaded.** `IBAPI_TIMEZONE_ALIASES` has an
      unquoted value with spaces, `dotenvy` refuses that line, and it **stops
      there** — so both Telegram settings, which sit below it, never loaded.
      `dotenv().ok()` threw the reason away. Now it says which line and what
      it is called, and never prints the value

### Still unchecked — [ ]
- [x] ~~**WHERE DOES IBKR PUT ITS DAILY AND WEEKLY BOUNDARIES?**~~ MEASURED
      20 August, market open. `docs/worksheets/ibkr-candles.md`
- [ ] **AND `config/when.toml` IS WRONG — it says 17:00 New York, 21:00 UTC in
      summer.** Gold's day actually starts **22:00 UTC** (17:00 CHICAGO, 6 of
      6 candles agree). EUR/USD starts 21:15 UTC (17:15 New York), though only
      2 of 6 could vote so that one is not settled.

      **One setting cannot serve both** — they are 45 minutes apart, because
      gold rolls on the metals clock and the euro on the forex one. Either
      `day_ends` goes per-pair in `config/pairs/*.toml` beside `approach_pips`,
      or it stops governing candles at all and only governs the calendar.
      **Nobody has traced what actually reads it.** Not changed — it moves
      every band on every pair
- [ ] **Re-measure in November.** Every candle sampled was in August, one side
      of the clock change. A fixed 22:00 UTC and a local 17:00 Chicago look
      identical in summer and are an hour apart in winter Twelve Data's
      day was measured — 17:00 New York — and that measurement **does not
      transfer**. `config/when.toml` still holds 17:00 New York and it is now
      an assumption rather than a finding.

      Getting it wrong does not error. It shifts every daily and weekly band,
      which shifts every level, which changes every signal — and it looks
      completely normal on the way past. Measure it the same way the old one
      was measured: match a daily candle's open against thirty hours of hourly
      candles and see which one it lands on
- [ ] **Does the datetime still mean two things?** On Twelve Data an hourly
      stamp was the candle's open and a daily stamp was the date it *ended* on.
      `Bar::opened_at` is written to that. Unchecked on IBKR
- [ ] **Weekend daily candles** — noise on the old feed, and never handled. It
      already cost one wrong answer: a normal-hour measurement taken on a
      Saturday said gold moves 0.73 an hour instead of 13.33
- [ ] **The bot itself has not run against IBKR.** Ticks and candles both
      come back, but `.env` line 12 has to be quoted before it can say a word
      on Telegram — see below
- [x] ~~**`.env` NEEDS ONE FIX, AND IT IS HIS TO MAKE.**~~ Done. It is line 15
      now and correctly quoted:
      `IBAPI_TIMEZONE_ALIASES="Gulf Standard Time=Asia/Dubai"`. Checked
      24 August

**60 historical requests in any ten minutes** is the limit that shapes
everything now — one every ten seconds sustained, which is why `BREATHE` is ten
seconds. It is slightly *stricter* than the eight a minute before it.

**And IBKR does not refuse when you go over. It PACES.** The request simply
takes longer, and then longer, and a candle report arrives late enough to be
about a candle he has already watched close on his own screen.

---

## Next, in this order

Agreed 16 August, before anything new is added.

### 0b. ~~Ask for a candle when one is due, not every ten minutes~~ — done

**Done 16 August.** `watch/closes/due.rs`, six tests.

**What it does now.** When price is at a zone, it asks the feed for that
pair's 1-hour and 4-hour candles **every ten minutes** and lets the returned
timestamp say whether it is one already reported.

**Why it polls at all.** Nothing tells us a candle closed. The price line
sends prices, about one a second, and never says "the 14:00 hourly just
finished". Approaching a zone, reaching it and sitting in it all cost nothing
— they come off that line. Only *what the candle did* needs a request.

And it deliberately does not work the boundaries out for itself. Nobody has
measured where this feed puts its 4-hour candles, and guessing wrong reports a
candle that has not finished — the one mistake that makes results look better
rather than broken.

**The waste.** A 4-hour candle closes six times a day. Asked every ten
minutes, about **140 of every 144 asks find nothing new**. The hourly is five
in six wasted.

**The fix.** Once the feed hands back a candle stamped 14:00 on the 4-hour, it
has told us where its own boundaries are. The next one is due at 18:00. Wait
until then rather than asking twenty-four times in between.

This does **not** break the rule above. It is reading the feed's own stamp,
not calculating a boundary — and the returned stamp is still what decides
whether a candle is finished, exactly as now.

Per pair per day it went from **288 asks to 60** — the hourly asked twice an
hour instead of six times, the 4-hour twice per candle instead of
twenty-four. And a close report is no longer up to ten minutes late; it lands
when the candle does.

There is a floor of one minute between asks about the same pair, because every
moment worked out from a stale stamp is in the past, and "ask when the next is
due" would then mean "ask again immediately, forever".

---

### 0. TOP PRIORITY — draw cards off the price loop

**Agreed 16 August. This is the next code change, before rung 3 and before
the news.**

Drawing a card runs Chrome and *waits* for it. That wait is a plain blocking
wait sitting inside async code, so for the **2 to 10 seconds** a card takes —
60 in the worst case — one of Tokio's worker threads does nothing but poll
Chrome.

**What it costs today:** on his Mac there are eight or more workers, so prices
keep arriving on the others and queue in the socket buffer. Nothing is lost.
Alerts can be a few seconds late.

**Where it actually bites:**

- **Several cards at once.** Four pairs reaching zones together blocks four
  workers.
- **A small cloud box.** One or two cores and the whole bot stalls while a
  card draws — no prices read, no messages answered. **Hosting is the plan,
  so this is the one that matters.**

**The fix:** `tokio::task::spawn_blocking`. Tokio keeps a separate pool for
exactly this, and work sent there never touches the threads running the price
loop.

**The catch:** the card functions borrow — `&Pair`, `&Band` — and that pool
needs owned values. So each call site clones its inputs first. They are small
structs; the clone is nothing next to running Chrome.

**Six call sites:** the alert, the candle close, the heartbeat, `/status`, the
armed card, the trouble card.

Held back deliberately on 16 August: it touches every path that sends
anything, and the first live session had not been watched yet. **Do it once a
live session has run clean.**

1. ~~**Go over the file structure.**~~ Done 16 August — four files were over
   the 250-line limit, `watch/mod.rs` had an `impl` in it, `bin/cards/` had no
   README, and two READMEs named files that had become folders. Reading
   `watch/run.rs` for it turned up a real bug: **the subscription reply was
   never read**
2. ~~**Read the code back for bugs.**~~ Done 16 August, three passes over
   every file. Thirteen found. Every one loses or corrupts a message rather
   than crashing, which is the only kind of bug this project has — including
   one introduced by the pass before it The last week added the calendar, rungs 1
   and 2, the heartbeat, trouble handling, the reconnect and the inbox fold-in
   — and every careful read-back so far has found something. Two failing
   doc-tests, a spin loop, a secret in an error message, a silent reconnect
   loop, and a level that armed but was never watched were all found this way
   rather than by a test going red.
3. ~~**`/restore`**~~ Done 16 August. Stopping a pair is no longer a one-way
   door from his phone.

~~**`EURUSD` is sitting in `config/pairs/removed/`**~~ — it is back. Checked
24 August: `EURUSD.toml` is in `config/pairs/` with its four weekly levels, and
`removed/` is empty.

---

## A typo used to create a pair that could never work — fixed 20 August

He typed `auduss`. The bot answered **"AUDUSS is new. Which timeframe?"**,
wrote `config/pairs/AUDUSS.toml` with `symbol = "AUD/USS"`, saved the level,
and replied **"Saved. Could not draw the chart just now — the levels are
safe."**

Then nothing, forever, and nothing said why.

- [x] **There was no validation anywhere.** One line — `text.to_uppercase()` —
      and whatever he typed became a pair. `with_slash` splits any six letters
      down the middle and `digits_for` guesses 5
- [x] **The reply was the worst part.** *"just now"* says temporary and
      *"safe"* says it will work. That message was written for a feed hiccup,
      and a pair that can never exist wore it identically
- [x] **It stayed broken quietly.** Every reload tried to size its bands, IBKR
      refused, and the only record was an `eprintln!` to a terminal he is not
      watching — while `/pairs` and the heartbeat listed it as watched
- [x] **Now the broker is asked, because spelling cannot answer it.** `AUDUS`
      and `AUDUSDD` are the wrong length; **`AUDUSS` is the right shape with a
      currency that does not exist**, and that is exactly the typo a thumb
      makes. `IbkrConnection::serves` asks `contract_details`
- [x] **THREE ANSWERS, NOT A BOOL.** Yes, never-heard-of-it, and could-not-ask.
      If the last two were ever folded together, one gateway outage would
      retire every pair he owns
- [x] **Files already on disk are swept once at startup** and moved to
      `config/pairs/removed/` — **moved, never deleted**, back with one
      `/restore`. He is told on Telegram, not in the terminal
- [x] **The symbol is READ from the file, never guessed from its name.** Found
      by reading the sweep back: `/restore` writes `GBPUSD-2.toml`, and working
      that up into a symbol gives nonsense IBKR would rightly refuse — so the
      first version would have retired a pair he was using
- [x] ~~**Two bogus pairs are sitting in his config right now**~~ — `AUDIS`
      and `AUDSSS` are gone. Checked 24 August: `config/pairs/` holds AUDUSD,
      EURUSD, GBPUSD, USDCAD and XAUUSD, and `removed/` is empty
- [ ] **No "did you mean AUDUSD?"** IBKR's contract search can return near
      matches, so the refusal could offer the right spelling as a tap. Not
      built

---

## Still open

**Bugs live in [`docs/bugs/`](docs/bugs/)**, marked 🔴 🟠 🟡. This list is
work not started; that folder is things that are wrong.

- [ ] **Draw cards off the price loop — TOP PRIORITY.** A blocking wait on
      Chrome sits inside async code, so a card holds a worker thread for 2–10
      seconds. Harmless on eight cores, fatal on one. `spawn_blocking`, six
      call sites, inputs cloned first. Full note under *Next, in this order*
- [ ] **Rung 3 — the strategy.** Needs `nsc-strategy`, and needs two answers
      from him: what makes him *skip* a rejection, and where the stop goes.
      Everything else can be built without him
- [ ] **NOTHING HAS RUN AGAINST A LIVE IBKR FEED.** The switch landed
      20 August: it compiles, 203 tests pass, and not one tick has come down
      the line. Run `--bin listen` first — it answers in one line whether the
      account gets forex prices at all
- [ ] **Gold may not be served.** `XAU/USD` is a commodity to IBKR and spot
      metals are a separate market data subscription. He watches gold, and
      there is no fallback feed any more
- [ ] **IBKR's daily and weekly boundaries have never been measured.**
      `config/when.toml` still says 17:00 New York, which was measured on
      Twelve Data. Wrong, it shifts every band and every signal and looks
      perfectly normal doing it
- [ ] **It has never run through a live session.** Rungs 1 and 2, the
      and the heartbeat have only been exercised through
      `--bin cards`
- [ ] **No fallback feed.** Twelve Data went on 20 August. TWS not logged in
      means no bot at all — worth reconsidering once IBKR has been watched for
      a week
- [ ] **Nothing is stored.** No database. A restart forgets every zone it was
      already sitting in, and there is no record to answer "why did nothing
      fire last week?"
- [ ] **Rejected setups are not saved** — that is a `CLAUDE.md` rule and it
      needs rung 3 to exist first

---

## Phase 2 — Postgres and Redis

**Noted 16 August. Not now, and nothing is half-built for it.**

Right now the bot keeps everything in memory and its levels in TOML files.
That is right for what it does today and it stops being right soon.

**Postgres — the record.** Three things need it, and all three are already
written down as missing:

- **Rejected setups get saved, not thrown away.** That is a `CLAUDE.md` rule,
  and it is the file that answers "why did nothing fire this week?". It also
  supplies the "don't take this" examples the Phase 4 model needs
- **Every signal, and what happened next.** Without it there is nothing to
  measure against, and no backtest that means anything
- **Candle history.** The backtester has to read years of candles from
  somewhere that is not an API with a rate limit

**Redis — the fast, throwaway state.** What is in memory today and lost on
every restart:

- **Which zones price is already sitting in.** A restart forgets, and the
  report of where price stands comes out empty
- **Which candles have already been reported**, so a restart does not send
  yesterday's close again
- **Where he is in a Telegram conversation** — the pair, the chart, the flow
  he is part-way through. Lost on restart today

**What must not change when they land.** `nsc-core` never touches either.
No `sqlx`, no `redis`, no `tokio` in that manifest — it is the manifest that
enforces it, not a rule anybody remembers. If a rule needs a row, it gets
handed the row.

The place they plug in is the same one the backtester uses: `BarClosed` in
`nsc-data::events`. One meeting point, so the live bot and the backtester keep
running the same code.

---

## Asked for, not started

Nothing. **The news was the last one** — raised 16 August, built 25 August.
See [The news](#the-news--x-25-august).

---

## Then, in order

- [ ] **Keep it** — Postgres, one table of candles, written as they arrive
- [ ] **The past** — download history per timeframe, and a scan that says
      whether it is complete before anything reads it
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

**Round to the instrument, and write it one way.** The feed sends gold as
`4385.59525`; gold is quoted to two. And a card saying *4,094.00* was captioned
*4094* — the same number, twice, looking like two prices.

**A reply that parses is not a reply that worked.** Twelve Data refuses with a
normal-looking `{"code": 401}`. Telegram refuses with a polite `ok: false`.
Both in one afternoon, so it is a pattern rather than bad luck.

**A message the bot cannot send is the same as no message.** The heartbeat
died with the process it was meant to report on, and silence is exactly what a
quiet day looks like.

**Secrets travel in error messages.** The rule "never print the url, the key is
in it" was written down, followed on the happy path, and never applied to the
error path — which is the one that prints.

**Count the failures, not the passes.** Two doc-tests failed for days while
every report said "112 tests, clippy clean" — because the count came from
`grep "^test .* ok$"`, which cannot see a failure. The honest check is to read
the `test result:` lines and look for `FAILED`.

**An indented block in a doc comment is Rust.** rustdoc compiles it. A table of
what each colour means and a list of `cargo run` lines were both being fed to
the compiler. They need a ```text fence, and only the binaries' docs are
compiled — which is why it went unnoticed.

**Search the string, not the code that prints it.** Two lines were missed in a
wording sweep because `rustfmt` had split their `println!` across lines. The
replace matched nothing, changed nothing, and reported success.
