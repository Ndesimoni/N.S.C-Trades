# docs/

Everything about this project that is not code.

**Click a link below to see the design.** Each one is published on the web and
also kept in this repo — so it can still be changed when the thing it describes
changes.

---

## The design

### [Broker to Telegram →](https://claude.ai/code/artifact/1093ff9f-f3b3-4af7-afd5-6377629ea1dd)

**The whole of phase one on one page. Start here.**

- The **two lanes** — the price watcher on every tick, the analysis on closed
  candles only — and the gate between them
- **What sits inside each stage**, eight cards, no arrows
- **Where your rules live** — the six layers a setup has to survive, and why
  reversal, breakout and trend are the same six answered differently
- **What lands on your phone**, an alert and a signal side by side
- **The build order**, and what each step proves before the next one starts

Source: [`diagrams/plan.html`](diagrams/plan.html)

### [The clock, not the stamp →](https://claude.ai/code/artifact/9088bc46-0abf-4d19-b33e-7f3ba4d2895a)

Why a candle's timestamp does not say when it became true. A 4-hour candle
running 21:00 to 01:00 is stamped 21:00 — but nobody knew what it would do
until 01:00.

Comparing stamps fails **both ways**: 4-hour readings arrive four hours early,
and 15-minute swings that plainly happened get thrown out.

**Still true.** It was drawn for the version of this project that was cleared
out, and the rule it argues is the one `Bar::is_finished` obeys today.

Source: [`diagrams/clock-not-stamp.html`](diagrams/clock-not-stamp.html)

---

## What the feed actually sends

[`worksheets/twelve-data.md`](worksheets/twelve-data.md)

Not a picture, but it is the thing to check a number against. Every line was
measured by making the request and reading the reply, not read off their
documentation.

Where their day ends, why the newest candle is always still forming, the two
different meanings of the datetime field, and the weekend candles that are
noise.

---

## The cards, as they were last drawn

Written fresh every time the bot runs. Not in git — they are output.

| | |
|---|---|
| [`preview/chart.png`](../preview/chart.png) | the picture that went to Telegram |
| [`preview/chart.html`](../preview/chart.html) | the same card as a web page, with real numbers in it |

**Open the `.html` one in Chrome.** Edit
[`assets/card/chart.html`](../assets/card/chart.html), refresh, see the change
— no rebuild, no Rust, no running the bot.

That loop is the whole reason the design lives in HTML.

---

## Keeping this true

**A picture of a rule that has since changed is worse than no picture, because
it gets believed.**

When a rule changes, come back here. Either update the page and republish it to
the same address, or say underneath it what replaced it.

That is not a hypothetical. An earlier diagram in this project described a
swing rule that had already been replaced, and it sat there being wrong.
