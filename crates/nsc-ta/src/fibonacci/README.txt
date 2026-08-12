fibonacci/ — drawing retracements over a move
=============================================


WHAT THIS FOLDER IS FOR

  Taking the move price has just made, and working out where the retracement
  levels sit on it.


THE RATIOS ARE TRIVIAL. THE MOVE IS THE WHOLE GAME

  The same four ratios drawn from a different pair of swings give completely
  different prices. So which move gets measured is the only decision here that
  matters.

  THE MOVE IS THE LAST COMPLETED LEG: the two most recent confirmed swings.
  That is what price is retracing right now, and it is the same run the swing
  finder measured to confirm those swings in the first place.

  Anything cleverer — the biggest recent move, the move in the direction of
  the trend — would let two parts of the chart-reading code disagree about
  what the current move is. That disagreement stays invisible until a signal
  looks wrong and nobody can say why.

  The move is kept and handed on with the levels. When a Fibonacci signal
  looks wrong, the move it chose is nearly always the disagreement — and an
  argument about a move is one you can settle by looking at a chart.


THE FOUR LEVELS EACH DO A DIFFERENT JOB

  This is not one zone with lines in it.

      0.382         A READING, not an entry. A pullback this shallow means
                    the market barely paused, which is what a strong trend
                    looks like. Same number as shallow_retracement in
                    [swings], because it is the same belief.

      0.5 to 0.618  THE GOLDEN ZONE. The most important two, and where to
                    look to get in.

      0.786         WHERE STOPS GET LOOKED AT. Not always, and never on its
                    own — other factors come into it.

  This folder reports where those prices are and how deep price has come back.
  It decides nothing. The location layer picks an entry and the invalidation
  layer places a stop, both in nsc-strategy.


THE FILES

  mod.rs        The front door.

  draw.rs       Picking the move. Ignores swings that had not confirmed yet,
                so a move can never be drawn from a swing the market had not
                printed.

  reading.rs    The four prices, and how deep price is now as a share of the
                move.

  tests.rs      Thirteen tests.

  README.txt    This file.


SETTINGS THAT REFUSE TO MAKE SENSE TOGETHER

  Three checks that catch a config nobody would notice was wrong:

    The strong-trend level has to be SHALLOWER than the zone. At or past it,
    it says nothing the zone does not already say.

    The stop level has to be BEYOND the zone, or the stop would be hit by the
    entry it is supposed to protect.

    The shallow edge of the zone comes first, and the two cannot be equal.


STILL OPEN

  WHICH TIMEFRAME the move is measured on, and what to do when a bigger move
  is still running inside a smaller one. See docs/worksheets/fibonacci.md.

  EXTENSIONS. 1.272 and 1.618 are in the settings for targets and are the
  textbook pair rather than the trader's. Nothing reads them yet.
