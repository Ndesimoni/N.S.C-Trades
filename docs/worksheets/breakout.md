# Worksheet — Breakout

Price has been stuck. It gets out. You go with it.

The whole difficulty is one thing: **most breaks fail.** Price pokes through a
level, everyone piles in, and it comes straight back. Nearly every question
below is really the same question — how do you tell a real break from a
fakeout?

Answer in your own words. `config/strategies/breakout.toml` is the translation
of these answers — do not skip ahead to it.

*(Nothing captured yet. Fill this in when you take me through this one.)*

---

## 1. Direction — which breaks are you allowed to take?

- Must the break go the same way as the bigger trend, or will you take a
  break in either direction?
- If the higher timeframe is going sideways — is that a reason to skip, or is
  sideways exactly when breakouts work best?
- Which timeframe decides that?

---

## 2. Place — what exactly is being broken?

"A breakout" is not a thing on its own. Something specific has to break.

- What do you trade the break of? The top of a range, the last swing high, a
  trendline, a pattern edge, a round number, the previous day's high, the
  session high?
- How many times must the level have held before it counts? A level nobody
  defended is not a breakout — it is just price moving.
- **The build-up.** Does there need to be a quiet, tight stretch before the
  break? How long, and how tight? Describe tightness as a multiple of a
  normal candle if you can.
- Is a break out of a wide, noisy range the same trade to you as a break out
  of a tight coil? If not, say what the difference feels like.

---

## 3. Trigger — what makes it a real break?

This section decides how much of your money goes to fakeouts.

- **How far past the level counts as broken?** A one-pip poke is not a break.
  As a fraction of a normal candle.
- Does the candle have to **close** past the level, or is a wick through
  enough? How many closes?
- **The big one: do you enter on the break, or wait for the retest?**
  - Enter immediately — best price, catches every fakeout.
  - Wait for the retest — much better win rate, misses the ones that never
    come back.
  - Or both, depending on something. If it depends, on what?
- If you wait for the retest: how long do you wait before giving up? How
  close does price have to come back? Does the retest need its own candle
  confirmation, or is touching and holding enough?
- Which chart patterns do you trade breaks of — triangle, flag, head and
  shoulders neckline, double top/bottom neckline?
- Which candlesticks confirm it for you?
- When is a break too late to take?

---

## 4. Stop — where does it go, and why there?

- Back inside the level? The other side of the range? Past the breaking
  candle's wick?
- How much room past it?
- If price closes back inside the level, is the trade dead even if your stop
  was not hit? Do you want out at that point?
- How wide is too wide — skip, or trade smaller?

---

## 5. Target — where do you get out?

- Do you project the height of the range from the break point? Take the next
  level? A fixed multiple of risk? A Fibonacci extension?
- What is the worst risk-to-reward you will accept on a breakout? Is it the
  same number as your other strategies, or do you demand more here because
  more of them fail?
- Partials, break-even — same questions as always.

---

## 6. Skip — what makes you pass on a setup that ticks every box?

- A break caused by a news announcement — do you take it or sit it out?
- Do you skip a break when there is a big higher-timeframe level sitting just
  above it? How much room does the trade need?
- Breaks late in a session, with no time left to run — skip?
- How many times will you try the same level before you stop believing in it?
- Days and times you avoid entirely.
- Anything about how the chart "feels" before a break that you cannot yet put
  into words. Write it down badly rather than not at all.

---

## Once this is filled in

1. Translate it into `config/strategies/breakout.toml`.
2. Backtest it. **Expect it to do worse than you do.** The gap is the rules
   you have not written down yet.
3. Every signal you disagree with means a rule is missing. Add it here first,
   then to the settings file.
