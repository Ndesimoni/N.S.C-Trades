swings/ — finding swing highs and lows
======================================


WHAT THIS FOLDER IS FOR

  Finding them. The TYPE lives in nsc-core::swing, because that is what the
  bot knows; this is how a chart is read to produce one.

  SWINGS SIT UNDER EVERYTHING ELSE. Levels, trendlines, Fibonacci anchors and
  trend direction are all counted off them.

  Settled 12 August 2026. docs/worksheets/swings.md.


THE FILES

  mod.rs       The front door.
  rules.rs     The four numbers, all shares of a move.
  extreme.rs   A price and its candle, and the arithmetic of a run.
  facing.rs    Which way round a leg is, written ONCE.
  memory.rs    The recent runs, and what counts as a move at all.
  leg.rs       THE RULE -- the two ways a peak proves itself.
  seed.rs      The start, before any swing has confirmed.
  step.rs      What a candle did to the leg it arrived on.
  finder.rs    The state machine, one candle at a time.
  tests/       Six tests.
  README.txt   This file.


NO CANDLE COUNTING

  A peak is not a peak because of how many candles sit either side of it.

  That question passes a lazy rounded top with twenty quiet candles round it,
  and fails a sharp turn with four. It was a stand-in and it was the wrong
  question.

  WHAT PROVES A PEAK IS WHAT PRICE DID AFTERWARDS.


TWO ROUTES, AND THE SECOND ONE MATTERS MORE

      DEPTH        price gives back HALF the run
                   -> confirmed at the candle that reaches half

      RESUMPTION   price gives back NEAR half (0.382), then takes the peak out
                   -> confirmed at the candle that clears the peak, and BOTH
                      ends of the pause are swings at once

  The second exists because THE STRONGEST TRENDS BARELY PAUSE. A rule that
  only confirmed on depth would read structure fine in chop and GO BLIND IN A
  CLEAN TREND -- which is exactly the market worth reading.

  Trend strength shows up as pullback depth. Throwing away the shallow case
  throws away the strongest trends.


EVERY NUMBER IS A SHARE OF THE MOVE

  Half of THAT move, not half of a fixed distance. A 300-point rally needs
  about 150 back; a 60-point rally about 30.

  Which is why the same four settings work on the 4-hour and the daily, and on
  gold and EUR/USD. The per-timeframe lookback numbers the old design needed
  were only ever trying to approximate this.


THREE THINGS THAT FALL OUT OF IT, ALL GOOD

  SWINGS ALTERNATE. High, low, high, low, the way you would draw them. The old
  windowed finder could call the same candle BOTH -- an outside bar that beats
  everything around it in both directions. Under this rule that cannot happen,
  because after a high it is looking for a low.

  ONE SETTING WORKS EVERYWHERE, because half a run is a ratio.

  CONFIRMATION GETS HONEST INSTEAD OF FIXED. The old rule said three candles,
  always. Now it is the candle where the pullback proved it -- measured on
  real gold: 1 candle at best on the 4-hour, and 21 CANDLES on the daily.

  THE COST IS REAL AND IT IS THE RIGHT COST: on the daily a slow shallow
  pullback can leave a swing unconfirmed for weeks. It is on the chart and
  plain to see, and the bot still cannot use it -- because by his own rule it
  has not proved itself yet.


THE GUARD THAT CATCHES THE LEFT EDGE

  A run that starts and ends on the SAME CANDLE is not a run -- it is one
  candle's height.

  Without that check, two flat candles in a row confirm a swing: the whole of
  that "run" is given back inside the next candle, which passes every share
  test there is. It bites hardest at the very start of a history, where there
  is no memory of earlier runs to measure against.


THE MEMORY COMPARES AGAINST THE BIGGEST, NOT THE LAST

  Each run being half the one before it would otherwise pass forever while the
  chain shrank to nothing: 200, 120, 72, 43, 26.

  Against the biggest of the last five, the third one already fails and the
  shrinking stops.
