# The ways this kind of system goes wrong

Every one of these is silent. Nothing crashes. No error appears. That is
exactly why they are written down.

## Using data the market hadn't printed yet

Also called lookahead bias.

A swing high is not *known* to be a swing high until a few candles later —
you need to see price come back down to know that was the top. If your
backtest uses that swing high at the moment it formed, it is using knowledge
it could not have had.

Your backtest will look great. Your live results will not resemble it at all.

**The trap:** this does not cause an error. It makes your numbers *better*.

**Defence:** `nsc-backtest/src/guards.rs`, switched on in every test run.

## Missing candles nobody noticed

An hour of missing data shifts where a swing sits. That shifts where a level
sits. That changes every signal after it.

The backtest still finishes. It still prints a believable number.

**Defence:** `nsc-data/src/gaps.rs`. Run it after every download.

## Backtest and live quietly drifting apart

Caused by any code that asks "am I backtesting right now?"

**Defence:** keep `nsc-ta` and `nsc-strategy` free of outside-world code, and
make both the backtester and the live bot go through the same entry point.

## Guessing in your own favour

Sometimes price hits your stop and your target inside the same candle. On a
15-minute candle, you genuinely cannot tell which came first.

If the backtester assumes the target came first, each guess is small. Over
300 trades it decides whether the strategy looks profitable.

**Defence:** mark it `ambiguous` and leave it out of the numbers entirely.

## Picking the best result out of a big grid

Test 500 combinations of settings and one will look brilliant. It is the
luckiest one, not the best one. Next month it will be ordinary.

**Defence:** look for a *patch* of settings that all work reasonably, not the
single highest number. And test fewer settings at once — a big grid always
contains something that looks amazing.

## Wrong clock

Server not set to UTC, or your daily candle closing at a different time than
your broker's.

Your bot's levels stop matching the levels on your own chart. You stop
trusting it — and it looks like a strategy problem when it is a clock problem.

**Defence:** UTC everywhere. Set the daily close once, in `config/app.toml`.

## The feed that died while the bot stayed up

**This is the most likely thing to actually go wrong in production.**

The process does not crash. It keeps running. It just stops receiving prices.

And a bot with nothing to say looks exactly like a quiet market. You can lose
a week before you notice.

**Defence:** `nsc-live/src/tasks/health.rs`. "Healthy" must mean *receiving
candles*, not *still running*.

## Too many signals

Fifteen a day and you stop reading them. Then you stop pressing 👍/👎. Then
the training data stops arriving and Phase 4 never happens.

**Defence:** limits on open signals, cooldowns per pair, and a quality
threshold you actually enforce.

## Judging a signal after you know how it turned out

Pressing 👎 a week later, when you already know it lost, is not judgement. It
is hindsight — and it teaches the model to predict the past.

**Defence:** press the button when the signal arrives. The replay tool hides
the outcome on purpose.

## Letting the AI talk you into a trade

If the AI can promote a setup your own rules rejected, then the AI is now your
strategy. That is the exact thing this whole design exists to prevent.

**Defence:** the AI can lower confidence or block a trade. It can never
approve one your rules turned down.
