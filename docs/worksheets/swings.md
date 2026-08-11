# Worksheet — Swings

How you pick swing highs and lows. Captured 12 Aug 2026, from talking through
the `swing-lookback` diagram.

This describes **what you see on a chart**, so it becomes `nsc-ta::swings`.
Nothing here decides a trade.

Swings matter more than anything else in this project. Levels, trendlines,
Fibonacci anchors and trend direction are all built from them.

---

## What you told me

### The timeframes that matter

Daily and 4-hour. Those are the ones you read swings on.

### Always use the wick

Settled. The swing sits at the high of the candle for a peak and the low for a
trough, wick included, however long it is.

That is what the code already does, so nothing changes here.

### You do not count candles

**This is the rule that matters, and it replaces the current design.**

Your words: there is no exact number of candles either side. You go by how the
pullback forms and how deep it is.

So the question "did this candle beat the three before it and the three after
it" is not the question you ask. It was a stand-in, and it is the wrong one.

### The test is the depth of the pullback

A peak counts once price has pulled back by **at least half — or near half —
of the run that made it.**

Not half of a fixed distance. Half of *that particular move*. A 300-point rally
needs a pullback of roughly 150 before the top of it is a swing. A 60-point
rally needs about 30.

This is why the candle count never worked: the same rule holds whether the
turn took four candles or forty.

### A shallow pullback still counts if price runs again

Added 12 Aug 2026, and it matters more than the rule above.

If the pullback stops **near** half rather than reaching it, and then price
turns and runs again, that is still a real swing. So a peak has two ways to
prove itself:

| Route | What happens | When it is confirmed |
|---|---|---|
| Depth | price gives back half the run | at the candle that reaches half |
| Resumption | price gives back near half, then takes the peak out | at the candle that clears the peak |

On the second route both points are confirmed at once: the peak is a swing
high, and the bottom of the shallow pullback is a swing low.

**Why this is not a detail.** Trend strength shows up as pullback depth — the
stronger the move, the less it gives back. A rule that only confirmed on depth
would read structure fine in chop and go blind in a clean trend, which is
exactly the market you want to be reading.

### What all of this is for

Your question, and the answer is yes: **trend**.

Higher highs and higher lows is an uptrend. Lower highs and lower lows is a
downtrend. Neither sentence can be said without swing points to count, which is
why this module sits under everything else.

What counts as a higher high has its own rule — taking the old high out is not
enough on its own. That lives in `structure.md`.

It is also the same measurement Fibonacci uses: the run is the leg and the
pullback is the retracement. And the numbers turn out to be the same numbers.

**The shallow threshold is 0.382** — settled 12 Aug 2026. That is the level you
watch when a trend is strong, because a strong move barely pauses before
running on. It is the same situation the shallow route describes, so it is one
belief written once rather than two numbers that could drift apart.

So the two depths are:

    0.382    counts if price then takes the peak out
    0.5      counts on its own, no resumption needed

---

## What that means for the code

### The finder gets rewritten

Today it slides a window along and asks whether the candle in the middle beats
its neighbours. That has to go, because it answers the wrong question.

What replaces it follows the rule directly:

1. Keep track of the highest price since the last confirmed low. That is the
   running peak, and the distance from that low up to it is **the run**.
2. Watch how far price falls back from the peak.
3. The moment that fall reaches **half the run**, the peak becomes a
   confirmed swing high.
4. Then do the same upside-down, looking for the low.

### Three things that fall out of it, all of them good

**Swings alternate.** High, low, high, low, the way you would draw them.
Today's finder can call the same candle both a high and a low — an outside bar
that beats everything around it in both directions. Under the new rule that
cannot happen, because after a high it is looking for a low.

**One setting works on every timeframe.** Half the run is a ratio, so it means
the same thing on the 4-hour and the daily, on gold and on EURUSD. The
per-timeframe `lookback` numbers we discussed an hour ago are not needed —
they were only ever trying to approximate this.

**Confirmation gets honest instead of fixed.** Today a swing is knowable
exactly 3 candles later, always. Under the new rule it is knowable at the
candle where the pullback reached half the run — sometimes two candles,
sometimes thirty. That is the true answer, and it is still safe: the moment is
measured from candles that have already closed, so nothing is used before it
existed.

