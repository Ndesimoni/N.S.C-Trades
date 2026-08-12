structure/ — trend, and the moment it is proved
===============================================


WHAT THIS FOLDER IS FOR

  Saying what happened at an old high or low, so that every part of the
  program means the same thing by "higher high" and by "uptrend".

  Finding these is not done here. That is nsc-ta::structure. This folder only
  holds the shape.


THE FILES

  mod.rs        The front door.

  trend.rs      Trend. Up, Down, or Unclear.

  breaks.rs     StructureBreak. An old extreme that price crossed AND carried
                far enough past.

  attempts.rs   FailedAttempt. One it crossed and could not hold past.

  event.rs      StructureEvent. Either of those two, so a caller handles both
                without having to remember that failures exist.

  tests.rs      Nine tests. Five of them are about what gets REFUSED.

  README.txt    This file.


CROSSING A HIGH IS NOT TAKING IT

  Price crossing an old high proves nothing on its own. It has to cross it AND
  carry a share of the run that made it past.

  Poke through by five points and stall, and the high was touched, not taken.
  That poke is the most common trap on a chart: it looks like a breakout, it
  brings buyers in, and price turns straight back down.


A FAILED ATTEMPT IS A RESULT, NOT A SILENCE

  When price crosses and gives up, that gets its own record.

  Not a break, and not nothing. The market tried there and could not hold it.
  Those rows are the "do not take this" examples nothing else in the system
  collects, and they cannot be gathered later — the chart does not remember
  what nearly happened.

  A strategy that uses them does not exist yet. Collecting them now is what
  makes writing one possible.

  The extreme stays on the books afterwards, so a later push that does carry
  far enough still takes it, and the failure sits alongside as what happened
  first.


WHY EVERYTHING IS A SHARE OF THE RUN

  Never pips, and not normal candles either.

  A threshold in normal candles asks the same few points of a 200-point rally
  and a 40-point drift — trivial for one, most of the way for the other. A
  share of the run scales with the move in front of you.

  It also means one number works on the 4-hour and the daily, and on gold and
  EURUSD, which is true of every threshold in this project.


WHAT IS KEPT AFTER THE TEST PASSES

  How far past the extreme price went, as a share of the run.

  A break carrying twice the previous run and one scraping the minimum both
  say "higher high", and they are plainly not the same event. The number
  survives so the rules layer can weigh it. Deciding what it is WORTH belongs
  in config/strategy.toml, not here.


THE CHECK THAT MATTERS

  StructureBreak::new refuses a break dated before the extreme it breaks, and
  FailedAttempt::new refuses an attempt that never crossed at all.

  Neither mistake causes an error anywhere else. They just quietly produce
  history that never happened.


WHERE THIS CAME FROM

  docs/worksheets/structure.md, and docs/diagrams/higher-high.html for the
  picture.
