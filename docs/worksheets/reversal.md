# Worksheet — Reversal

Catching a turn: buying near the bottom of a fall, or selling near the top of
a rise, once the trend has shown it is running out of strength.

This is the only one of the three that trades **against** the bigger trend, so
it has to prove more before it fires.

Answer in your own words. `config/strategies/reversal.toml` is the translation
of these answers — do not skip ahead to it. Rules written straight into
settings look precise and usually mean nothing.

---

## What you have told me so far

Captured from the XAUUSD daily chart on 10 Aug 2026. **Check it and correct
anything I have wrong** — everything below becomes a setting.

### The trendline fan tells you how strong the trend is

The market had been falling since March. Normally a downtrend means you sell.
You did not sell.

Instead you drew **two descending trendlines**, and you use which line price
is trading against as a **strength meter**:

- The **first, steeper line** held from April all the way to August. While
  price keeps getting rejected there, the downtrend is **strong** and sellers
  are in control.
- The **second, shallower line** is the warning. For price to even reach it,
  buyers had to push further than the steep line would ever have allowed. So
  reaching the second line is not a normal pullback — it is **evidence of
  buyers**. The trend is losing strength.

So trendlines are not support and resistance to you. They measure conviction.

### The floor holding is what turns "not selling" into "buying"

Weakening sellers alone is only a reason to stand aside. What made it a buy:

- A **long rising line acting as support** underneath, from the January low.
- Price came down to it and **struggled to break it**.
- While struggling, it formed **something like a double bottom** — two
  attempts at the low, neither got through.

Sellers weakening **plus** the floor holding is the combination.

### One thing I got wrong

I first guessed the **break** of the descending trendline turned you bullish.
It did not. You were already turning bullish before the break — the fan and
the double bottom did it. The break came afterwards as confirmation.

That difference matters: a bot built on my version enters at 4,350 after the
move has run. A bot built on yours is watching the floor at 4,000.

### The trade you drew

Entry 4,132.020 · Stop 4,055.913 · Target 4,298.055 — about **2.2 to 1**.
Entry was *below* the current price, so this was **wait for the pullback**,
not buy now.

---

## 1. Direction — is the trend tired enough to fade?

- Which timeframe shows the trend you are fading? *(The chart you sent was
  the Daily — is that always the one?)*
- **The fan:** how do you decide where the second line goes? From which two
  points do you draw it?
- Does price have to **close** beyond the steep line, or is touching it
  enough? A wick through and back is not the same thing.
- How many lines do you draw before the fan means something — is two the
  number, or have you gone to three?
- Once price is trading against the shallower line, how long does that
  reading stay good before it goes stale?
- Besides the fan, what else tells you a trend is running out? A new low that
  fails to hold? Each leg travelling less than the last? An indicator?
- **Would you ever take this trade on the level alone, with no exhaustion
  evidence at all?** Answer honestly — this is what separates a disciplined
  reversal strategy from just buying dips.

---

## 2. Place — what does price have to be at?

- What kinds of level count: rising trendline, horizontal support, Fibonacci,
  round number? How many at once?
- **How close is close enough?** Your eye judges this instantly. Try to
  describe it as a fraction of a normal candle's size — that becomes the
  setting.
- Does it matter *how* price arrived? Drifting down slowly is not the same as
  one violent candle. On the gold chart price ground down through July rather
  than crashing — did that matter to you?
- How old must a level be before it has proved anything? A level from last
  week has not held much.
- After how many touches is a level worn out and more likely to break?

---

## 3. Trigger — what actually makes you click buy?

- **The double bottom.** How equal do the two lows have to be? Exactly level,
  or is the second one allowed to be a bit lower? How much lower?
- How far apart in time do the two lows need to be? Too close together and it
  is one messy bottom, not two attempts.
- **The big one: do you wait for the neckline to break, or do you buy at the
  second low?** Buying the second low gets a much better price and is wrong
  more often. Waiting is right more often and pays less. Which do you
  actually do?
- Which candlesticks do you act on — engulfing, pin bar, inside bar? Is
  engulfing measured on the body only, or the whole candle with wicks?
- Do you need both the chart pattern and a candlestick, or is either enough?
- What makes a trigger too late to bother with?
- Are there other reversal shapes you use besides the double bottom? Head and
  shoulders? Something that has no name?

---

## 4. Stop — where does it go, and why there?

- Describe the **rule**, not a number. Past the lowest point of the pattern?
  Past the swing that made the level? Past the trigger candle's wick?
- How much room past it, and why that much?
- On the gold trade your stop was 4,055.913 — what was it sitting under?
- How wide is too wide? Do you skip the trade or take it smaller?

---

## 5. Target — where do you get out?

- Your gold trade was about 2.2 to 1. Is 2:1 your floor, or does it vary?
- Target 4,298 was just under the level price had failed at. Is that the
  rule — the next level in the way?
- Do you take part off early? Where?
- When do you move the stop to break-even? Be honest about whether that
  actually helps you or just turns winners into scratches.

---

## 6. Skip — what makes you pass on a setup that ticks every box?

- Times of day and days of the week you avoid.
- News you stay out for, and for how long afterwards.
- Would you take this if the fall had been fast and vertical rather than a
  grind? Is there a point where a market is falling too hard to catch?
- How many times will you try the same level before giving up on it?
- Anything about how the chart "feels". Write it down even if it sounds
  impossible to measure — this is usually the most valuable thing on the
  page, and it is almost never written down.

---

## Once this is filled in

1. Translate it into `config/strategies/reversal.toml`.
2. Backtest it. **Expect it to do worse than you do.** The gap is the rules
   you have not written down yet. Closing that gap is the actual work.
3. Every signal you disagree with means a rule is missing. Add it here first,
   then to the settings file.
