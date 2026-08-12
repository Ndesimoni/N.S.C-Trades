structure/ — which way the market is going
==========================================


WHAT THIS FOLDER IS FOR

  Counting higher highs and lower lows off the swing points, and saying which
  way the market has last proved it is going.

  Swings go in. Breaks of structure come out, each carrying how far past the
  old extreme price went.


THE RULE

  Taking an old high out is NOT enough.

  Price has to cross it AND carry a share of the run that made it past —
  measured from the high itself, not from where the pullback started.

      the run           1900 up to 2100        200 points
      the cross         price clears 2100      the test starts
      what must follow  2180                   40% of 200 past it

  Poke through by five points and stall, and the high was touched, not taken.

  Why that matters: the poke is the most common trap on a chart. It looks like
  a breakout, it brings buyers in, and price turns straight back down. A bot
  without this rule reads it as a higher high, calls the trend intact, and
  goes looking for a long at the worst possible moment.

  Lower lows work the same way, mirrored. It is ONE piece of code with a
  direction passed in — an uptrend and a downtrend judged by slightly
  different rules is how a bot ends up bullish and bearish about the same
  chart on the same day.


WHY IT IS A SHARE OF THE RUN

  ta.toml used to ask for a fraction of a normal candle instead.

  A share of the run is the better yardstick. A 200-point rally and a
  20-point drift are different events, and what counts as real follow-through
  should change with the move in front of you rather than with how big candles
  happen to be this week.

  It also means the same number works on the 4-hour and the daily, and on gold
  and EURUSD, which is true of every threshold in this project.


A CROSS THAT STALLS IS NOT THROWN AWAY

  If price crosses and stops short, the extreme stays on the books. A later
  candle that carries far enough completes the break then.

  The test is about how FAR price got, not how quickly.

  The one thing that clears an extreme early is a NEWER swing high confirming.
  Then that newer high becomes the one to take out, even when it is lower than
  the old one — "the previous high" means the most recent one.


A PUSH THAT GIVES UP IS RECORDED

  Price crosses, runs out of steam short of the follow-through, and comes back
  under. That gets its own row: which extreme, how far it got, when it started
  and when it was over.

  Not a break, and not nothing. The market tried there and could not hold it.
  Those are the "do not take this" examples nothing else in the system
  collects, and they cannot be gathered afterwards.

  A failed attempt never moves the trend. It is evidence, not a direction.

  One push is one attempt, however much it wobbles above the line while it
  lasts. It ends when a whole candle fails to get past the extreme at all.


THE FILES

  mod.rs      The front door.

  watch.rs    An extreme being watched, and what price does at it. Three
              outcomes: taken, a push still under way, or a push that gave up.
              The run travels with the extreme, so the two cannot be paired up
              wrongly later.

  reader.rs   One candle at a time, the way the live bot works. Holds the
              extremes being watched and the trend so far.

  series.rs   A whole history at once, for the backtester. Feeds candles
              through the SAME reader, so the two cannot drift apart.

  tests/      Fourteen tests. Read guards.rs first.

  README.txt  This file.


THE FIRST SWING OF A HISTORY

  The run behind a high is normally the move up from the swing low before it.
  The first swing has no swing before it.

  Rather than ignore it, the lowest price seen so far stands in — that is
  where the move came up from, as far as anything here can know. Without that,
  the bot would ignore the first high of every history, including the one it
  starts trading on.


WHAT IS DELIBERATELY MISSING

  CHANGE OF CHARACTER — the first swing that breaks the pattern, the earliest
  hint that a trend is turning.

  It is a real idea and the module docs used to promise it. It has not been
  described by the trader yet, and inventing it would put a rule in the bot
  that nobody agreed to and that nobody could explain afterwards.

  A SECOND WAY TO ACCEPT A HIGHER HIGH. The rule above was described as
  "option 1", so there is at least one more. When it arrives it is an extra
  route to acceptance, like the shallow route in swings/, not a change to this
  one.

  WHETHER THE TREND NEEDS HIGHER LOWS TOO. Today the trend is simply the
  direction of the last extreme properly taken out. Textbook structure also
  wants the pullback low to be higher than the one before it. That has not
  been asked about, so it is not assumed.


SETTINGS IT READS

  From [structure] in config/ta.toml:

    min_follow_through   how much of the run must be carried past

  See docs/worksheets/structure.md for where it came from, and
  docs/diagrams/higher-high.html for the picture.
