nsc-ta — reading a chart
========================


WHAT THIS CRATE IS FOR

  Describing what is on a chart. Swings, levels, trends, candle shapes.

  IT DESCRIBES, IT NEVER DECIDES. "This candle is a pin bar" belongs here.
  "Therefore buy" does not — that is nsc-strategy, and it does not exist yet.


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


WHAT IS NOT HERE YET

  Swings, support and resistance, trendlines, Fibonacci and trend direction
  are all planned and none are written.

  TREND IS THE ONE THAT BLOCKS THINGS. A hammer and a hanging man are the same
  candle; what separates them is the trend before it. Until swings exist, this
  crate can only say `long lower wick` -- and it should. The levels the bot
  watches today are the ones HE DREW, read from config/pairs -- see nsc-core.
