# nsc_trades — project rules

Reads forex charts and sends trading signals to Telegram. Rust workspace.
**Version 1 sends signals and places no trades.** Trading is Phase 6 and does
not exist in this code.

**There is no code right now.** It was cleared on 14 August 2026 and is being
rebuilt against a settled data design: the broker's chart is the truth, every
timeframe arrives on its own websocket subscription, and nothing is computed
from anything else. The old code is in git at `a4a2170` if you want to read it
back — it was right about its own job and wrong about where candles come from.

These rules survived unchanged. They are what the rebuild gets measured
against.

---

## Explain everything in plain English

Write for someone who trades, not someone who writes compilers.

- **Short sentences.** One idea each.
- **Explain a technical term the moment you use it**, in the same sentence.
  Not "avoid lookahead bias" but "avoid lookahead bias — using price the
  market had not printed yet".
- **Say what it does before how it works.**
- **Give a concrete example** instead of an abstract description. "20 pips
  means something different on GBPJPY than on EURUSD" beats "thresholds must
  be normalized across instruments".
- **Use the plain word.** "Stops working on other pairs", not "fails to
  generalize". "Runs out of order", not "exhibits non-deterministic
  interleaving".
- **Name the consequence.** Do not say a rule exists; say what breaks without
  it.

This means simpler **language**, not simpler **content**. The ideas in this
project are genuinely technical and dropping them would make the system
worse. The goal is that any explanation can be read once and acted on.

Applies everywhere: chat replies, code comments, `//!` module docs, commit
messages, and every worksheet.

---

## The two rules that override everything else

### 1. `nsc-ta` and `nsc-strategy` never touch the outside world

No database, no internet, no async, no reading the clock, no global state.
If either crate gains `tokio`, `sqlx` or `reqwest` in its `Cargo.toml`, the
change is wrong.

This is what lets the backtester and the live bot run the *same* code. **Never
write "if we're backtesting, do this instead".** The moment you do, your
backtest is testing something different from what runs live — and you will not
spot it from the results, because the mismatch makes backtests look better,
not broken.

### 2. Never use price the market hadn't printed yet

- A swing high can only be used at or after the candle that confirmed it.
- Candles that are still forming are invisible to the analysis.
- A 4-hour candle does not exist until its last 15-minute candle has closed.
- Analysing candle 100, you may not read candle 101.

This mistake does not cause an error. It makes your backtest look *better*.
That is what makes it dangerous. `nsc-backtest::guards` stays on in every test
run, and `LookaheadDetected` kills the run completely — a bad number with a
warning attached still gets read and acted on weeks later.

---

## Architecture rules

- **Who can use what:** `nsc-core` ← `nsc-ta` ← `nsc-strategy` ← the drivers.
  The clean crates never reach down into the messy ones. If `nsc-ta` needs a
  database row, hand it the row.
- **`nsc-core` holds the shared types.** Never define a second `Candle` or
  `Level` somewhere else.
- **One meeting point:** the backtester and the live bot both go through
  `BarClosed` in `nsc-data::events`.
- **Rules are applied in exactly one place** — `nsc-strategy`, run by either
  `nsc-live` or `nsc-backtest`. `nsc-api` never applies rules.
- **Brokers hide behind `MarketDataSource`.** Nothing outside
  `nsc-data::sources` knows which broker you use.

---

## Conventions

**Measure in ATR, never in pips.** ATR is the size of a normal candle. Every
distance and tolerance is a multiple of it — how close counts as "at the
level", how much room the stop gets, how big a candle is too big.

Why: 20 pips is a big move on EURUSD and a small one on GBPJPY. A pip setting
works on the pair you tested and quietly stops working on every other one.

**Keep a file under 200 lines. 250 is the hard limit.** Counting everything —
doc comments, tests, the lot.

Past that, a file is a thing you scroll rather than a thing you read, and the
part you need is never on screen at the same time as the part it depends on.
The bugs that hurt in this project are the quiet ones — a boundary an hour
out, a candle read one too early — and quiet bugs hide in files nobody reads
end to end.

**And no more than 170 lines of actual code.** This one counts only lines that
are neither blank nor a comment. Written out, a line counts unless it is empty
or starts with `//`, `///` or `//!`.

Two limits, because they stop two different things.

The 250 is about the *file* — how far you scroll to hold it in your head.
The 170 is about the *thinking* — how much is actually going on in it. A file
can pass the first and fail the second: 240 lines with barely a comment in it
is a file doing far too much and telling you nothing about why.

**Explaining does not count against you, and that is deliberate.** The prose
in this project is not decoration — it is where the reasons live, and the
reasons are what stop the same bug being reintroduced next month. A limit that
punished a doc comment would quietly train everybody to delete them, and the
first thing to go is always the paragraph explaining why the obvious approach
was wrong.

So: write as much explanation as the thing needs. If a file is over 170 lines
of code, it is not because it is well documented — it is because it is doing
more than one job, and it wants splitting.

```sh
# what is doing the most, code only
find crates -name '*.rs' | while read f; do
  printf '%4d %s\n' "$(grep -cvE '^\s*(//|$)' "$f")" "$f"
done | sort -rn | head
```

**A module that defines a type and has tests is a folder, not a file.** If it
has a `struct` or an `enum` in it *and* a `#[cfg(test)]` block, it starts as a
folder — whatever its length.

```
swing/
  mod.rs        module docs, and what the outside world can see
  kind.rs       SwingKind — high or low
  point.rs      Swing — the swing itself
  tests.rs
  README.txt
```

