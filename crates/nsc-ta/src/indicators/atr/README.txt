atr/ — how big is a normal candle right now
===========================================


WHAT THIS FOLDER IS FOR

  ATR predicts nothing. It just says how big a normal candle is at the
  moment.

  That sounds dull. It is the reason one settings file works on EURUSD and
  gold at the same time.

  Every threshold in this project is a multiple of ATR:

      how close counts as "at the level"
      how much room the stop gets
      how big a candle is too big to chase
      how far a swing must stand out to count as a swing

  Say you wrote "the stop goes 20 pips below the level". That works on the
  pair you tested it on. On gold, 20 pips is nothing and you are stopped out
  instantly. In a quiet week it is far too wide.

  "The stop goes 0.3 of a normal candle below the level" means the same thing
  everywhere, and adjusts itself when the market gets busier.


THE FILES

  mod.rs        The front door. Says what is in here and what escapes the
                folder: Atr and atr_series.

  running.rs    The Atr struct. Takes candles one at a time and keeps the
                value up to date. This is what the live bot uses.

  series.rs     atr_series. Takes a whole history at once. This is what the
                backtester uses.

  tests.rs      Eight tests.

  README.txt    This file.


HOW THEY FIT TOGETHER

      running.rs  ─►  series.rs      series just runs the same struct in a
                                     loop. There is no second copy of the
                                     maths.

      running.rs  ─►  nsc-core       candles in, PriceDistance out
      running.rs  ─►  error.rs       refuses unfinished candles

      mod.rs      ─►  lets the outside world see Atr and atr_series


TRUE RANGE: WHY IT IS NOT JUST HIGH MINUS LOW

  The obvious way to measure a candle's size is high minus low. That misses
  gaps.

  Say gold closes Friday at 4300 and opens Monday at 4250.

  Monday's candle might only span 4245 to 4260. Fifteen dollars. Looks like a
  quiet day. But price actually moved fifty dollars from where it left off.

  True range fixes that by taking the largest of three:

      high - low                    the candle on its own
      | high - previous close |     the gap up, if there was one
      | low  - previous close |     the gap down, if there was one

  Gaps are real on the indices and oil, which stop trading overnight. Using
  high minus low there would make ATR too small every morning — and then
  every stop measured against it would be too tight, and ordinary opening
  moves would stop you out.

  Test: a_gap_counts_as_movement


SMOOTHING: THE DECISION THAT DECIDES IF WE MATCH YOUR CHART

  ATR averages true range over a number of candles, usually 14. There are two
  ways to average, and they give different numbers forever.

  A PLAIN AVERAGE treats all 14 candles equally and drops the oldest each
  time a new one arrives.

  WILDER'S SMOOTHING keeps a running value and nudges it towards the newest
  candle:

      new = (old x 13 + newest true range) / 14

  It reacts more slowly and never completely forgets.

  WE USE WILDER'S, because that is what your chart uses. TradingView's ATR is
  Wilder's.

  If we used a plain average, our ATR would sit permanently slightly off
  yours. And since every threshold here is a multiple of ATR, every level
  tolerance and every stop distance would drift with it. You would look at a
  signal, measure it on your own chart, and quietly disagree with the bot
  forever without knowing why.


WHY ONE VIOLENT CANDLE BARELY MOVES IT

  Look at the sum again. A new candle only gets one fourteenth of the weight.

  So a candle four times the normal size moves ATR from 10 to about 12, not
  to 40.

  That is deliberate. If ATR jumped every time one big candle printed, every
  stop in the system would widen at exactly the moment volatility spiked —
  which is when you can least afford a wide stop.

  Test: one_violent_candle_only_moves_the_average_a_little


THE FIRST FEW CANDLES ARE NOT TRUSTWORTHY

  Two reasons.

  ATR needs 14 candles before it has any value at all. Until then it returns
  nothing, which is normal — skip those candles and carry on.

  And the very first candle has no previous close, so its true range is just
  its height. Any gap into it is invisible. That makes the first value a
  little too small.

  Neither matters if you have a year of history. Both matter if you try to
  analyse the first day of it.


ONE CANDLE AT A TIME, AND ALL AT ONCE

  The live bot gets candles one at a time. The backtester has the whole
  history.

  Both go through the same struct. atr_series feeds candles into the same Atr
  the bot uses, one after another.

  Not similar code. The SAME code.

  This matters more than it sounds. If the two ever gave different answers,
  your backtest would stop describing what the bot actually does — and you
  would not notice, because that kind of mismatch makes backtests look
  better, not broken.

  Test: one_at_a_time_matches_all_at_once

  Right now that test cannot fail, because there is only one piece of code.
  It is there so that stays true if someone later decides to make the bulk
  version "faster".


UNFINISHED CANDLES ARE REFUSED

  A candle that is still forming has a high and low that have not happened
  yet.

  If ATR accepted one, ATR would change under you as the candle moved. Every
  threshold in the system would move with it, and a setup that passed a check
  one second would fail it the next.

  So update() returns an error. This is not bad market data — it is a bug in
  whatever fed the candle in.
