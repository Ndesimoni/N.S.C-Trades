candle/ — one candle, and lists of them
=======================================


WHAT THIS FOLDER IS FOR

  A candle is four prices and a time. Open, high, low, close.

  That part is easy. The two rules attached to it are not, and both of them
  cause bugs you cannot see.


THE FILES

  mod.rs        The front door.
                Says what the folder does, and lets the rest of the program
                see two things: Candle and CandleSeries.

  bar.rs        One candle.
                Holds the four prices, the start time, whether it has
                finished, and volume (which is always empty — see below).
                Refuses candles that cannot be real.

  series.rs     A list of candles for one instrument on one timeframe,
                oldest first.
                Decides what happens when a candle arrives out of order, or
                twice.

  tests.rs      Eight tests.

  README.txt    This file.


HOW THEY FIT TOGETHER

      bar.rs  ─►  series.rs        a list holds candles

      bar.rs  ─►  price/           the four prices are Price, and the
                                   height of a candle is a PriceDistance

      series.rs ─► symbol/         which instrument this list is
      series.rs ─► timeframe/      which timeframe

      both      ─► error.rs        bad input is reported, never a crash

      mod.rs    ─►  lets the outside world see Candle and CandleSeries


RULE ONE: open_time IS WHEN THE CANDLE STARTED

  Not when it ended.

  If you store the close time instead, everything shifts by one bar. Your
  levels, your swings, your signals — all one candle out.

  It still looks completely normal. That is what makes it so hard to find.


RULE TWO: AN UNFINISHED CANDLE MUST NOT BE USED

  On a live chart, the candle on the right keeps moving. It has not finished.
  Its high and low have not happened yet.

  If you read one, you are using prices the market has not printed. The
  candle you acted on is not the candle that ends up in the history.

  That is why every candle carries "complete". Check it before you use the
  candle for anything. Later, nsc-backtest::guards stops the whole run if an
  unfinished candle reaches the analysis.


WHAT COUNTS AS AN IMPOSSIBLE CANDLE

  bar.rs refuses a candle if:

      the high is below the low
      the open is outside the high and low
      the close is outside the high and low

  None of those can happen in a real market. If one arrives, the feed sent
  garbage.

  The error says which value was wrong and what the numbers were. That
  matters. Six hours into downloading a year of history, "bad candle" tells
  you nothing. "high 1.0840 is below low 1.0850" tells you everything.


WHAT WE DELIBERATELY DO NOT CHECK

  That prices are positive.

  It is the obvious check to add. It would also be wrong.

  In April 2020 oil traded at MINUS 37 dollars a barrel. Producers were
  paying people to take it away. You trade oil, so that week is real history
  you need.

  A positivity check would quietly delete it.

  The rule for this folder: reject what is IMPOSSIBLE, not what is
  SURPRISING. There is a test called negative_prices_are_allowed whose only
  job is to fail if someone forgets this.


VOLUME IS ALWAYS EMPTY

  The field exists because the database column exists.

  But cash forex has no traded volume, and neither do CFDs. Every instrument
  in config/symbols.toml is one or the other. So this will always be empty.

  Never write a rule that depends on it.


WHAT HAPPENS WHEN CANDLES ARRIVE OUT OF ORDER

  series.rs has two ways in, because loading old history and running live are
  different jobs.

  from_candles   You already have the candles — a year out of the database.
                 Checks the whole lot once. Every candle must come after the
                 one before it. Duplicates are refused.

  push           One candle at a time. This is what the live bot uses.

                 Newer than the last one         → added
                 Same time, last one still forming → replaces it
                 Same time, last one has closed  → refused
                 Older than the last one         → refused

  The middle case is the one people get wrong. A live candle updates over and
  over while it builds, always with the same start time. That is normal, not
  a duplicate.

  The third case is the important one. Once a candle has closed it is
  history. If history can be changed, you can run the same backtest twice and
  get two different answers, with no way to tell which one was right.


GAPS ARE ALLOWED

  A jump in time is fine here. Weekends are real. So are holidays, and the
  overnight close on indices and oil.

  Working out which gaps were expected needs to know about weekends and
  holidays, and this crate is not allowed to know anything about the outside
  world. So nsc-data does that, and records what it finds in the candle_gaps
  table.
