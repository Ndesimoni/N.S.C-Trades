tests/ — what proves a swing
============================


THE FILES

  mod.rs         The front door.
  making.rs      Candles to feed the finder, and the real rules file.
  detection.rs   Six tests.
  README.txt     This file.


THE CANDLES ARE BUILT, NOT REAL, AND THAT IS DELIBERATE HERE

  Everywhere else in nsc-ta the tests run on candles that actually printed.
  These do not, because what is being pinned is the RULE and not the market:
  "half of a 100-point run is 50" has to hold on numbers you can check in your
  head.

  The real check is the --bin swings binary, which runs the same finder over
  live IBKR history. On 199 gold 4-hour candles it found 12 swings, six highs
  and six lows, alternating -- and correctly found NOTHING in the most recent
  fortnight, because the run into 4,541 has not given back enough to prove
  itself yet.


THE TESTS THAT MATTER MOST

  nothing_is_ever_confirmed_on_its_own_candle -- the lookahead guard. You need
  candles AFTER a peak to know it was a peak.

  swings_come_out_alternating -- the old windowed finder could call one candle
  both a high and a low.

  two_flat_candles_do_not_make_a_swing -- the left-edge bug. This one was
  found by reading the code back rather than by a test going red.

  the_same_shape_over_more_candles_still_confirms -- the whole reason candle
  counting was thrown out. The same move drawn over four candles or forty
  confirms just the same.
