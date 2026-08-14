events/ — where the backtester and the live bot meet
====================================================


WHAT THIS FOLDER IS FOR

  One event: a candle finished.

      backtester ─┐
                  ├─→ BarClosed ─→ everything else
      live bot   ─┘

  The backtester replays these out of a file as fast as it can. The live bot
  builds them from the broker feed. Everything downstream takes the same type
  and cannot tell which one it is talking to.


WHY THERE IS ONLY ONE EVENT

  This is the mechanism behind the project's main rule.

  If any code anywhere asks "am I backtesting?", the backtest has stopped
  describing the live bot, and the whole point of testing against history is
  gone.

  Having one event and one type is what makes that question impossible to ask.


THE FILES

  mod.rs          The front door.

  bar_closed.rs   The event. A symbol, a timeframe, and a finished candle.

  tests.rs        Five tests.

  README.txt      This file.


THE TWO THINGS IT GUARANTEES

  1. THE CANDLE IS COMPLETE.

     BarClosed::new refuses a candle that is still forming. There is no way to
     build one around a half-formed candle, so nothing downstream has to
     remember to check.

     That matters because an unfinished candle's high and low have not
     happened yet. Reading one does not error — it quietly uses prices the
     market never printed, and it makes the results BETTER, which is what
     makes it dangerous.

  2. THERE IS ONE ANSWER TO "WHAT TIME IS IT".

     event.at() is the moment to pass to every is_known_at check.

     Without a single source for that, two pieces of code eventually disagree
     by one candle, and one of them is reading the future.


WHY at() IS THE OPEN TIME, NOT THE CLOSE

  This looks wrong and is not.

  A swing confirmed by this candle is stamped with THIS CANDLE'S OPEN TIME —
  that is how timeframe/ and swings/ record it. So:

      is_known_at(at())   yes for everything knowable now
                          no  for anything that needed the next candle

  Use the close time instead and swings become knowable one candle early.
  Every level built from them shifts, every signal moves, and nothing
  anywhere reports a problem.

  There is a test called
  a_swing_confirmed_by_this_candle_is_knowable_and_the_next_one_is_not.


WHY THE SYMBOL IS SHARED RATHER THAN COPIED

  A Symbol holds three strings. Six years of 15-minute candles is about sixty
  thousand events per instrument per timeframe.

  Copying the symbol into every event would mean millions of allocations to
  carry a fact that never changes, so the event holds a shared handle instead.
  Cloning one is a counter bump.

  There is a test that checks two clones point at the same symbol.


WHAT IS NOT HERE

  Anything about ticks, spreads or partial candles. The analysis only ever
  sees finished candles, and this is the only door it comes through.
