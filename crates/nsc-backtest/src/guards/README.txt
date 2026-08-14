guards/ — refusing data you did not have yet
============================================


WHAT THIS FOLDER IS FOR

  Killing a backtest the moment the analysis touches something that had not
  happened yet.

  Not warning. Killing. A number with a warning attached still gets read,
  compared and acted on weeks later — especially a good one.


WHY IT NEEDS TO EXIST AT ALL, WHEN THE TYPES ALREADY CHECK

  The types refuse a bad thing one at a time. A Swing cannot claim to have
  been confirmed before the candle it sits on. A half-formed Candle cannot
  become a BarClosed.

  None of them can see the RUN.

  A swing confirmed on Friday is a perfectly good swing. It is only wrong if
  something reads it on Tuesday. Nothing about the swing says so — only the
  moment it was read does, and only the run knows that moment.

  Guard is that moment, made into something you have to walk through.


THE FILES

  mod.rs       The front door.

  watcher.rs   Guard. Stands at one clock time. Hands back swings, levels and
               candles that were knowable at it, and kills the run otherwise.

  tests/       21 tests, in three files.
                 support.rs   the candles, swings and levels they share
                 clock.rs     the clock-versus-stamp mistake — read this first
                 refusals.rs  what gets through and what kills the run

  README.txt   This file.


THE PART THAT WAS WRONG FIRST, AND IS THE WHOLE POINT

  Everything here is stamped with its candle's OPEN time.

  A 4-hour candle running 21:00 to 01:00 is stamped 21:00. A swing it confirms
  carries that same 21:00. But nobody knew what that candle would do until
  01:00, when it finished.

  So the guard holds the clock time the bar FINISHED, and works the same thing
  out for whatever it is handed. That is why you pass the timeframe in — a
  stamp on its own does not say when it became true.

  Compare stamp against stamp and you get it wrong in both directions:

    - a 4-hour reading looks knowable four hours early
    - a 15-minute swing from inside those four hours gets thrown out, even
      though it plainly happened

  Tests: a_four_hour_swing_is_not_knowable_until_its_candle_closes
         a_fresh_smaller_swing_survives_a_bigger_bars_guard

  Both fail if the check goes back to comparing stamps. That was checked, not
  assumed.


HOW IT IS USED

    let guard = Guard::at(&bar, boundary)?;
    let swings = guard.swings(&swings, bar.timeframe())?;
    let levels = guard.levels(&levels)?;

  It hands the thing BACK rather than returning nothing. "Check it, then use
  it" is two lines, and the day someone deletes the first one the run goes
  quiet instead of loud. Making the guard the only way to hold the value means
  the step cannot be skipped.

  Levels need no timeframe argument — a level already knows which chart it
  came from. Swings do not carry one, so the caller says.


WHAT IT CANNOT CATCH

  Anything the analysis never shows it. This is a gate, not a search. It only
  sees what walks through.

  So the rule stays "reads go through the guard". This folder just makes that
  rule cheap enough to follow that nobody is tempted not to.


THERE IS NO OFF SWITCH

  On purpose. A check you can turn off is a check that is off on the one run
  whose number you end up believing.
