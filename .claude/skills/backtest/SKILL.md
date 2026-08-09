---
name: backtest
description: Use when running a backtest or trying different settings, reading the results, or deciding whether a change is actually an improvement. Also use when backtest results look too good.
---

# Running and reading a backtest

## Before believing any result

1. Any missing candles in the period? Run the gap check. Holed data produces
   confident, wrong numbers.
2. Are the future-data checks switched on? A run that touched future data must
   produce no number at all.
3. Enough trades? Under about 100 it is a story with a decimal point. Show the
   trade count next to every number.
4. Were news blackouts applied using the old calendar? A backtest that trades
   through every jobs report is measuring a system you are not going to run.

## Read the average result, not the win rate

Winning 35% of the time at 3R comfortably beats winning 70% at 0.4R. Win rate
is the number that will tempt you to ruin the first one.

The most useful diagnostics are how far price ran each way:

- **In your favour** — targets sitting past where price actually turns
- **Against you before it worked** — stops getting clipped just before the move

Those two answer "should I move my stop or my target?" without guessing.

## Reading a settings sweep: look for a patch, not a peak

What comes out is a landscape, not a leaderboard.

Look for a **broad patch where nearby settings all do reasonably well**. One
combination that beats its neighbours by a mile has found a quirk of this
particular history. Adopting it feels like being thorough. It is overfitting.

Test the fewest settings that answer your question. A grid across six settings
always contains something amazing, purely because it is enormous.

## When the results look great

Suspect the machinery before celebrating. In order of likelihood:

1. Future data leaking in somewhere the checks do not reach
2. Ambiguous trades guessed in your favour — check how many came back
   `ambiguous`. A high count means your small candles are too coarse to
   resolve these trades at all
3. Spread and slippage not applied
4. You tuned on the same data you are now measuring
5. Too few trades

## Compare like with like

Every run saves its full settings and the code version. A result means nothing
without knowing which version of the chart-reading code produced it.

Comparing across a change to swing detection is comparing two different
systems — and doing it by accident is how a "promising" setting gets adopted
on the strength of a bug.

## The breakdowns worth more than the headline number

`nsc-backtest::report` splits by pair, session, day of week, confidence band,
and which layer rejected the setups that never fired.

- **Rejecting layer** answers "why did nothing fire this week?" without
  rerunning anything.
- **Confidence band** is the first honest test of your scoring. If
  high-confidence signals do not beat low-confidence ones, the scores measure
  nothing.
- **By pair** catches a strategy that is really being carried by one pair.

## Keep some data back

Hold back the most recent 6–12 months and do not look at it while tuning.

Once you have looked, it is contaminated forever. There is no way to un-see
it, and every decision after that is affected by it.
