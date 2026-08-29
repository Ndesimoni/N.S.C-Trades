---
name: read-the-chart
description: Use when he asks what is happening on a chart right now — what shapes are forming, what the last few candles did, how a pair is behaving, or "read gold for me". Also use when a shape on his screen does not match what the bot says it is.
---

# Reading a chart, without guessing

**Run the code. Report what it found. Never read the picture yourself.**

```sh
cargo run -p nsc-work-man --bin read -- XAU/USD 4h
cargo run -p nsc-work-man --bin read -- EUR/USD 1h 120
```

It fetches real candles from IBKR, runs `nsc-ta` over them, and prints the
name the **code** gave each one, plus a tally of what turned up.

## Why it is a command and not a look

Ask any model to eyeball a chart and name what it sees and you get a
confident, believable answer that nothing checked. That is the same rule
`CLAUDE.md` applies to the AI layer: **it never does arithmetic.** Shapes,
distances and sizes are worked out by normal code and handed over finished.

So the answer to "what is forming on gold" is the output of that command. If
the output disagrees with his eye, **the thresholds are wrong, not the
reading** — they live in `config/candles.toml` and they are still textbook
defaults nobody has chosen.

## Before it will run

- **TWS or IB Gateway must be logged in.** There is no feed without it.
- **One connection per client id.** If the bot is running it holds the id from
  `.env`, and this will be refused with `early eof`. Come in on a spare:
  `IBKR_CLIENT_ID=9 cargo run …`

## What you may say, and what you may not

**May.** The names in the output. The tally. The reach of a candle in ATR.
That a shape is common or rare in the window read.

**May not:**

- **The trend.** Swings were removed on 29 August 2026. Nothing here can say
  which way a chart is going, so never answer "is gold bullish". Say it is not
  built.
- **A pattern the code does not detect.** Engulfing, tweezers, harami, stars —
  none of them exist yet. `nsc-ta` names ONE candle at a time.
- **Hammer or hanging man.** They are the same candle; what separates them is
  the trend before it, which is not built. The code says `long lower wick` and
  so should you.
- **Anything about the still-forming candle as though it were settled.** The
  newest row is marked `STILL FORMING` and is left out of the tally. Its shape
  is not its shape yet — a doji ten minutes in is a doji that has not
  happened.

## Reading the tally honestly

The tally is the part worth reading, and it usually says the same thing:

```
  plain                 11   45%
  long body              6   25%
  belt-hold              2    8%
```

**A shape filling half the column is describing the market, not marking it.**
On three years of gold a spinning top turned up 408 times — one candle in ten.
A rule built on that fires every day and means nothing.

So when he asks what is forming, the useful answer is usually short: which
shapes appeared, which of them are rare enough to be worth a second look, and
**what level they printed at** — which this command does not know and does not
pretend to.

## If a shape looks wrong

That is a real bug report and it belongs in `config/candles.toml`. Two were
found this way already, both against real candles rather than invented ones:

- The clearest **dragonfly** and **gravestone** in three years both came back
  as plain dojis, because "no wick" was set at 0.05 — right at the end of a
  marubozu, far too tight beside a tail of 0.90.
- The clearest **closing marubozu** read as a full marubozu, a shape found
  seven times in 4,165 candles.

Change the number, run `cargo test -p nsc-ta`, and check the eighteen real
candles in `crates/nsc-ta/src/candle/tests/real.rs` still get the names they
should. **Never loosen a threshold until a shape appears** — that is fitting
the rule to the wish, and it is why `high wave` is left at one example in
three years rather than widened until it found more.
