nsc-ta — reading a chart
========================


WHAT THIS CRATE IS FOR

  Describing what is on a chart. CANDLES AND WHAT A RUN OF THEM DOES --
  nothing else.

  IT DESCRIBES, IT NEVER DECIDES. "This candle is a pin bar" belongs here.
  "Therefore buy" does not — that is nsc-strategy.


IT CANNOT REACH ANYTHING

  No database, no internet, no async, no reading the clock, no global state.
  If tokio, sqlx or reqwest ever appear in its Cargo.toml, the change is
  wrong.

  THAT IS WHAT LETS THE BACKTESTER AND THE LIVE BOT RUN THE SAME CODE. There
  is no "if we are backtesting, do this instead" anywhere, and there must
  never be — the moment there is, the backtest is testing something different
  from what runs live, and the mismatch makes backtests look BETTER rather
  than broken.

  If something in here needs a database row, it gets handed the row.


NEVER USE PRICE THE MARKET HAD NOT PRINTED YET

  A swing high can only be used at or after the candle that confirmed it.
  Analysing candle 100, you may not read candle 101. Candles that are still
  forming are invisible.

  This mistake does not cause an error. It makes results look better, which
  is what makes it dangerous.


THE FILES

  lib.rs      The front door.

  candle/     What ONE candle is -- four numbers measured before it is
              named, then the twelve shapes it can be.

  pattern/    What a RUN of them does -- engulfing, harami, tweezers,
              piercing line, dark cloud cover, and the star with the
              abandoned baby inside it.

  README.txt  This file.


WHAT IS DELIBERATELY NOT HERE

  Swings, chart patterns, trendlines, Fibonacci and the indicators were all
  built and then REMOVED on 29 August 2026, at his word: he does his own
  analysis and draws his own levels, and the bot works with candlesticks.

  That was not a retreat. The swing finder had a ratchet in it that left it
  blind -- 51 swings in 30,000 candles, and none at all on the Aussie 1-hour
  after March 2025 -- and everything above it stood on that. Taking the chart
  patterns out took the bug with them, because nothing needs swings any more.

  TREND IS THE ONE THING THIS COSTS. A hammer and a hanging man are the same
  candle; what separates them is the trend before it. Without swings this crate
  can only say `long lower wick` -- and it should say exactly that, rather than
  guess.

  The levels the bot watches are the ones HE DREW, read from config/pairs --
  see nsc-core.
