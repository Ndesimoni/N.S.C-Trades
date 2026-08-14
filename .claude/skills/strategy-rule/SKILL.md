---
name: strategy-rule
description: Use when changing trading rules — editing config/strategy.toml, changing the direction/place/trigger/stop/target/skip layers in nsc-strategy, adjusting confidence scores, or when the bot sends signals the trader disagrees with.
---

# Changing a trading rule

## The six layers

```
1. DIRECTION   why am I looking at this pair?    ── must pass
2. PLACE       where must price be?              ── must pass
3. TRIGGER     what makes me click buy?          ── must pass
4. STOP        where does the stop go?           ── works out the SL
5. TARGET      where do I get out?               ── works out the TP
6. SKIP        anything that cancels it anyway?  ── kills the setup

CONFLUENCE     how confident am I?               ── scores it
```

Find the right layer first. A rule put in the wrong layer still works, and
then becomes impossible to reason about. A location filter hidden inside the
trigger makes "why did nothing fire?" unanswerable.

## Must-pass versus scored

Direction, place and trigger must **all** pass. Only the extras get points.

Do not turn a must-pass rule into points to make the bot send more signals. A
pure points system will eventually send you a setup you would never take, and
you will have no way to work out which point value caused it.

## When the bot sends a signal you disagree with

**A rule is missing. The model is not broken.** Work in this order:

1. Write the rule in plain words first, in a worksheet beside the config. If you
   cannot say it, you cannot code it.
2. Work out which layer it belongs to. Most of them are skip rules — that
   layer is where a trader's real edge usually lives and it is almost never
   written down.
3. Add it to `config/strategy.toml`.
4. Backtest before and after. A rule that removes losers and winners in equal
   measure is costing you trades for nothing.

Resist tightening rules until the bot goes quiet. Silence is not correctness,
and a bot that fires twice a month teaches you nothing, because you never
collect the verdicts Phase 4 needs.

## Changing rules means bumping the version

`[meta] version` in `config/strategy.toml`, every time behaviour changes. It
gets saved on every signal and every backtest run.

Comparing results across an unversioned rule change means comparing two
different systems while believing you are measuring one.

## About the confidence scores

They start as guesses. That is fine and it is the point. Once you have a few
hundred judged signals, the Phase 4 analysis tells you which ones the data
actually supports.

Do not hand-tune the numbers to make recent signals look better. That is
overfitting, done by hand.

There is a sanity check available today: `nsc-backtest::report` splits results
by confidence band. If high-confidence signals do not beat low-confidence
ones, the scores are not measuring anything, and adjusting them will not fix
that.

## Adding a second setup

A **new settings file**, not more conditions bolted onto the existing one.
Fading and following have different win rates and different risk-to-reward.
Averaging them into one set of numbers hides both.

## Testing

`crates/nsc-strategy/tests/rules.rs`, using made-up snapshots — the payoff for
this crate never touching the outside world. Check that each must-pass rule
can reject on its own, and that which layer rejected gets recorded.

Check the settings file makes sense at load time. A file asking for more
confluences than it has sources switched on shows up as "the bot never sends
anything", which is a miserable thing to work out from the outside.
