# Diagrams

Pictures drawn to settle a question.

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
