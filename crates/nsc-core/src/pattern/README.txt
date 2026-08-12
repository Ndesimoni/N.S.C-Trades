pattern/ — names for the shapes a chart makes
=============================================


WHAT THIS FOLDER IS FOR

  Saying what a candlestick pattern IS, so every part of the program means the
  same thing by "pin bar".

  Finding them is not done here. That is nsc-ta::candles. This folder only
  holds the shape.


THE SIX SHAPES

  Pin bar, engulfing, doji, belt-hold, tweezers. That is the whole list, and
  it is the list the trader actually uses rather than a textbook index.

  Hammer and inverted hammer are not separate. A hammer IS a bullish pin bar.
  Textbook separates them by what came before — a hammer follows a downtrend —
  and that is context, not shape.

  A pattern in the code that nobody would act on is noise in every backtest it
  turns up in.


THE FILES

  mod.rs        The front door.

  shape.rs      CandleShape, DojiKind and Bias. Which pattern, which sort of
                doji, and which way it points if it points anywhere.

  sighting.rs   PatternSighting. One shape found on one candle, with the
                measurements that made it one.

  tests.rs      Eight tests.

  README.txt    This file.


THE MEASUREMENTS TRAVEL WITH THE SIGHTING

  A pin bar whose wick is nine times its body and one that scrapes past the
  minimum are both pin bars. They are not the same candle.

  A rules layer that only hears "pin bar" cannot tell them apart, and it
  cannot go back and measure afterwards. So the proportions of the candle come
  along with the name.

  What those measurements are WORTH is decided in config/strategy.toml. Not
  here.


A DOJI POINTS NOWHERE

  Bias has three values, not two. A doji is the market failing to pick a side,
  which is exactly why it needs context to mean anything, and calling it
  bullish or bearish would be inventing a direction it does not have.


WHERE THE SHAPE OF A CANDLE LIVES

  In candle/proportions.rs, not here — body, upper wick and lower wick as
  shares of the candle's own height.

  That is a fact about a candle rather than about a pattern, and putting it on
  Candle means the detectors all measure the same way instead of each doing
  its own arithmetic.


WHAT IS NOT HERE

  CHART patterns — head and shoulders, triangles, flags, double tops. Many
  swings, far more subjective, and much weaker statistically.

  Deliberately left for later: trend plus levels plus Fibonacci plus
  candlesticks gives most of the edge for a fraction of the work.


WHERE THIS CAME FROM

  docs/worksheets/candles.md. The numbers behind these shapes are TEXTBOOK,
  not the trader's own, and that file says so.
