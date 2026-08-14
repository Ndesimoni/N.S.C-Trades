replay/ — handing history over one candle at a time
===================================================


WHAT THIS FOLDER IS FOR

  Giving the analysis the same sequence of events the live bot would have
  received, in the same order.

  The point is not to read a file. It is that a result from history has to
  describe what the bot actually does — and it only does if the bot and the
  backtester see the same things in the same order.


THE FILES

  mod.rs      The front door.

  walker.rs   Replay. Takes the file's candles one at a time and gives back
              every bar that finished because of it.

  tests.rs    Seven tests. Read the last one first.

  README.txt  This file.


WHAT IT DOES THAT A PLAIN LOOP DOES NOT

  A 15-minute candle at 16:45 also finishes the 4-hour candle that began at
  13:00.

  The live bot learns BOTH at that moment. So the replay builds the bigger
  timeframes as it goes and gives back a BarClosed for each one that finished
  — not just the 15-minute one it was handed.

  Feed it 400 candles and you get 400 fifteen-minute bars, 200 half-hour bars,
  100 hourly bars, and so on, arriving exactly when they became true.


BIGGEST TIMEFRAME FIRST, AND THIS IS NOT OPTIONAL

  At 16:00 the 30-minute, the 1-hour and the 4-hour can all finish together.

  Smaller timeframes read the bigger ones for context. So the bigger ones have
  to have moved BEFORE the smaller ones are evaluated.

  Run them the other way and the 30-minute reads a 4-hour that has not been
  updated yet. Same candles, different answers — and the backtester and the
  bot would disagree, which is the one thing this whole design exists to
  prevent.

  This is evaluate_largest_first in config/app.toml.

  Test: bars_come_out_biggest_timeframe_first


THE TEST THAT MATTERS MOST

  replaying_gives_the_same_candles_as_building_them_all_at_once

  The chart tool builds a whole history at once. The replay does it one candle
  at a time. If those two ever produced different candles, every backtest
  would stop describing the bot — and it would not look broken, it would just
  look different.

  They cannot differ today, because both go through the same aggregator. That
  test is what keeps it that way when someone later decides the bulk path
  could be faster.


WHAT IT REFUSES

  A timeframe smaller than the file. You cannot cut a 15-minute candle into
  5-minute ones, and quietly ignoring the request would leave you wondering
  why no bars ever arrived.

  A candle that is still forming. Its high and low have not happened yet.

  Asking for the base timeframe twice does not emit it twice.


WHAT SITS NEXT TO IT

  guards/ — the watcher that kills a run when a swing or level is used before
  it could have been known. Build a Guard from each bar this folder emits.


WHAT IS NOT HERE YET

  harness.rs — running the strategy over the replayed bars. Waits on
  nsc-strategy, which is Phase 2.
