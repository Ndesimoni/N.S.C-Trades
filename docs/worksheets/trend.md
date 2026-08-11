# Worksheet — Trend

The trend is already going. You wait for it to pull back, and you join it.

The most forgiving of the three, because you are going the same way as
everything else on the chart. Two things decide whether it works: how you tell
a pullback from a trend that has actually ended, and how deep a pullback you
are willing to wait for.

Answer in your own words. `config/strategies/trend.toml` is the translation of
these answers — do not skip ahead to it.

*(Nothing captured yet. Fill this in when you take me through this one.)*

---

## 1. Direction — which way, and how do you know?

Get this vague and the bot sends you a buy and a sell on the same instrument
on the same afternoon. You will stop trusting it by week two.

- Which timeframe tells you the direction? The one you check first when you
  open a chart.
- **What exactly makes it bullish?** This is the hardest one, because your eye
  does it in half a second. Try to catch what your eye is doing. Some shapes a
  real answer takes:
  - *"Higher highs and higher lows on the daily — the last high broke above
    the previous one and the pullback held above the last low."*
  - *"Price above the 50 EMA and the EMA pointing up."*
  - *"Price in the top half of the weekly range."*
  - *"The last swing high broke and price came back to retest it."*

  Yours is probably a mix. Write the mix.
- How far past an old high does price have to push before you accept the
  break? A one-pip poke should not flip your bias.
- How many higher highs and higher lows before you call it a trend? Two? Three?
- Do you use a moving average at all? Which one, and what does it have to be
  doing?
- **When direction is unclear** — chopping sideways, no clean highs and lows —
  do you skip the instrument, or drop to a smaller chart and trade the smaller
  trend?

---

## 2. Place — how deep a pullback do you want?

- What should price pull back **into**? Fibonacci zone, an old level, the
  rising trendline, the moving average, a broken ceiling that should now be a
  floor?
- How many of those need to line up at the same price?
- **How shallow is too shallow?** A pullback that barely moves has not reset
  anything.
- **How deep is too deep?** Past what point do you stop calling it a pullback?
- Which Fibonacci levels do you actually trade, if any?
- Does it matter *how* price pulled back? A slow drifting pullback against one
  violent candle straight back into the level — same trade to you, or not?
- **How close is close enough** to the level? Describe it as a fraction of a
  normal candle's size if you can.

---

## 3. Trigger — what actually makes you click buy?

- Which candlesticks do you act on — engulfing, pin bar, inside bar break?
- Be exact on engulfing: measured on the **body** only, or the whole candle
  including wicks? These give noticeably different results.
- Which chart shapes mean a pullback is finishing — a flag, a small triangle?
- Is a candle enough on its own, or does something have to break first — the
  high of the pullback, a small line drawn over it?
- Do you enter when the candle closes, or wait for price to come back to a
  price you choose?
- What makes a trigger too late — how many candles after price reached your
  zone do you stop caring?

---

## 4. Stop — where does it go, and why there?

- The **rule**, not a number. "Just past the swing the pullback made, with a
  bit of room" is a rule. "30 pips" is not.
- How much room, and why that much?
- How wide is too wide? Skip, or trade smaller?

---

## 5. Target — where do you get out?

- Fixed multiple of risk, the next level in the way, a Fibonacci extension,
  the previous high, or do you trail it?
- Worst risk-to-reward you will accept. Same as your other strategies or
  different?
- Do you take part off early? Where?
- When does the stop go to break-even? Be honest about whether it helps.

---

## 6. Skip — what makes you pass on a setup that ticks every box?

- **How far into a trend will you still join it?** Late in a move, the
  pullback you are buying is the start of the reversal. Is there a leg count
  where you stop?
- Do you skip when a big higher-timeframe level sits just above your entry?
- What if the pullback has taken longer than the push that caused it — is that
  still a pullback to you?
- Times of day and days of the week you avoid.
- News you stay out for, and for how long afterwards.
- Anything about how it "feels". Write it down even if it sounds impossible to
  measure. This is usually where the real edge is, and it almost never gets
  written down.

---

## Once this is filled in

1. Translate it into `config/strategies/trend.toml`.
2. Backtest it. **Expect it to do worse than you do.** The gap is the rules
   you have not written down yet.
3. Every signal you disagree with means a rule is missing. Add it here first,
   then to the settings file.
