# Screenshots still to collect

A running list of the examples still needed to pin down rules that are
currently vague.

**Do not go hunting for these.** Save them as you come across them in normal
trading. A screenshot taken in the moment, with a sentence about why, is worth
more than an hour spent scrolling back looking for one.

Put them in `~/Desktop/training-image-data/` and say which item it answers.

---

## The one thing that makes a screenshot useful

**Say what you decided and why — before I look at it.**

A chart on its own only shows what happened. What is missing from the code is
what *you* thought. Two traders look at the same chart and take opposite
trades; the chart cannot tell us which of them you are.

So: "this is the one I took, because X" or "this looked the same but I passed,
because Y".

**The pairs are worth the most.** One you took and one you passed on that
looked similar. The difference between them is almost always the rule that is
missing.

---

## For levels

*(from `levels.md`, captured 11 Aug 2026)*

**1. An exception to the weekly rule.**
Your rule is: price at a weekly level, do not trade — not even when it breaks.
But you said there are times you would take the weekly or daily breakout.

Next time you take one, screenshot it and say what made it different.

*Answers:* when the weekly veto does not apply.

**2. A level you decided not to draw.**
One that looked similar to a level you did draw, but you left it off.

*Answers:* what makes a level worth having at all. Right now the code will
find every level that has enough touches, which will be more than you would
draw.

**3. ~~A level touched four or more times~~ — ANSWERED 11 Aug 2026.**
More touches makes a level stronger, not worn out. Settings changed to match.
See `levels.md`.

---

## For the reversal strategy

*(from `reversal.md` — three questions still blocking)*

**4. A reversal where you bought the second low.**
And one where you waited for the neckline to break. Or if you only ever do
one, say which.

*Answers:* the biggest open question in `reversal.toml`. Buying the second low
gets a better price and is wrong more often. Waiting is right more often and
pays less.

**5. A double bottom where the second low was a bit lower than the first.**
And whether you still took it.

*Answers:* how equal the two lows have to be.

**6. A reversal you took with no trendline fan** — just the level holding.
Or confirmation that you never do that.

*Answers:* whether the exhaustion evidence is required, or just nice to have.
This is the line between a disciplined reversal strategy and buying dips.

---

## For the candlestick patterns

*(from `candles.md`, captured 12 Aug 2026)*

**7. A pin bar you took, and one you passed on.**
Two that looked similar. Same for an engulfing candle if you have a pair.

*Answers:* the measurements. The code is currently using textbook numbers —
tail twice the body, doji body under 5% of the range — because you said to use
standard ones for now. They are somebody else's numbers, and only a pair like
this replaces them with yours.

**8. The candlestick patterns you have not named.**
You said you trade many more shapes than you can name, and that training on
your labelled trades should pick them up.

Nothing to screenshot for this one. What it needs is a note on the signal when
you press 👍 or 👎 — "took it because of the way that candle closed" is enough.
The shape itself comes out of the measurements later.

*Answers:* which shapes matter beyond the eight that are built. It also
requires storing EVERY candle's proportions rather than only the ones that
matched a named shape — see `candles.md`.

---

## For the breakout and trend strategies

Nothing captured yet. Those worksheets are still empty, so there is nothing
specific to ask for.

When we get to them, the first useful screenshot for each is the same: **one
you took, and one that looked similar that you passed on.**

---

## Generally worth saving, any time

**A setup that ticked every box and you still passed.**
Say what stopped you, even if it sounds unmeasurable — "it felt heavy",
"I did not like the wick".

This is the SKIP layer, and it is where the real edge usually lives. It is
also the part that almost never gets written down, because it feels like
instinct rather than a rule.

Write it down badly rather than not at all. Vague notes can be sharpened
later. Trades you cannot remember cannot.

---

## Where the answers go

| These screenshots | End up in |
|---|---|
| Levels 1–3 | `levels.md`, then `nsc-ta::levels` and the veto layer |
| Reversal 4–6 | `reversal.md`, then `config/strategies/reversal.toml` |
| Candles 7 | `candles.md`, then `[candles]` in `config/ta.toml` |
| Candles 8 | `candles.md`, then the Phase 4 model in `research/` |
| Breakout, trend | Their own worksheets |
| The SKIP examples | The `[veto]` section of whichever strategy they belong to |
