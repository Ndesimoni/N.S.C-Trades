# Writing down your strategy

This is the **generic** worksheet — the six questions every strategy has to
answer. It explains what each layer is for.

**For an actual strategy, use the one written for it:**

| Worksheet | Becomes | The trade |
|-----------|---------|-----------|
| [worksheets/reversal.md](worksheets/reversal.md) | `config/strategies/reversal.toml` | Catching a turn once the trend runs out of strength |
| [worksheets/breakout.md](worksheets/breakout.md) | `config/strategies/breakout.toml` | Price escapes a range and you go with it |
| [worksheets/trend.md](worksheets/trend.md) | `config/strategies/trend.toml` | The trend is running; you buy the pullback |

Answer them in your own words first. The config file is the translation of
your answers.

Do not skip to the config file. Rules written straight into settings look
precise and usually mean nothing.

Do one setup at a time. A second setup gets its own file. It does not get
bolted onto the first.

---

## 1. Direction — why am I looking at this pair at all?

- Which timeframe tells you the direction?
- What exactly makes it bullish? Higher highs and higher lows? A moving
  average? Where price sits in the week's range?
- Do you ever trade against that direction? When?
- If the direction is unclear, do you skip it or look at a smaller timeframe?

> If you cannot answer this in one sentence, the bot will send you a buy and
> a sell on the same pair on the same day, and you will stop trusting it.

---

## 2. Place — where does price have to be before I care?

- Which do you need: a level, a Fibonacci zone, a trendline? How many at once?
- **How close is close enough?** Your eye judges this instantly. Try to
  describe it as a fraction of a normal candle's size — that becomes the
  setting.
- Does a level get stronger or weaker each time price tests it? After how
  many touches is it finished?
- Does it matter *how* price got to the level? Drifting slowly is not the same
  as one violent candle.

> Most traders have never put that last one into words, and it is usually a
> real filter.

---

## 3. Trigger — what actually makes me click buy?

- Which candle patterns do you act on? Be exact — engulfing measured on the
  body, or the whole candle including wicks?
- Is a candle enough on its own, or do you need something to break first?
- Do you enter when the candle closes, or wait for price to come back?
- What makes a trigger too late to bother with?

> This decides your entry price, which decides your risk-to-reward on every
> single trade. Being vague here is expensive.

---

## 4. Stop — where does it go, and why there?

- Describe the **rule**, not a number. "Just past the swing that made the
  level, with a bit of room" is a rule. "30 pips" is not.
- How much room, and why that much?
- How wide is too wide? Do you skip the trade or trade smaller?

---

## 5. Target — where do I get out?

- A fixed multiple of your risk, the next level in the way, or a Fibonacci
  extension?
- What is the worst risk-to-reward you will accept?
- Do you take part of the trade off early? Where?
- When do you move your stop to break-even? Be honest about whether this
  actually helps you.

---

## 6. Skip — what makes me pass on a setup that ticks every box?

- Times of day and days of the week you avoid
- News you stay out for, and for how long afterwards
- Pairs you will not take when you already have something else open
- Anything about how the chart "feels" — write it down even if it sounds
  impossible to measure. This is often the most valuable thing on the page.

> This section is where your real edge probably lives, and it is almost never
> written down. Expect it to grow more than any other part.

---

## Once you've filled this in

1. Turn it into `config/strategy.toml`.
2. Backtest it. Expect it to do worse than you do. The gap is the rules you
   have not written down yet — closing that gap is the actual work.
3. Every signal you disagree with means a rule is missing. Add it here first,
   then to the settings file.