Worth knowing the cost: on the daily, a slow shallow pullback can leave a
swing unconfirmed for weeks. It is on the chart and plain to see, and the bot
still cannot use it, because by your own rule it has not proved itself yet.

### Ties stop being a problem

Two candles sharing the same high currently produce no swing at all, on the
grounds that missing a level beats inventing one. Under the new rule the
running peak simply keeps the first of them, so the swing exists.

That also settles the clash with tweezer tops in `candles.md`, where the
pattern needs exactly the shape swing detection was throwing away.

---

## The floor: a run is measured against the run before it

Your answer, 12 Aug 2026. **A run that does not reach at least half — perhaps
three quarters — of the previous run is not considered at all.**

So the floor is relative, like everything else here. No pip number, no ATR
number: a move is only a move next to the move that came before it. A 200-point
leg followed by a 40-point wobble means the wobble is not structure, and it does
not matter what instrument or timeframe that happens on.

That is a better answer than the ATR floor I suggested, because it adapts. A
quiet week and a violent week both get judged on their own terms.

### The problem it has, and it is worth fixing before it is built

Measured only against the run immediately before, the rule can ratchet
downwards. Each run passes on its own and the chain still shrinks to nothing:

    200  →  120  →  72  →  43  →  26  →  15

Every one of those is 60% of the one before it, so every one passes. Six legs
later the "runs" are 7% of where the sequence started, and the chart is back to
being full of noise — the exact thing this rule was added to stop.

**The fix is to compare against the biggest recent run, not the last one.**
Then the shrinking stops at the third leg: 72 is not half of the 200 still in
memory, so it is not a run, and the ratchet never gets going.

A rejected leg is not a leg, so it never enters the memory either. Structure
simply goes quiet until a real move comes back — which is the honest answer for
a market that has gone quiet.

### How much of the previous run

**Between 50% and 75%.** Settled 12 Aug 2026.

So it is one setting with a range you are happy inside, rather than a single
figure. It starts at 0.5 — the loosest end, which throws the least away — and
0.75 is the tightest you would want it. Tightening it means fewer, bigger
swings and a cleaner chart, at the cost of missing smaller structure.

    min_run_fraction = 0.5      # your range: 0.5 to 0.75

### Which run it gets compared against

Not answered, so this is my call and it is easy to change.

**The biggest of the last five legs.** Five is enough to stop the ratchet — by
the sixth shrinking leg the original big run is still in memory, so the wobble
is measured against 200 rather than against 26 and fails.

    run_memory_legs = 5

The alternative was to compare against everything inside the 500-candle window
the levels use. That is rejected: one enormous move a year ago would then
suppress every piece of structure since, and the bot would go quiet for months
with nothing to show for it.

---

## Still open

1. ~~**How near is near half?**~~ **ANSWERED** — 0.382, the same number as the
   strong-trend Fibonacci level. See `fibonacci.md`.
2. ~~**The floor for a flat market**~~ **ANSWERED** — a run must reach 50% to
   75% of the previous one.
3. **Major and minor swings.** Still one flat list in the code. If you separate
   the turns that structure a move from the wiggles inside a pullback, that is
   two thresholds running side by side — say a 50% pullback for the minor ones
   and a deeper one for the major.

Nothing on that list blocks the rewrite. Number 3 changes what the finder
returns rather than how it decides, so it can be added afterwards without
undoing anything.

---

## What this becomes

`nsc-ta::swings`, driven by `[swings]` in `config/ta.toml`.

The settings change shape completely:

| Was | Becomes |
|---|---|
| `lookback = 3` | gone — no candle counting |
| `min_atr_multiple = 0.5` | gone — replaced by the pullback test |
| — | `confirm_retracement = 0.5` — the depth that proves a peak on its own |
| — | `shallow_retracement = 0.382` — counts once price takes the peak out |
| — | `min_run_fraction = 0.5` — how big a run must be next to recent ones |
| — | `run_memory_legs = 5` — how far back "recent" reaches |
| `require_confirmed = true` | stays, and matters more than ever |

Everything built on swings inherits the change: levels move, and every test
that fixes a swing at a particular candle gets rewritten with it.
