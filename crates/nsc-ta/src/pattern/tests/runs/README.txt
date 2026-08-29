runs/ — real candle runs
========================


WHAT THIS FOLDER IS FOR

  The candles the pattern tests judge. EVERY ONE OF THEM PRINTED.

  The two engulfings come from the gallery drawn in August. The rest were
  pulled out of live IBKR data -- the gold runs on 20 August 2026, the pushes
  on 21 August, by sweeping all five pairs in config/pairs across every
  timeframe from 30 minutes up.


THE PAIR DOES NOT MATTER AND NEITHER DOES THE TIMEFRAME

  His instruction, and also the design.

  A setup is a SHAPE. The same shape on the euro's 30-minute and on gold's
  weekly is the same setup, and it is stored, tested and named the same way.
  Nothing in these rules asks which pair it is or which timeframe it came off.

  That is why every threshold is either a share of the candle's OWN height or
  a multiple of the NORMAL candle at that moment -- never points, never pips.
  A body that is a fifth of its candle is a fifth on EURUSD and a fifth on
  gold. Write one number in points and it works on the pair it was set on and
  quietly stops working on every other one.

  So the runs below are deliberately mixed: gold, the euro and the pound, on
  daily and 30-minute, in one file and judged by the same numbers.


THE FILES

  mod.rs       The front door.

  making.rs    Turning written-down prices back into candles, and how big a
               normal candle was in each era.

  pairs.rs     Runs of two -- engulfing up and down, tweezer bottom, dark
               cloud cover. Gold.

  triples.rs   Runs of three -- morning star, evening star, three white
               soldiers, three black crows, and the made-up abandoned baby.
               Gold.

  pushes.rs    Runs that make -- or just miss -- HIS own pattern, nsc-bull and
               nsc-bear. Gold, the euro and the pound; daily and 30-minute.
               Each run carries its OWN normal candle rather than borrowing a
               shared one, because they come from different pairs and eras.

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
