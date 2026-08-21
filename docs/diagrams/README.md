# Diagrams

Pictures drawn to settle a question.

---

## [Push then pin, in the wild →](https://claude.ai/code/artifact/f9b8ed11-659a-432a-84f2-b363f1a46fd4)

`push-then-pin-found.html` · **open — measured once, and it did not continue**

**Argues:** the settled rule hunted across all five pairs in `config/pairs` and
every timeframe from 30 minutes up. **82 found** in 6,725 finished candles — 45
`nsc-bull`, 37 `nsc-bear` — on every pair and every timeframe.

The rule, as he settled it on 21 August 2026: **candle 1** mostly body (0.60 of
its own height) **and at least the size of a normal candle**; **candle 2** a pin
with a tail at least twice its body pointing *against* the push, nose or no
nose, body big or small.

Real candles: 5 pairs x 5 timeframes from Interactive Brokers, read 21 August
2026. Shapes named by a decimal-exact mirror of `naming.rs`, **checked against
the `read` binary's own output on all 6,725 candles — every name agrees**.

**The uncomfortable part.** Each one followed for ten candles, entering at the
open after the pin closes. It reached +1 normal candle before -1 in **29 of 75 —
38%, where a coin flip is 50%**. It ran further against than in favour (1.65
against, 1.17 for) and the median position was negative at 1, 3, 5 and 10
candles. Every timeframe and four of five pairs point the same way. **The bigger
the push, the worse it did** — 2.0x-plus pushes came back 21% and a median of
-1.47, which is what exhaustion looks like rather than a pause.

**Do not over-read it.** 80 trades is under the 100 this project's backtest rule
sets as the floor; no spread or slippage was taken off; the +1/-1 stop and
target are Claude's invention, not his; and **no level was involved** — all 80
are shapes in open space, which is not how he trades them. The gold pair he
circled is not in the numbers, because it closed the day before and has no ten
candles after it.

**Still open:** the same 82 measured against his levels, once gold's black zone
is in `config/pairs/XAUUSD.toml`. That is the test that matters.

---

## [Push then pin →](https://claude.ai/code/artifact/3aa76e11-85f5-40cd-9116-5513dc9a2488)

`push-then-pin.html` · **open — the pattern he trades, not yet built**

**Argues:** a two-candle pattern he actually trades, drawn as **twenty pairs** —
every combination of the setup. A momentum candle showing one side winning, then
a pin bar whose tail points **against** the push and runs at least twice its own
body. The tail is a failed pullback, which makes this a **continuation** pattern.
Tail pointing the same way as the push is a different animal and he does not
trade it; those two are kept on the page marked "not this".

The two halves vary independently — four momentum candles, six pin bars — so each
is walked through in full with the other held steady.

Made-up candles for the shapes, drawn from the thresholds in
`config/candles.toml` and named by a mirror of the running order in `naming.rs`.
The one real example is gold, 19–20 August 2026, from Interactive Brokers.

**He ruled out the indecision shapes.** Anything the code names a `spinning top`
or a `high wave` — a small body with real wick on BOTH sides — is not his pattern,
however long one of the wicks is. The pin has a tail one way and little or
nothing the other.

**Found while drawing it:** with no top, the body and the tail are the whole
candle, so a 2x tail caps the body at exactly **1/3** — arithmetic, not a
setting. `body.small` is 0.33, so the largest body his rule allows is turned away
by one thousandth. The code also splits his one pin bar into **two names** by
body size — dragonfly or gravestone doji under 0.05, long lower or upper wick
above it — so a detector has to accept all four.

**Settled since:** the pins the code turns away at the 1/3 body boundary are his
too, and the family is named `nsc-bull` / `nsc-bear` — his own prefix, so his
patterns never get mistaken for the textbook twelve.

