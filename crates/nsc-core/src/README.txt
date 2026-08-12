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

  level/          Support and resistance, as a band of price rather than an
                  exact number.
                  Read level/README.txt.

                  One type, not a Support and a Resistance. A support that
                  breaks and later holds price down is the same line doing a
                  different job — you would not rub it out and draw a new
                  one.

                  It carries facts only: the band, the timeframe it was found
                  on, the touch count, the dates. No strength score, no
                  exhausted flag. Whether a level will hold is a judgement
                  that needs the trend and the candle as well, so it lives in
                  nsc-strategy.

                  Carries confirmed_at for the same reason a swing does, and
                  refuses to exist without one that is later than its last
                  touch.


  structure/      Trend, and the moment it is proved.
                  Read structure/README.txt.

                  Taking an old high out is not enough. Price has to cross it
                  AND carry a share of the run that made it past. The poke
                  that crosses and stalls is the most common trap on a chart,
                  and this is what refuses it.


  pattern/        Names for the six candlestick shapes, and one sighting of
                  one. Read pattern/README.txt.

                  A sighting carries the MEASUREMENTS that made it. A pin bar
                  whose wick is nine times its body and one that scrapes the
                  minimum are both pin bars, and a rules layer that only hears
                  "pin bar" cannot tell them apart.

  candle/         ...also holds Proportions: how a candle divides into body
                  and two wicks, as shares of its own height.


WHAT IS STILL EMPTY

  These are stubs. A doc comment and nothing behind it.

  trendline.rs    A drawn line.
  fib.rs          Fibonacci levels.
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
