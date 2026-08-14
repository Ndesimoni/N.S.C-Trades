---
name: signal-design
description: Use when changing what the bot sends to Telegram — the wording or layout of an alert or a signal, the chart picture that goes with it, or how a number is rounded and shown. Also use when he says a message looks wrong, small, cluttered or unprofessional.
---

# Making a message worth reading on a phone

Everything the bot sends lands on a phone, at a glance, often while he is
doing something else. It has one job: **be understood without being studied.**

## Who is reading it

A trader who reads charts. Ninety percent of his decisions are technical
analysis, off the chart itself.

That has a consequence worth accepting rather than fighting: **text will never
look professional to him.** You can tidy it and you should, but there is a
ceiling, and the ceiling is low. The picture is what he actually reads.

So: keep the text short and let the chart do the work.

## What Telegram actually gives you

There is no font size. There is no colour. There is no layout. Knowing that
saves an afternoon of trying.

| Works | Notes |
|---|---|
| `<b>` | full reading size — this is your "big" |
| `<i>` | quieter, good for a timestamp |
| `<blockquote>` | draws a vertical accent bar. The best structural tool available |
| `<code>` | monospace inline, renders **smaller** |
| `<pre>` | aligned columns, renders **smaller** — the trade is size for alignment |
| `━━━` | a rule made of characters. Cheap and it works |
| emoji | render large. Use at most one, or none |

`<pre>` was tried first and rejected — it aligns beautifully and is too small
to read on a phone. **Size beats alignment.**

## What worked, and why

```
XAU/USD
━━━━━━━━━━━━━━━━━
1 HOUR  ·  14 Aug, 18:00 UTC

▌ 4377.78
▌ ▼ 7.82  ·  0.18%

O   4385.60
H   4387.45
L   4376.56

Range   10.89
```

- **The pair alone on the first line.** What, before how much.
- **Blockquote around the price and the move.** Separates the number that
  matters from the supporting detail, without a bigger font.
- **`O H L`, not `Open High Low`.** Labels of different widths push every
  number to a different place, and the column falls apart. Short labels of
  equal width keep it, even in a proportional font.
- **One idea per block**, with a blank line between.

## Round to the instrument, always

Twelve Data sends gold as `4385.59525`. Five decimals is the raw feed. **Gold
is quoted to two.**

Printing every decimal the feed sends is the single thing that makes a message
look like a debug dump rather than a signal. It was also the biggest single
improvement when fixed.

Digits belong in `config/`, one per pair — gold 2, EURUSD 5, USDJPY 3. Never
guess a rounding in the formatting code.

## An alert is not a signal

They come from different lanes and must not look alike.

**An alert** says *your zone is live*. Price reached a level you drew. It has
no entry, no stop and no target, because there is no trade — only a reason to
look. It may fire on a candle that has not closed.

**A signal** says *your rules matched*. It has entry, stop, target, the reward
ratio, which strategy, and **one sentence you could argue with**. It only ever
comes from a closed candle.

If those two ever start looking the same, the alert has quietly become a
signal, and the price watcher has become a strategy nobody reviewed.

## The cards, and which message sends which

Each card in `assets/card/` is a whole picture that stands up on its own. They
are pieces, not a set — **a message picks the ones it needs.**

| Message | Cards |
|---|---|
| Price watcher — *your zone is live* | its own card: the level touched, price now. No chart; nothing has formed |
| A candle closed | `chart.html` alone. It carries its own open, high, low and range |
| A signal | the chart with your levels and the entry, stop and target on it, plus a card carrying the reasoning |
| Detail on request | `readout.html` — where price sat inside the candle |

Several pictures go as a **media group**, not several messages. One buzz, and
each picture opens on its own when tapped.

### Two things about drawing them

**The card's height lives in its own CSS**, as `--card-height`. Rust reads it
out of the file. Two numbers in two files drift apart; one does not.

**Chrome's headless mode always paints 87 pixels of white** below the page —
it hands the page a viewport that much shorter than the window asked for. Ask
for 87 extra and cut them off. The mode that did not do this has been removed
from Chrome.

## The chart picture

From step 5 a signal carries a picture. Rules for it:

**Draw what the bot read, not a prettier version.** If a level sits in the
wrong place or a swing appears where he sees none, the picture should show
that. Every signal is then a check on the code, and he will spot in a glance
what no table of numbers would ever reveal.

**His level colours are a specification, not a choice.**

| Colour | Timeframe |
|---|---|
| Black | Weekly |
| Blue | Daily |
| Yellow | 4-hour |

Drawing every level in one colour was done once already and the chart looked
nothing like his.

**Nothing he does not use.** No indicators, no volume — spot forex and gold
have none — no decoration.

**Say what it is.** It is our drawing of the broker's candles, not a photo of
his platform. If a *price* differs from his chart that is a real bug worth
chasing. If the *look* differs, that is just us.

## Before sending anything new

Ask what his eye lands on first. If it is not the thing the message is about,
move that thing to its own line or put it in a blockquote — those are the only
two levers there are.

Then send one to the channel and look at it on a phone. Not in a terminal: a
terminal renders none of the tags and every line is the same width, so it
tells you nothing about how it reads.
