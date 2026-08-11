indicators/ — the few indicators this system uses
=================================================


WHAT THIS FOLDER IS FOR

  You trade off price, not off indicators. So there are only a handful here,
  and they are support, not signal.

  One of them matters far more than the rest.


WHAT IS HERE

  mod.rs               The front door.

  atr/                 Average True Range. How big a normal candle is right
                       now. This is the yardstick every threshold in the
                       project is measured against.

                       Read atr/README.txt. It is the one to understand.

  moving_average.rs    Not written yet.

  rsi.rs               Not written yet.


WHY SO FEW

  An indicator is price, rearranged. It cannot tell you anything price did
  not already say — it just says it more slowly.

  So they earn their place only when they do a job price cannot do directly:

      ATR              turns "how big is this move" into a number that works
                       on any instrument

      RSI              spots when price makes a new low but momentum does
                       not, which is one kind of evidence a trend is tiring

      Moving average   a simple way of asking which way things are pointing

  Everything else in this system is read straight off the candles: swings,
  levels, trendlines, Fibonacci, chart patterns.


THEY ALL UPDATE ONE CANDLE AT A TIME

  Where it is possible, an indicator here keeps a running value and takes one
  candle at a time.

  Two reasons.

  It matches how the live bot works. Candles arrive one at a time, forever.

  And it is fast enough for a settings sweep. The backtester runs these
  millions of times across different settings. Recalculating a whole
  fourteen-candle window on every candle is the difference between a sweep
  that takes seconds and one that eats an afternoon.
