# Worksheet — Candlestick patterns

The candles you read. Captured 12 Aug 2026.

Different from the strategy worksheets. This one describes **shapes**, and
becomes `nsc-ta::candles` — code that measures a candle and reports what it
is. It has no opinion about buying or selling.

**The picture:** [the candlestick shapes, and two still undecided →](https://claude.ai/code/artifact/9f9ef70d-c0d6-4bfb-b621-25a26f94339a)
— every shape drawn with the measurement that makes it one.

---

## What you told me

### The ones you named

1. **Pin bar** — bullish or bearish
2. **Engulfing** — bullish or bearish
3. **Hammer / inverted hammer**
4. **Doji**
5. **Belt-hold line**
6. **Tweezer tops and bottoms**

And, added 12 Aug 2026 when asked directly:

7. **Inside bar**
8. **Morning and evening star**

### The list is not finished, and it is not meant to be

Your words: you trade many other candlestick patterns combined with other
factors, and **you do not know all their names**. The plan is that training on
your labelled trades picks them up and names them later.

That is a better plan than guessing, and it changes what this code is for.

**What it means now.** Nobody adds a detector nobody asked for. A shape in the
code that you would not act on is noise in every backtest it appears in, and a
rule nobody can explain is exactly what this project is built to avoid.

**What it means for Phase 4.** A model cannot learn a shape from the word
"engulfing". It learns from measurements. So every sighting carries the
proportions of the candle it was found on — body, upper wick and lower wick as
shares of that candle's height — rather than just a name.

**The gap, and it is worth naming now.** Those measurements are only recorded
for candles that matched one of the eight. A shape with no name yet produces
nothing, so there is nothing for a model to find it in.

Fixing that is cheap and it has to happen before the data is collected, not
after — the same argument as failed attempts in `structure.md`. Every candle's
proportions should be stored with the signal, named shape or not. Then the
model has something to learn from and the unnamed patterns can surface on
their own.

**Not built yet.** It belongs with the storage layer, which does not exist.
Written down here so it is not discovered too late. See
`worksheets/to-collect.md`.

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
| `inside_bar.rs` | **Built** — you confirmed you trade it. |
| `star.rs` | **Built** — the only three-candle shape so far. |
| — | **Add** `belt_hold.rs` |
| — | **Add** `tweezers.rs` |

Both were confirmed on 12 Aug 2026 and are built. The star is the first
three-candle shape in the project — everything else reads one candle or two,
and the finder now looks at the newest three.

---

## What this became

**Built 12 Aug 2026.** `nsc-ta::candles` has all six, driven by a `[candles]`
section in `config/ta.toml` — ten settings, eight of them shares of a candle
and two in ATR, plus three for the star. Thirty-two tests on the detectors,
eight on the types.

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
3. ~~**Do inside bars and stars go?**~~ **ANSWERED 12 Aug 2026** — both are
   traded, and both are built.
4. **The patterns with no names yet.** They come out of Phase 4, and what they
   need is every candle's measurements stored, not just the matched ones.
4. **The tweezer tolerance.** 0.05 of a normal candle is the placeholder. Two
   candles never share a high to the tick, and how close counts as the same
   price is a judgement nobody has made yet.