Do not wait for it to get long. Splitting is cheap while there are three
things in it and a nuisance once there are twelve, and by then everything
importing it has to be touched too.

A file that only declares errors, or only holds one small function, stays a
file. It is types **and** tests together that earn a folder.

**`mod.rs` is a front door, not a room.** Module docs, which files are inside,
and what the outside world can see. No types, no logic. That way one screen
tells you what a folder contains without opening anything.

Each file holds one idea **and the behaviour that goes with it**. Do not put
every struct in a `types.rs` and every function in a `logic.rs` — that splits
things that are read together and joins things that are not.

**Every folder with code in it gets a `README.txt`** saying what the folder is
for, what each file does, and how they connect. Plain English, no jargon.

A folder of empty stubs does not get one. Describing code that does not exist
is the same lie as describing code that no longer does.

Keep it true. A `README.txt` that describes files that no longer exist is
worse than none at all, because it is believed. **If you add, remove or
rename a file in a folder, update its `README.txt` in the same change.**

**Finish a piece of work, update `PROGRESS.md` in the same change.** Tick what
is done, mark what is half done, and correct the test count. It does not exist
right now — the first piece of work that lands creates it.

Same reason as the READMEs. A progress file that says something is still
missing when it was finished last week is worse than not having one, because
the next decision gets made against it. It is the file you look at to answer
"what now", so it has to be true on the day you look.

**Errors:** libraries use typed errors (`thiserror`); the two binaries use
`anyhow`. The point of separate error types is to tell the caller *retry or
give up*. Never lump a bad API key in with a network timeout — the bot will
retry the bad key forever and it looks exactly like a dead connection.

**No `unwrap`, `expect` or `panic!` in library crates.** The backtester runs
this code over years of candles. One bad candle must not destroy two hours of
work. Tests and binaries may panic.

**Everything is UTC.** The daily close time is applied once, in
`nsc-core::timeframe`, from `config/app.toml`. Never work it out again
somewhere else.

**Anything a trader would tune goes in `config/`, not in the code.** If you
are about to write a named constant for a threshold, it belongs in a TOML
file.

**Rejected setups get saved, not thrown away.** Save which layer rejected
them. Those rows answer "why did nothing fire this week?" and they are the
"don't take this" examples the Phase 4 model needs.

**Every signal must be explainable in one sentence.** If `reasons.rs` cannot
write that sentence, the rules are too loose. Fix the rules, not the wording.

---

## Finished is not finished until you have read it twice

When a piece of work is done, go back over it **before** saying it works.

Not by running the tests again. Green tests are what you had a minute ago, and
they only check what somebody thought to ask. Three real bugs have been found
in this project by reading code back against the worksheet that describes it,
and the tests were passing for all three:

- a swing peak thrown away when price crashed straight through the start of
  the run
- two flat candles inventing a swing, on every history, at the left edge
- a failed attempt at a level silently dropped when a newer swing replaced it

The second pass asks four things:

1. **Does the code do what the worksheet says**, line by line? Not roughly —
   the worksheet is the specification and the code is the guess.
2. **Is every README, worksheet and config comment still true?** File lists,
   test counts, settings that moved or were renamed. A README that describes
   something that is no longer there is worse than none, because it is
   believed.
3. **What did this change make possible that nothing checks yet?** New states,
   new orderings, the first candle, the last candle, an empty list.
4. **Does any type still mean what its name says?** Borrowing `AtrMultiple` to
   multiply by a run is right arithmetic under a wrong name, and the next
   person reads the name.

Anything found this way gets its own test — one that fails without the fix.
Check that it does fail. A test that passes either way pins nothing.

---

## The AI layer specifically

- **It never does arithmetic.** Levels, distances, risk-to-reward and stop
  placement are worked out by normal code and handed over as finished facts.
  Ask an AI to measure something and it returns a confident, believable, wrong
  number.
- **It can never approve.** It may lower confidence, add a warning, or block a
  trade. It may not rescue a setup your rules rejected. If it can, the AI has
  become your strategy — which is the exact thing this design prevents.
- **It fails safe.** Timeout, rate limit, bad response → skip the check, note
  it on the signal, and send based on the chart alone.

---

## Commands

```sh
cargo check --workspace          # quick feedback
cargo test --workspace           # golden files and rule tests
cargo clippy --workspace -- -D warnings
cargo fmt --all
```

The record — **these work**, as of 30 August 2026:

```sh
docker compose up -d                   # Postgres, on 5434
cargo run -p nsc-work-man --bin keep   # saved candles into the record
cargo test -p nsc-data -- --ignored    # the queries, against a real database
```

**These still do not exist:**

```sh
cargo run -p nsc-backtest              # test rules against history
```

The bot itself is `cargo run -p nsc-work-man` — his name for the crate, not
`nsc-live`. Downloading history is `--bin history` on that same crate, not a
`backfill` binary on `nsc-data`.

---

## Never

- Commit `.env`, API keys, or broker logins.
- Regenerate golden test files without reading what changed. An unread golden
  file is a silent change to the part of the system everything depends on.
- Guess in your own favour when stop and target were both hit in one candle.
  Mark it `ambiguous` and leave it out of the numbers.
- Reload settings while the bot is running. Restart instead — the restart is
  how you know which rules produced which signals.
- Add trading code. Version 1 sends signals only, and `features.execution`
  being `false` is not a gap waiting to be helpfully filled in.
