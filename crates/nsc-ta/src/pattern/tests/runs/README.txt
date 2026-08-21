runs/ — real gold candle runs
=============================


WHAT THIS FOLDER IS FOR

  The candles the pattern tests judge. EVERY ONE OF THEM PRINTED.

  The two engulfings come from the gallery drawn in August. The rest were
  pulled out of live IBKR data on 20 August 2026 by running --bin read over
  gold and reading back the runs it found.


THE FILES

  mod.rs       The front door.

  making.rs    Turning written-down prices back into candles, and how big a
               normal candle was in each era.

  pairs.rs     Runs of two -- engulfing up and down, tweezer bottom, dark
               cloud cover.

  triples.rs   Runs of three -- morning star, evening star, three white
               soldiers, three black crows, and the made-up abandoned baby.

  README.txt   This file.


ONE "NORMAL CANDLE" PER ERA, AND THAT IS THE POINT

  Gold was around 2,030 in February 2024 and around 4,120 in July 2026, and
  its 4-hour range grew with it.

  The first version of these tests used a single 20 for everything, and the
  real July 2026 tweezer failed: its lows are 1.70 apart, which is inside
  tolerance on a 35-point candle and outside it on a 20-point one.

  THE CANDLES WERE RIGHT AND THE YARDSTICK WAS WRONG. It is why ending_at
  takes `normal` as an argument instead of looking one up.


ONE OF THEM IS MADE UP, AND IT SAYS SO IN ITS NAME

  abandoned_baby_made_up. No real one exists in the gold data and none can --
  spot forex does not gap mid-week. It is the real morning star with its
  middle candle moved clear of both neighbours, which is the only way to
  exercise the strict case at all.
