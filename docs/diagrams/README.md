# Diagrams

Pictures drawn to settle a question.

**Seven pages were removed on 29 August 2026** — four about the chart patterns
(the curvy, the doubles, the swing finder) and three about the indicators. Those
features were taken out of the project the same day, and a diagram of code that
no longer exists is worse than no diagram, because it gets believed.

---

## [Two shapes added →](https://claude.ai/code/artifact/e5db4109-29ba-4cfb-bf84-796b9711e185)

`two-added.html` · **open — the shapes, not yet the signals**

**Argues:** harami and marching joined rung 3 on 29 August 2026, and two size
floors went in with them. Real examples of all four kinds, found by
`pattern::ending_at` with the floors on, with the dashed pair showing exactly
what the size test measured.

**What the floors changed:** harami 26,788 → 16,596 (−38%), engulfing 36,560 →
~21,900 (−40%). **Marching deliberately got none** — 96% already reach a normal
candle, and a threshold that never refuses anything is worse than none.

**It says plainly what it cannot show.** Every level he has drawn is weekly or
daily and only 1-hour and 4-hour candles are on disk, so no band can be sized
without guessing at a daily candle. These are the shapes; the level half of the
setup is untested until TWS is up.

---

## [The six he does not trade →](https://claude.ai/code/artifact/5421c6ee-823c-45a7-82de-9273f454d503)

`six-untraded.html` · **open — six named patterns, none of them measured**

**Argues:** what every pattern `nsc-ta` can name but rung 3 ignores actually looks
like — harami, tweezer, piercing line, dark cloud cover, star and marching, **two
real examples of each**, found by `pattern::ending_at` with today's
`config/patterns.toml`. So what is drawn is what the code calls it, not a
textbook picture.

The two examples of each come from different pairs and different years, and the
dashed line marks the price the pattern is about — the shared low on a tweezer,
halfway into candle 1 on a piercing line.

**The abandoned baby is missing and that is a finding.** It needs a gap either
side of the middle candle, and spot forex does not gap, so the sweep found none
at all in 270,000 candles. The code can name a shape it will never meet.

**Nothing here has been measured.** There is no evidence any of the six is worth
adding to rung 3, and none that it is not.

---

## [Push then pin, on gold and the Aussie →](https://claude.ai/code/artifact/f20543a0-55a5-4e1c-b682-961b683b1d25)

`gold-and-aussie-pushes.html` · **open — the two pairs he asked about, and neither continued**

**Argues:** his setup, cut down to the two pairs he watches most. **19 found on
each.** Gold reached +1 normal candle before -1 in 47% of them with a median of
+0.08; the Aussie in 36%, median -0.02. Gold is the best of the five pairs and it
is still a coin flip.

Same real candles as the sweep below — Interactive Brokers, 21-22 August 2026,
five timeframes. Nothing here is a live read.

**It also records a count that does not reconcile.** The sweep is written down as
82 found, as 80 in its own breakdown, and as 75 followed. The rows sum to 80. It
changes no conclusion and it needs settling, because an unreconciled count is how
a real mistake hides.

**Says three times what it cannot tell you:** it is not a live read, trend is not
built, and none of these had a level under them.

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
