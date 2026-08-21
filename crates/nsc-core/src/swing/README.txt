swing/ — one swing high or low
==============================


WHAT THIS FOLDER IS FOR

  The type. Not the finding of them -- that is nsc-ta::swings, because finding
  a swing is reading a chart and this crate only holds what the bot KNOWS.

  SWINGS SIT UNDER EVERYTHING ELSE. Levels, trendlines, Fibonacci anchors and
  trend direction are all counted off them, which is why the type lives here
  rather than beside the code that finds them.


THE FILES

  mod.rs      The front door.
  kind.rs     SwingKind -- high or low, and its opposite.
  point.rs    Swing -- the swing itself, and its TWO times.
  error.rs    The one thing that can be wrong: known too soon.
  tests.rs    Six tests, five of them about that one thing.
  README.txt  This file.


THE TWO TIMES ARE THE WHOLE POINT

      bar_time      the candle it sits on -- where you would DRAW it
      confirmed_at  the first moment you could have KNOWN it

  Mixing them up is the single easiest way to produce a backtest that looks
  wonderful and cannot be traded.

  A swing REFUSES TO EXIST if confirmed_at is not after bar_time. You need
  candles after a peak to know it was a peak, so a swing known on its own
  candle is a lookahead bug -- and lookahead bugs do not announce themselves
  any other way. They make results look BETTER, not broken.

  known_by(now) is the question a backtest has to ask. It answers on the
  confirmation and never on where the swing sits.


HOW LATE CONFIRMATION IS, IS NOT FIXED

  The old design confirmed a swing exactly three candles later, always. That
  was a stand-in for the real rule.

  A swing is knowable at the candle where the pullback proved it -- sometimes
  two candles, sometimes thirty. That is the true answer, and it is still
  safe: the moment is measured from candles that have already closed.

  THE COST, WORTH KNOWING: on the daily, a slow shallow pullback can leave a
  swing unconfirmed for weeks. It is on the chart and plain to see, and the
  bot still cannot use it -- because by his own rule it has not proved itself.


THE PRICE IS THE WICK

  Settled 12 August 2026. The swing sits at the high of the candle for a peak
  and the low for a trough, wick included, however long it is.
