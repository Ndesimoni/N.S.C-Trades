gaps/ — checking a history is sound before building on it
=========================================================


WHAT THIS FOLDER IS FOR

  Bad data does not fail loudly.

  A missing hour shifts a swing high, which shifts a level, which changes
  every signal after it. The backtest still finishes and still prints a
  perfectly believable number.

  A 4-hour candle built from twelve 15-minute candles instead of sixteen has
  the wrong high and the wrong low, and it looks exactly like a normal candle.


TWO DIFFERENT FAULTS, AND THEY ARE NOT THE SAME THING

  1. A CANDLE MISSING FROM THE FILE

     The broker lost data, or the export was cut short. Apart from the
     weekend and the instrument's nightly break, this is always a fault.

     holes.rs finds it.

  2. A CANDLE THAT IS THERE BUT NEVER MOVED

     Open, high, low and close all at one price. This is NOT missing data.
     The market was open, nothing traded, and the broker printed the candle
     correctly.

     flat.rs finds it.

     It matters because the analysis reads a shelf of these as a real price
     level being defended, when nobody defended it. This project has already
     paid for it once: two flat candles at the left edge of a history invented
     a swing on every instrument, and every test was green.


THE FILES

  mod.rs      The front door.

  holes.rs    Hole and Reason. Walks the candles and reports every place the
              next one is not one step along, and says what accounts for it.

  flat.rs     FlatRun. Finds stretches of candles in a row with no range.

  tests/      17 tests, in three files.
                support.rs  the candles they are run over
                holes.rs    missing candles, and what accounts for them
                flat.rs     candles that are there but never moved

  README.txt  This file.


WHAT COUNTS AS AN EXPECTED HOLE

  Weekend       A new trading week began inside the hole. Friday 17:00 New
                York to Sunday 17:00. The market is shut.

  DailyBreak    The instrument's own nightly hour off. Gold, silver, copper
                and oil shut at 17:00 New York and reopen at 18:00. Spot
                forex does not shut at all.

                This came out of the real export, not out of a guess. The
                15-minute gold file stops at 20:45 UTC and starts again at
                22:00 UTC on EVERY weekday. Ten of those in a fortnight —
                calling them unexplained would have buried the real ones and
                nobody would have read the report again.

                Set per instrument as daily_break_minutes in
                config/symbols.toml. 60 for the metals and oil, 0 for forex.

  Unexplained   Everything else. Worth looking at, every time.

  Christmas Day and New Year's Day still come out as Unexplained. Telling a
  real market holiday from a broker losing an afternoon needs a holiday list,
  which does not exist yet. Saying "unexplained" and letting you look is
  honest; guessing which is which is not.


WHAT IT REFUSES TO DO

  Daily and weekly files. Those candles are not a fixed number of minutes
  apart — clocks change and weekends are three days long — so "one step
  along" is not a subtraction. Guessing would report a hole at every weekend
  and miss the real ones.

  Candles out of order, or two at the same time. That is a broken file rather
  than a hole, and scanning past it would let the rest read as clean.


WHAT IT WILL NEVER DO

  Delete, repair or fill anything in.

  A repaired candle is a made-up candle, and a week later it is
  indistinguishable from a real one. What to do about a hole is a decision,
  and decisions get made by the person, not by the scan.


WHAT IS NOT SETTLED YET

  A weekend hole that ALSO swallowed Friday afternoon is still called
  Weekend. Telling those apart needs the exact Friday close time, which
  belongs in nsc-core::timeframe and is not there. Until it is, read the
  candle count: a normal weekend is a fixed number and a bigger one stands
  out.

  US30 and SPX500 settle at 16:00 New York, an hour before everything else,
  so their daily_break_minutes is almost certainly wrong. It is marked as
  unmeasured in symbols.toml.

  Nothing calls this yet. It runs from a test. Wiring it into the download
  and into the start of a backtest is the next step.


WHAT THE REAL FILES SAY TODAY

  XAUUSD 15-minute, 1,196 candles   2 weekends, 10 nightly breaks, 0 unexplained
  XAUUSD 30-minute, 1,121 candles   5 weekends, 19 nightly breaks, 0 unexplained
  USDCAD 15-minute, 1,283 candles   2 weekends,  0 nightly breaks, 0 unexplained

  No flat runs at all in any of the three. That is a fortnight of liquid
  instruments, so it proves the scan finds nothing where there is nothing —
  not that flat candles are rare.
