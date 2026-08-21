fibonacci/ — retracements over a move
=====================================


THE RATIOS ARE THE EASY PART

  WHICH MOVE YOU MEASURE IS THE WHOLE GAME. The same ratios drawn from a
  different pair of points give completely different prices.

  So this stores the MOVE and works the levels out from it, rather than
  storing the levels. When a Fibonacci reading looks wrong, the move it picked
  is nearly always the disagreement -- and an argument about a move is one you
  can settle by looking at a chart.


THE FILES

  mod.rs      The front door.

  leg.rs      Leg -- the move, and the two questions you ask of it: where is a
              given share, and how deep is price now.

  rules.rs    Which shares matter, and what job each one has. Read out of
              config/fibonacci.toml.

  reading.rs  Where price sits in the move, and the lines it draws.

  tests.rs    Eleven tests.

  README.txt  This file.


IT DOES NOT KNOW WHERE THE MOVE CAME FROM

  And that is deliberate. Swings will anchor it one day; his own drawn levels
  can anchor it today. Neither belongs in here, and building it this way meant
  the anchoring question did not have to be settled before the maths could be.


FOUR LEVELS, AND EACH ONE HAS A JOB

  A LEVEL WITH NO JOB ATTACHED is a line the bot draws and nothing reads. That
  is why the settings are named rather than a list of ratios -- 0.786 is not
  "the fourth level", it is where a stop gets looked at.

      0.382   NOT AN ENTRY. A reading about the MOVE: a pullback this shallow
              means the market barely paused, which is what a powerful move
              looks like. Reading it as "not deep enough to buy" throws away
              the only thing it was telling you.

      0.5     the golden zone -- where to look to get in
      0.618

      0.786   where a stop gets LOOKED AT. Not always, and never on its own:
              this crate draws the line and nsc-strategy decides whether the
              stop actually goes there. A stop placed by one line every time
              is a stop everybody can see.

  The extensions -- 1.272 and 1.618 -- are for targets and are NOT CONFIRMED.
  They are the standard numbers, not his.


TWO PIECES OF ARITHMETIC THAT ARE EASY TO GET BACKWARDS

  A RETRACEMENT COUNTS BACK FROM THE EXTREME. 0.618 of a move up from 100 to
  200 is 138.2 -- the amount GIVEN BACK. Get it the wrong way round and every
  level is mirrored to 161.8, which is a real price on the chart and looks
  perfectly plausible.

  AN EXTENSION GOES PAST THE EXTREME, and is not a retracement with a number
  over one. 1.272 of that move is 227.2. Run it through the retracement
  formula and you get -27.2 -- off the bottom of the chart on this example,
  but a believable number on a real pair.

  One formula serves both directions, because the run carries the sign: the
  same 0.618 on a move DOWN from 200 to 100 gives 161.8.
