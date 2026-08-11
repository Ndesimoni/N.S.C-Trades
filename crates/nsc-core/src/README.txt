nsc-core — the shared vocabulary
================================


WHAT THIS CRATE IS FOR

  The words every other part of the program speaks in.

  A price. A candle. A swing high. A level. A signal.

  They live here, defined once, so that no two parts of the system can
  disagree about what a candle is. If nsc-ta had its own idea of a candle and
  nsc-strategy had another, the backtester and the live bot would quietly
  stop matching, and you would find out from bad trades rather than an error.


THE RULE THIS CRATE LIVES BY

  Types only. No database. No internet. No clock. No async.

  If you ever need to add tokio, sqlx or reqwest to this crate, whatever you
  are building belongs somewhere else.

  Why it matters: this is what lets the backtester and the live bot run the
  SAME code. The moment a shared type can go and fetch something, the two
  stop being comparable.


WHAT IS FINISHED

  lib.rs          The crate root. Lists every module and nothing else.

  error.rs        Everything that can go wrong, as types.
                  Not strings. The whole point is that the caller can tell
                  "skip this candle and carry on" apart from "your settings
                  are wrong, stop".

  price/          Prices, and the three ways of measuring a gap.
                  Read price/README.txt.

  timeframe/      Which candle a moment belongs to, and where the trading day
                  and week begin.
                  Read timeframe/README.txt.

  symbol/         What an instrument is — pip size, decimals, spread limit,
                  currencies.
                  Read symbol/README.txt.

  candle/         One candle, and lists of them.
                  Read candle/README.txt.

  swing/          A swing high or low.
                  Read swing/README.txt.

                  Carries TWO times, and the difference between them is the
                  point. bar_time is where the swing sits on the chart.
                  confirmed_at is the first moment you could have known it
                  was a swing.

                  Scroll back over any chart and the highs are obvious. That
                  is the trap. When candle 100 printed, nobody knew it was a
                  high — price could have carried on up. It only became one
                  once price turned away.

                  Call is_known_at before using a swing for anything.


WHAT IS STILL EMPTY

  These are stubs. A doc comment and nothing behind it.

  level.rs        Support and resistance.
  trendline.rs    A drawn line.
  fib.rs          Fibonacci levels.
  structure.rs    Higher highs, lower lows, trend direction.
  pattern.rs      Candlestick and chart patterns.
  session.rs      London, New York, Tokyo, Sydney.
  signal.rs       A finished trade idea.


HOW THE FINISHED PIECES FIT TOGETHER

      error.rs
         ▲  ▲  ▲
         │  │  │            everything can fail, so everything uses it
         │  │  │
      price/  timeframe/    both stand alone
         ▲
         │
      symbol/               uses price/ for pip maths and rounding


  Nothing here depends on anything outside this crate.


WHERE THIS SITS IN THE PROJECT

      nsc-core   ◄── you are here. types, no outside world
         ▲
      nsc-ta                 reads the chart, no outside world
         ▲
      nsc-strategy           your rules, no outside world
         ▲
      nsc-backtest / nsc-live    the two things that drive it


  Arrows point from a crate to the one it uses. The clean crates at the top
  never reach down into the messy ones below.


HOUSE RULES IN THIS CRATE

  No unwrap, expect or panic in the code. Tests may panic — a test that
  panics is a test that failed, which is the point.

  Why: the backtester runs this code across years of candles. One bad candle
  must not destroy two hours of work. Bad input gets reported and skipped.

  Keep a file under 200 lines. 250 at the very most, counting tests.
  When it gets too big, turn it into a folder with one file per idea — see
  price/ and timeframe/ for how that looks.
