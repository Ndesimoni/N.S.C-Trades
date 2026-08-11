swings/ — finding the peaks and troughs
=======================================


WHAT THIS FOLDER IS FOR

  Turning candles into swing points.

  Spend more time here than anywhere else. Levels, trendlines, Fibonacci
  anchors, trend direction and chart patterns are all built from this one
  output.


WHAT PROVES A SWING

  Not the number of candles either side of it.

  That is how most swing detection works and it asks the wrong question. A
  lazy rounded top with twenty quiet candles around it passes. A sharp turn
  with four candles around it fails. Neither answer matches what you see.

  What proves a peak is what price did AFTERWARDS. So the finder measures the
  RUN — from the last confirmed swing up to the peak — and then how much of
  that run gets given back.

      given back half of the run          the peak is a swing
      given back the shallower share,     the peak is a swing, and so is the
      and then price takes it out         bottom of that pause

  Both numbers are shares of that particular move, so a 300-point rally needs
  about 150 back and a 60-point rally about 30. No units means nothing to get
  wrong when you change instrument or timeframe.


WHY THE SECOND ROUTE EXISTS

  The strongest trends barely pause.

  A rule that only confirmed on depth would read structure perfectly well in a
  choppy market and go blind in a clean trend — which is the market you most
  want to be reading. So a shallower pause counts too, as soon as price takes
  the peak out and proves it WAS a pause.

  When that happens two swings are learned at once: the top of the run, and
  the bottom of the pause.


THE FLOOR

  Half of a tiny run is a tinier pullback, so without a floor a flat Tuesday
  afternoon fills with swings.

  A run has to be at least a share of the BIGGEST of the last few runs. Not of
  the last one — measured against only its predecessor the test ratchets
  downwards, 200 then 120 then 72 then 43, each one passing on its own while
  the chain shrinks to nothing. Against the biggest of five, the third one
  already fails.

  A rejected run is not remembered, so a quiet stretch cannot slowly redefine
  what a big move is. Structure simply goes quiet until a real move comes
  back, which is the honest answer for a market that has gone quiet.


THE FILES

  mod.rs      The front door.

  run.rs      A price with the candle it happened on, and what share of one
              distance another is.

  direction.rs  Which way round a leg is. Every comparison in here is "further
              along the run" or "further back against it", and which of > and
              < that means flips with the direction. Written once, so the
              flips cannot get out of step with each other.

  memory.rs   The last few runs, and whether the next one is big enough to
              count.

  step.rs     What a candle did to the leg it arrived on: nothing, or one or
              two swings and a fresh leg.

  leg.rs      One move in one direction: where it started, how far it has got,
              how much it has given back, and the two ways it can end.

  seed.rs     The start of a history, before anything has been confirmed.
              Keeps the highest and lowest so far, and whichever came LATER
              decides which way the market has been going. Worked out again on
              every candle, so a start that looked like a rise and turns into
              a fall simply reads as a fall.

  finder.rs   One candle at a time, the way the live bot works. Holds the
              state: seeking at first, then following one leg at a time.

  series.rs   A whole history at once, for the backtester. Feeds candles
              through the SAME finder, so the two cannot drift apart.

  tests/      Twelve tests. Read guards.rs first.

  README.txt  This file.


TWO THINGS THAT FALL OUT OF THE RULE

  SWINGS ALTERNATE. After a high the finder is hunting a low. The same candle
  can never be both, which is what a hand-drawn zigzag looks like.

  THE WAIT IS HONEST RATHER THAN FIXED. There is no "confirmed three candles
  later". A swing is knowable when the pullback gets there — sometimes two
  candles, sometimes thirty. On the daily a shallow pullback can leave a swing
  you can plainly see unusable for weeks.

  That last one is the rule being strict, not a bug. By your own rule that
  peak has not proved itself yet.


THE PRICE USED

  Always the wick. The high of the candle for a peak, the low for a trough.


WHAT IS NOT IN HERE

  ATR. The old finder needed it, because its noise filter was measured in
  normal candles. Every number is now a share of a move, so there is nothing
  left for it to measure. ATR still matters for levels and for stops.

  Major and minor swings. There is one flat list, and every swing in it is
  treated the same. If the turns that structure a move need separating from
  the wiggles inside a pullback, that is two thresholds running side by side —
  it changes what this folder RETURNS, not how it decides.


SETTINGS IT READS

  From [swings] in config/ta.toml:

    confirm_retracement   the give-back that proves a peak on its own
    shallow_retracement   the give-back that counts once price takes it out
    min_run_fraction      how big a run must be next to recent ones
    run_memory_legs       how far back "recent" reaches

  See docs/worksheets/swings.md for where these came from, and
  docs/diagrams/swing-pullback.html for the picture.
