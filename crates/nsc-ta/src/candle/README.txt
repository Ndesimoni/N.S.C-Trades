candle/ — what ONE candle is
============================


MEASURED FIRST, NAMED SECOND

  A doji, a hammer and a spinning top are the same four numbers with
  different thresholds laid over them. So the measuring happens once, in
  shape.rs, and the naming happens after, with numbers from config/.

  Bury the threshold inside a function called is_doji and changing your mind
  means changing code. Keep it in config/ and it is a restart.


THE FILES

  mod.rs      The front door.

  shape.rs    Shape. The four numbers, and nothing with an opinion in it.

  tests.rs    Seven tests, every one on a candle that actually printed.

  README.txt  This file.


THE FOUR NUMBERS

  body, upper, lower   Shares of the whole candle. They add to one -- but
                       see the note at the bottom before you test that.
  reach                The whole candle divided by ATR.

  REACH IS THE ODD ONE OUT AND IT EARNS ITS PLACE. The other three say what
  shape a candle is. reach says whether it is worth looking at at all.

  Of 394 dojis found in real gold, only 240 reached half a normal candle. The
  other 154 had tiny bodies because nothing happened that hour. No ratio
  inside a candle can tell those apart.

  And it is in ATR, not points, because three points is nothing on gold and a
  week on the euro. A points threshold works on the pair it was set on and
  quietly stops working on every other one.


A CANDLE CAN HAVE NO SHAPE

  Shape::of gives back None when the range is zero.

  That is not a fault and it is not rare. The feed sends weekend and holiday
  candles: seven of 5,000 gold 4-hour candles have the high, low, open and
  close all the same number, and 1,412 hourly candles have a range under
  0.02% of price.

  Dividing by that would make every number meaningless without saying so.


IT DESCRIBES, IT NEVER DECIDES

  "This is a doji" belongs here. "This is a buy" belongs in nsc-strategy.

  Keep that line and a rule can change without touching a pattern, and a
  pattern can be added without touching a rule.


THEY ADD TO ONE, BUT NOT EXACTLY

  Each share is its own division and each rounds at the 28th significant
  digit. Three of those do not add back to a clean one -- a real gold doji
  gives 1.0000000000000000000000000001.

  Decimal keeps 0.1 + 0.2 honest. It does not make division exact.

  So nothing may test `body + upper + lower == 1`. Nothing needs to: every
  pattern compares one share against one threshold.

  The first version of that test asserted an exact one and failed on the very
  first real candle. The test was wrong, not the code.
