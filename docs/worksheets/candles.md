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

The code adds a fourth, **Plain**: almost no wick either side. A candle that
opened, closed and went nowhere at all. It is not a textbook name, it is what
is left when the other three do not fit, and it is better than filing that
candle as a long-legged doji when it has no legs.

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

## The conflict with the swing finder — settled

A tweezer top is two candles with the same high, and that used to be the exact
shape swing detection threw away. The old finder refused ties: when two candles
shared the highest high, neither strictly beat the other, so neither became a
swing.

**Settled by the swing rewrite on 12 Aug 2026.** The finder no longer compares
neighbours at all — it tracks a running extreme and proves it with the
pullback, so the first of two equal highs simply is the extreme. A tweezer top
can be a swing high now, and the two modules no longer disagree.

---

## What this changes in the code

`nsc-ta/src/candles/` currently stubs five patterns. Against your list:

| Stub | What happens |
|---|---|
| `pin_bar.rs` | Kept. Covers hammer and inverted hammer too. |
| `engulfing.rs` | Kept. |
| `doji.rs` | Kept, with the three variants. |
| `inside_bar.rs` | Left untouched — see below. |
| `star.rs` | Left untouched — see below. |
| — | **Add** `belt_hold.rs` |
| — | **Add** `tweezers.rs` |

**Still open:** you did not mention inside bars or morning/evening stars, and
did not answer when asked. Both stubs are left exactly as they were rather
than deleted or quietly built. Deleting a stub is cheap; finding out in six
months that the bot never looked for a pattern you trade is not.

---

## What this became

**Built 12 Aug 2026.** `nsc-ta::candles` has all six, driven by a `[candles]`
section in `config/ta.toml` — ten settings, eight of them shares of a candle
and two in ATR. Twenty-four tests on the detectors, eight on the types.

The numbers in that file are marked as textbook so nobody later reads `2.0`
and assumes somebody chose it.

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
4. **The tweezer tolerance.** 0.05 of a normal candle is the placeholder. Two
   candles never share a high to the tick, and how close counts as the same
   price is a judgement nobody has made yet.
