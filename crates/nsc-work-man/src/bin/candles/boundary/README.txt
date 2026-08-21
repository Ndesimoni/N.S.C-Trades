boundary/ — where a feed starts its day
=======================================


WHAT THIS FOLDER IS FOR

  Working out where a feed puts its daily and weekly boundaries, WITHOUT
  asking the feed and without doing arithmetic.


THE FILES

  mod.rs        The front door.

  lined.rs      Lined -- one big candle, and the small candles that opened on
                the same price.

  matching.rs   Lining them up, finding the nearest when nothing matches, and
                deciding whether the candles agree on one answer.

  tests.rs      Four tests, all on what it is allowed to CONCLUDE.

  README.txt    This file.


HOW IT WORKS

  A daily candle's open IS an hourly candle's open -- the same tick, written
  down twice. So the hour that shares the number is the hour the day began.

  Same one level up: the day that shares the weekly candle's open is the day
  the week began.

  NOTHING HERE WORKS A BOUNDARY OUT BY ARITHMETIC. Guessing wrong reads a
  candle before the market printed it, and that does not error -- it makes
  results look better, which is the one kind of wrong nobody catches.


IT REFUSES TO ANSWER ON THIN EVIDENCE

  ONE CANDLE MATCHING IS A COINCIDENCE. A quiet market opens two hours on the
  same number often enough. agreed_on only answers when every candle in the
  sample points at the same hour, and only counts candles that matched exactly
  one smaller candle.

  Two hours sharing an open settles nothing, and taking the first would be a
  guess wearing a measurement's clothes.

  When nothing matches at all it says how far the nearest was, because that
  number is the useful one: out by 0.02 means the boundary is right and the
  feed rounds differently; out by 40 means the boundary is somewhere else.