**Still open:** nothing joins the two halves into one pattern; the detector needs
the tail's direction relative to the push; and `body.small` has to move to 0.3334
or the pattern needs its own threshold in `config/patterns.toml`, since that
setting is shared by every candle everywhere. See
[Push then pin, in the wild](#) for what these actually did next.

---

## [One body, one wick →](https://claude.ai/code/artifact/187a99a3-1e54-46e9-9b0d-e3eb9cc6e39d)

`two-gold-candles.html` · **open — the level these printed at is not in config**

**Argues:** the two days he circled on gold did opposite things with the same
kind of range. The 19th was **86.8% body** — a day that decided. The 20th was
**72.0% lower wick** and closed 4.89 points from its open — a day that went 65
points down and gave every one of them back. Drawn on one axis, the second is
less than half the height of the first, which is the part the eye gets wrong.

Real candles: 2 XAU/USD daily candles from Interactive Brokers, 19 and 20 August
2026, named by `nsc-ta` against `config/candles.toml`. "Normal" is the average
true range of the 14 days before each candle. Reach figures reproduce the
`--bin read` output exactly (2.08x and 0.87x).

**Still open:** the black zone these printed into is not in
`config/pairs/XAUUSD.toml`, so the bot cannot see it. A shape is only worth
anything once you know the level it printed at — that number has to be read off
the axis and added before any of this becomes a signal.

---

## [Where ADX says there is a trend →](https://claude.ai/code/artifact/cbc95367-6655-4e0e-9b75-aadd8b8b0769)

`adx-on-gold.html` · **open**

**Argues:** the textbook ADX threshold of 25 does not fit gold. On 5,000 hours
it calls 57% of all time a trend, and the median reading is 27 — the threshold
sits at the middle of normal.

Real candles: 501 XAU/USD 1-hour from Twelve Data, 18 May to 8 June 2026, with
the percentages measured across all 5,000 held. Tap a threshold and watch the
shading change.

**Still open:** which number, and whether ADX earns its place beside
Choppiness at all.

---

## [A moving average is not a level on gold →](https://claude.ai/code/artifact/abeaddcf-1e2f-40df-8934-14283b7bdb23)

`ma-on-gold.html` · **settled — do not use EMAs as levels on the 1-hour**

**Argues:** on XAU/USD 1-hour, an EMA does neither of the things it is meant to.
It crosses price 4 to 18 times a week, so which side price is on is not a trend.
And the textbook pullback — rising EMA, candle touches it, closes above — came
back **under 50% on all five periods tested** (10, 20, 50, 100, 150), with the 20
at 41% on 360 touches and the 50 at 38% on 167.

Real candles: 501 XAU/USD 1-hour from Twelve Data, 18 May to 8 June 2026, with
every figure measured across all 5,000 held. Distances in ATR.

**Still open:** the MA's *slope* as a direction gate, which this did not test,
and whether any of it changes on the daily.

---

## [The range indicator is really a coil indicator →](https://claude.ai/code/artifact/de8ffd1f-2599-471e-a49e-c103d0a22a76)

`ranges-on-gold.html` · **settled — do not trade inside what it calls a range**

**Argues:** Choppiness marks gold's tight patches correctly, but what follows is
the opposite of the label. On the 4-hour, price stayed inside the box **2% of
the time** and travelled **106% of its width**. On the 1-hour, 7% and 104%. And
the readings it calls *trending* are the calmer ones — 18% and 23% stayed put.

A high reading is compression before a break, not a range to sell the top of. It
never says which way.

Real candles: XAU/USD 1-hour and 4-hour from Twelve Data, both timeframes on the
page. **Weekends and holidays removed** — 1,412 of the 5,000 hourly candles had
a range under 0.02% of price, and the first run of this reported price
travelling 5,119% of its box before that was caught.

**Still open:** whether a high reading is worth acting on at all, given it gives
no direction.

---

## [What a doji is, as four numbers →](https://claude.ai/code/artifact/d42faa62-e237-414f-858b-22d5b836c33e)

`doji-model.html` · **open — the model is agreed, the threshold is not**

**Argues:** a candle should be measured before it is named. Four numbers come
out — `body`, `upper`, `lower` as shares of the whole candle, and `reach` in
ATR — and every one is a fact. The name comes after, from a threshold in
`config/`.

Move `doji_body` and nothing recompiles: 0.05 gives 203 dojis, 0.10 gives 394,
0.20 gives 813, out of 3,546 live gold hourly candles.

Real candles: six actual XAU/USD 1-hour candles drawn to their own proportions,
including the borderline one at 0.127 that the threshold argues about.

**Two things it already shows:** a doji is not rare — one hourly candle in nine
at 0.10 — and two in five are too small to mean anything, which is what `reach`
is for.

**Still open:** the threshold, and the rest of the candle names.

---

## [The three candles the tests are built on →](https://claude.ai/code/artifact/c655b344-a136-42bf-902f-49a8a07e5c11)

`test-candles.html` · **reference**

Every test in `crates/nsc-ta/src/candle/tests.rs` uses a candle that actually
printed. This is what they look like, each with the hours either side.

- **The doji**, 15 May 2026 16:00 — travelled $23.16, finished 31 cents from
  where it started
- **The marubozu**, 21 March 2026 00:00 — opened at its high, closed at its
  low, $64 down, no wick at either end. The only candle in 5,000 measuring
  exactly 1.0000
- **The flat one**, 19 April 2025 07:00 — Easter Saturday, open high low and
  close all 3326.27, and no shape at all

Real candles: XAU/USD 1-hour and 4-hour from Twelve Data.

---

## [The five shapes, found in three years of gold →](https://claude.ai/code/artifact/21c85fd3-6d73-435e-a12a-9757f4fb70e3)

`shapes-gallery.html` · **reference**

Pin bar, doji, belt-hold, engulfing and tweezers — the five this project decided
a trader actually reads, each with the clearest real example and the candles
either side.

**How often each turned up in 4,165 candles:** pin bar up 348, pin bar down 315,
doji 241, engulfing up 231, engulfing down 194, tweezer bottom 167, tweezer top
165, belt-hold up 152, belt-hold down 117.

**A shape every twelve candles is not a signal, it is a description.** What makes
one worth a message is the level it printed at, and that belongs to
`nsc-strategy`.

Real candles: XAU/USD 4-hour from Twelve Data, 23 Oct 2023 to 14 Aug 2026,
weekends removed, found with the thresholds already in `config/ta.toml` and ATR
worked out as it goes.

---

## [Twenty-two names, fewer shapes →](https://claude.ai/code/artifact/e9b99458-5f1b-42d3-a595-b14f6797bdac)

`candle-taxonomy.html` · **settled — do not build a detector per name**

**Argues:** the full single-candle list has 22 names on it, and the four numbers
cannot tell all of them apart, because several are one shape wearing two names.

| These two | Told apart by |
|---|---|
| Hammer / Hanging Man | a downtrend before it, or an uptrend |
| Shooting Star / Inverted Hammer | an uptrend before it, or a downtrend |
| Paper Umbrella | nothing — it *is* the hammer shape |
| Takuri | a hammer with a longer tail. Same candle |
| Long Bullish / Bullish Belt Hold | the same candle in the example found |

Build one detector per name and two of them fire on the same candle, so a
backtest counts one setup twice.

**What the counts said:** spinning tops 408 in three years, one candle in ten. A
true marubozu is rare — four bullish and three bearish in 4,165 — and almost
every "marubozu" is really an opening or closing one. High-wave found exactly
one, left as one rather than loosening the rule until a shape appeared.

Real candles: XAU/USD 4-hour from Twelve Data, 23 Oct 2023 to 14 Aug 2026.
