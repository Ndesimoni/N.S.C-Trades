# Worksheet — Candlestick patterns

The candles you read. Captured 12 Aug 2026.

Different from the strategy worksheets. This one describes **shapes**, and
becomes `nsc-ta::candles` — code that measures a candle and reports what it
is. It has no opinion about buying or selling.

---

## What you told me

### The six you use

1. **Pin bar** — bullish or bearish
2. **Engulfing** — bullish or bearish
3. **Hammer / inverted hammer**
4. **Doji**
5. **Belt-hold line**
6. **Tweezer tops and bottoms**

That is the whole list. These are the shapes the bot looks for and nothing
more.

### These are not a strategy

Your words: knowing an engulfing candle is a candle engulfing another one does
not tell you to buy. A buy or a sell comes from many factors together, and
that is a strategy, built later.

So the numbers below are **standard textbook measurements**, chosen because
you said to use them for now. They are not your measured thresholds and must
not be treated as such. They live in `config/ta.toml` so they can be replaced
by yours without touching code, once you have examples.

### What that means for where things go

| Question | Answered by |
|---|---|
| Is this candle a pin bar? | `nsc-ta::candles` — a shape |
| Is the wick long enough? | `nsc-ta::candles` — a measurement |
| Did it happen at a level? | `nsc-strategy` |
| Was there a downtrend before it? | `nsc-strategy` |
| Is it a reason to buy? | `nsc-strategy` |

**The detector never looks left.** Textbook descriptions usually bolt the
context onto the pattern — "a hammer after a downtrend". That half is the
strategy's job. The same candle in open space is still a hammer; it is simply
not a trade.

---

## The textbook definitions being used

Two different yardsticks, on purpose:

- **Shape** is measured against the candle's own range. A body that is 20% of
  its candle is 20% on EURUSD and on gold, so these need no ATR.
- **Size** — whether a candle is big or small at all — is measured in ATR,
  like everything else in this project.

### Pin bar

A long wick with a small body at the far end of it.

- The tail is at least **2 times** the body.
- The body is no more than **a third** of the whole candle.
- The body sits in the **third of the candle furthest from the tail**.
- The wick on the other side ("the nose") is no more than a **quarter** of
  the candle.

Bullish means the tail points down. Bearish means it points up.

### Hammer and inverted hammer

**Same shape as a pin bar.** A hammer is a bullish pin bar; an inverted hammer
has the long wick on top.

Textbook separates them by what came before — a hammer follows a downtrend, a
shooting star follows an uptrend — and that is context, not shape. So they are
one detector with a direction, and the trend part belongs to the rules.

**Open:** if a hammer means something different to you *as a shape*, this is
wrong and it splits into two detectors.

### Engulfing

The second candle's **body** completely covers the first candle's body, and
the two are opposite colours.

- Bodies only. Wicks are ignored — that is the standard definition.
- The first body must be a real body, not a doji, or almost everything
  engulfs it.

### Doji

Open and close in nearly the same place: the body is no more than **5%** of
the candle's range.

Three named variants, same rule with different wicks:

- **Long-legged** — long wicks both sides
- **Dragonfly** — long lower wick, almost no upper
- **Gravestone** — long upper wick, almost no lower

### Belt-hold line

A long candle that opens at one extreme with no wick there.

- Bullish: opens at its low, closes near its high.
- Bearish: opens at its high, closes near its low.
- The wick on the opening side is at most **5%** of the range — in real forex
  there is nearly always a tick or two, so "none" cannot mean exactly zero.
- The body is at least **60%** of the range.
- "Long" is measured in ATR: the candle is at least **one normal candle** tall.

### Tweezer tops and bottoms

Two neighbouring candles that reach the **same** high (top) or the same low
(bottom).

- They are never identical to the tick, so "the same" needs a tolerance:
  **0.05 of a normal candle** to start with.
- Opposite colours, per the textbook.

---

## One conflict to settle before this is built

A tweezer top is two candles with the same high.

The swing finder deliberately **refuses ties** — see
`a_flat_top_produces_no_swing`. When two candles share the highest high,
neither strictly beats the other, so neither becomes a swing. Missing a level
was judged safer than inventing one.

So the exact shape a tweezer looks for is the shape swing detection throws
away. Both can be right, but they must agree on what "the same price" means,
and right now only one of them has a tolerance for it.

---

## What this changes in the code

`nsc-ta/src/candles/` currently stubs five patterns. Against your list:

| Stub | What happens |
|---|---|
| `pin_bar.rs` | Kept. Covers hammer and inverted hammer too. |
| `engulfing.rs` | Kept. |
| `doji.rs` | Kept, with the three variants. |
| `inside_bar.rs` | **Delete** — not on your list. |
| `star.rs` | **Delete** — morning and evening star not on your list. |
| — | **Add** `belt_hold.rs` |
| — | **Add** `tweezers.rs` |

**Open:** you did not mention inside bars or morning/evening stars. Confirm
they go, rather than that they were just missed. Deleting a stub is cheap;
finding out in six months that the bot never looked for a pattern you trade is
not.

---

## What this becomes

`nsc-ta::candles`, driven by a new `[candles]` section in `config/ta.toml`,
which does not exist yet.

Every detector reports the pattern **and its measurements** — how long the
wick was, how much of the range the body took. The rules layer needs those
numbers to tell a marginal pin bar from a textbook one, and it cannot get them
back from a yes or no.

---

## Still open

1. **Your own numbers.** These are textbook defaults. The screenshots that
   would replace them are one you took and one you passed on that looked the
   same.
2. **Is a hammer a different shape to you, or a pin bar after a downtrend?**
3. **Do inside bars and stars go?**
4. **The tweezer tolerance**, and whether swing detection should use the same
   one.
