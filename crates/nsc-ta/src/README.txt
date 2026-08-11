nsc-ta — reading the chart
==========================


WHAT THIS CRATE IS FOR

  Everything you would do by eye on a chart, done in code.

  Swing highs and lows. Support and resistance. Trendlines. Fibonacci. Trend
  direction. Candlestick patterns. Chart patterns.

  Candles go in. Facts about the chart come out.


THE RULE THIS CRATE LIVES BY

  It never touches the outside world. No database, no internet, no async, no
  reading the clock.

  If you need a database row, take it as an argument.

  Why it matters: this is what lets the backtester and the live bot run the
  SAME analysis code. The moment they run different code, backtest results
  stop describing the bot — and you will not notice, because that kind of
  mismatch makes backtests look better rather than broken.

  If tokio, sqlx or reqwest ever appear in this crate's Cargo.toml, the
  change is wrong.


THE SECOND RULE

  Never use a price the market had not printed yet.

  Analysing candle 100, you may read candles 1 to 100, and swings that were
  confirmed by candle 100. Nothing else.

  A swing high at candle 100 is not knowable until a few candles later,
  however obvious it looks when you scroll back. Using it any earlier is
  using knowledge you did not have.

  This mistake never causes an error. It makes your backtest look BETTER.
  That is what makes it dangerous.


WHAT IS FINISHED

  lib.rs          The crate root. Lists every module and nothing else.

  error.rs        What can go wrong. Three kinds, and they need three
                  different responses:

                    NotEnoughCandles  normal at the start of a history.
                                      Skip and carry on.
                    BadSetting        someone typed a zero into ta.toml.
                                      Stop — retrying will never fix it.
                    IncompleteCandle  a bug in whatever fed the candle in.
                                      Stop.

  config/         The settings from config/ta.toml, as types.
                  Read config/README.txt.

                  This crate cannot read files, so someone else loads the
                  TOML and hands these over. That is also what makes the
                  analysis testable.

  indicators/     ATR so far. Read indicators/atr/README.txt — ATR is the
                  yardstick everything else is measured against.


WHAT IS STILL EMPTY

  These are stubs. A doc comment and nothing behind it.

  swings.rs       Swing highs and lows. Next to be built, and the one to
                  slow down on.
  levels.rs       Support and resistance.
  trendlines.rs   Drawn lines.
  fibonacci.rs    Retracements and extensions.
  structure.rs    Higher highs, lower lows, trend direction.
  candles/        Candlestick patterns.
  patterns/       Chart patterns. Deliberately last.
  aggregate.rs    Building bigger candles from smaller ones.
  context.rs      What one timeframe hands down to the smaller ones.
  snapshot.rs     Everything above, gathered for the rules engine to read.


THE ORDER THINGS GET BUILT IN

      candles
         │
      ATR                  needed before swings, because the filter that
         │                 ignores noise is measured in ATR
      swings               ← everything below is built from these
    ┌────┼────┬──────────┬──────────┐
  levels  trendlines  fibonacci  structure
    └────┴────┴──────────┴──────────┘
                 │
             snapshot      what the rules engine reads

  Candlestick patterns run off the raw candles, alongside all of this.
  Chart patterns run off the sequence of swings.

  Spend more time on swings than anything else. Every level, every trendline,
  every Fibonacci anchor and every trend reading is built from them. Get the
  sensitivity wrong and everything downstream is quietly rubbish, in a way
  that is very hard to trace back.


HOUSE RULES

  No unwrap, expect or panic in the code. Tests may panic.

  A file stays under 200 lines, 250 at the very most, counting tests. When it
  gets too big it becomes a folder with one file per idea — see indicators/
  atr/ for how that looks.

  mod.rs is a front door, not a room. Module docs, what is inside, and what
  the outside world can see. No logic in it.

  Every folder with code gets a README.txt like this one.
