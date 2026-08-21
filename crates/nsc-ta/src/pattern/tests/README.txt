tests/ — patterns, on runs that actually printed
================================================


THE FILES

  mod.rs      The front door.

  settings.rs The thresholds the tests judge against -- the ones out of
              config/patterns.toml, not a set made up for the tests. A
              threshold that only exists in a test is one nobody has to live
              with, and the first thing it does is pass.

  runs/       The candle runs, every one of which printed. Its own README.

  two.rs      Two candles together. Seven tests.

  three.rs    The star, the abandoned baby and the march. Ten tests.

  README.txt  This file.


ONE "NORMAL CANDLE" PER ERA, AND THAT IS THE POINT

  Gold was around 2,030 in February 2024 and around 4,120 in July 2026, and
  its 4-hour range grew with it.

  The first version of these tests used a single 20 for everything, and the
  real July 2026 tweezer failed: its lows are 1.70 apart, which is inside
  tolerance on a 35-point candle and outside it on a 20-point one.

  THE CANDLES WERE RIGHT AND THE YARDSTICK WAS WRONG. That is exactly why the
  taxonomy said ATR is worked out AS IT GOES, and why ending_at takes `normal`
  as an argument instead of looking it up.


ONE RUN IS ONLY EVER ONE ANSWER

  The 7 June 2024 candles are the clearest bearish engulfing AND the clearest
  tweezer top in three years. a_run_that_is_two_things_comes_back_as_one pins
  which one wins.


THE ABANDONED BABY IS MADE UP, AND IT SAYS SO

  No real one exists in the gold data, and none can -- spot forex does not gap
  mid-week. abandoned_baby_made_up is the real morning star with its middle
  candle moved clear of both neighbours, which is the only way to exercise the
  strict case at all.

  a_real_star_never_comes_back_abandoned is the test that matters: every real
  star on gold comes back with the flag FALSE.


TWO ASSERTIONS THAT WERE WRONG THE FIRST TIME

  Both said "is_none()" where they meant "is not THAT pattern".

  A run that fails the engulfing test does not vanish -- it falls through and
  is judged as whatever else it is, which for the bare-cover case is a tweezer
  bottom, because those two candles really do share a low.

  The running order doing its job looks like a miss if you assert the wrong
  thing.
