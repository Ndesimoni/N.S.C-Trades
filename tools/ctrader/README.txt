tools/ctrader/ — getting candles out of cTrader
===============================================


WHAT THIS FOLDER IS FOR

  One small C# indicator that writes the bars on a cTrader chart to a CSV
  this project can read.

  It is not part of the bot. It runs inside cTrader, on a laptop, when you
  want data. Nothing in crates/ knows it exists.


WHY IT EXISTS AT ALL

  cTrader's own export moves around between versions, and the menu path that
  works on one build is missing on the next. A script that runs inside the
  platform does the same job the same way every time.

  It also does two things a menu export would not.


  IT CONVERTS TO UTC.

  cTrader charts are usually in broker server time, often two or three hours
  off UTC. An export in broker time shifts every candle — and with it every
  level the bot draws, because where the daily candle starts decides where
  daily levels sit.

  Doing it here means it is done once, correctly, rather than guessed at from
  a file that does not say what timezone it is in.


  IT LEAVES OUT THE NEWEST BAR.

  While the market is open, the newest bar is still forming. Its high and low
  have not finished happening. Nothing in a CSV can say so, and a bot that
  reads one is using prices the market has not printed.


THE FILES

  ExportBars.cs   The indicator. Instructions are in the comment at the top.

  README.txt      This file.


THE ONE THING THAT CATCHES PEOPLE OUT

  cTrader only holds the bars it has actually loaded.

  Open the chart, SCROLL LEFT until it has as much history as you want, and
  only then add the indicator. A chart you have not scrolled back on will
  export a few hundred bars and look like it worked.

  Two years of daily is about 520 bars, which is what the level lookback
  window expects.
