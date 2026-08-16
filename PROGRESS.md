# Progress

Where the project actually is, as of **16 August 2026**.

Updated whenever a piece of work finishes — that is a rule in `CLAUDE.md`. A
progress file that is out of date is worse than none, because the next decision
gets made against it.

```
[x]  done
[~]  started, not finished
[ ]  not started
```

**Two crates · 143 tests · clippy clean · it watches his levels, says what
happens at them, and tells him when it cannot.**

```
nsc-core        what the bot knows      no reqwest, no tokio — it CANNOT reach
nsc-work-man    everything that reaches
```

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
```

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

**What he can send it** — `/status`, `/help`, `/pairs`, `/level`, `/remove`,
`/restore` — is written up for him in
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
| **·** | a third of the way into that candle | the same card, marked *so far* — hollow chip, hollow candle, *not a close* |
| **3** | it closed there **and** a strategy matched | **not built.** Needs `nsc-strategy`, which needs his answers |
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
- [x] **A 4-hour candle does not exist until its last hour has closed.** Three
      hourly closes can pass with the 4-hour silent
- [x] **The twenty-minute look** — the same candle, part-way through, marked
      *so far*. `look_in_minutes`, scaled per timeframe
- [x] **It is the only place that reads an unfinished candle**, and the card
      says so on its face. It must never reach a strategy
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
      it holds and what can be done to it — add, take one off, stop watching
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

crates/nsc-work-man/      EVERYTHING THAT REACHES
  main.rs       four lines — it runs the watcher
  watch/        THE BOT. Rungs 1 and 2, the calendar, the
                heartbeat, reconnecting, and picking up a
                level he sends while it runs
  inbox/        the other side of Telegram — his levels,
                stopping a pair, /status and /help. Spawned
                beside the watcher                         6 tests
  card/         filling a template, letting Chrome draw   14 tests
  feed/         asking Twelve Data                         7 tests
  telegram/     sending — words, pictures, media groups    3 tests
  retry/        trying again. Lives here BECAUSE IT SLEEPS 3 tests
  review.rs     one pair's levels, drawn
  bin/cards/    draw ANY card without waiting for anything
  bin/levels.rs draw a pair's bands on demand
  bin/listen.rs the raw price stream, kept as proof

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

## What the feed actually does — [x]

All measured, none read off their documentation.
`docs/worksheets/twelve-data.md`.

- [x] **Their day ends at 17:00 New York.** Checked by matching the daily
      candle's open against thirty hours of hourly candles — exactly one
      matched
- [x] **Their week opens Sunday 17:00 New York**
- [x] **The newest candle is always still forming**, and skipping the first in
      the list is not the fix — position is right *most* of the time, which is
      worse than being wrong always
- [x] **The datetime field means two different things.** An hourly stamp is the
      candle's open. A daily stamp is the date it *ends* on
- [x] **The websocket works and it changes the cost of everything.** Prices
      cost nothing, so a request only happens when price reaches a level
- [ ] **Weekend daily candles exist and are noise** — ranges of 0.57 and 1.32
      against 60–200 on a real day. **Not handled.** It has already cost one
      wrong answer: a normal-hour measurement taken on a Saturday said gold
      moves 0.73 an hour instead of 13.33, and nearly settled a rule the wrong
      way
- [ ] **Gold has never been watched ticking** — every socket test so far has
      been on a shut market or on BTC/USD

**8 requests a minute** is the limit that shapes everything. Friday 21:00 UTC
is the worst moment of the week: the hour, the 4-hour, the day and the week all
end on the same second.

---

## Next, in this order

Agreed 16 August, before anything new is added.

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

**`EURUSD` is sitting in `config/pairs/removed/`** with all four levels,
stopped from his phone on 16 August. Not put back — that is his call.

---

## Still open

- [ ] **Rung 3 — the strategy.** Needs `nsc-strategy`, and needs two answers
      from him: what makes him *skip* a rejection, and where the stop goes.
      Everything else can be built without him
- [ ] **It has never run through a live session.** Rungs 1 and 2, the
      twenty-minute look and the heartbeat have only been exercised through
      `--bin cards`. First real test is Monday 21:00 UTC, when Tuesday opens
- [ ] **OANDA** — applied 14 August, nothing back. Worth having because it
      marks each candle finished or not, so the guessing stops
- [ ] **Nothing is stored.** No database. A restart forgets every zone it was
      already sitting in, and there is no record to answer "why did nothing
      fire last week?"
- [ ] **Rejected setups are not saved** — that is a `CLAUDE.md` rule and it
      needs rung 3 to exist first

---

## Asked for, not started

- [ ] **The news.** What is coming up and what it says — so a level sitting in
      front of a rate decision is not read the same way as one on a quiet
      Thursday. Raised 16 August, deliberately left until the rest is settled

---

## Then, in order

- [ ] **Keep it** — Postgres, one table of candles, written as they arrive
- [ ] **The past** — download history per timeframe, and a scan that says
      whether it is complete before anything reads it
- [ ] **Read the chart** — swings, candle types, structure
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
