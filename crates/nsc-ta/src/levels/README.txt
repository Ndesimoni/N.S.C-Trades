levels/ — finding support and resistance
========================================


WHAT THIS FOLDER IS FOR

  Turning swing points into the horizontal lines you would draw on a chart.

  Candles and swings go in. Bands of price that have turned the market before
  come out, each with its touch count and its dates.


THE FILES

  mod.rs        The front door. Module docs, and what the outside world sees.

  grouping.rs   The one piece of thinking: where do you put a band of fixed
                thickness so it catches the most swing points?

  finder.rs     One timeframe. Works out the thickness from ATR, throws away
                swings that are too old or not confirmed yet, groups what is
                left, and turns each group into a Level.

  across.rs     Every timeframe at once, and which levels get a line.
                Builds weekly, daily and 4-hour candles from one file through
                the aggregator, finds levels on each, then works out which of
                them are hidden behind a bigger one. See below.

  tests/        Twenty-eight tests. Read tests/guards.rs first.

  README.txt    This file.


HOW IT WORKS

      candles ──► ATR ──────► band thickness   half a normal candle
                                    │
      swings  ──► too old? ────► grouping ──► Level      one per group
                  not confirmed?

  The thickness is decided ONCE for the whole timeframe. Every band on that
  timeframe is the same width.


THE DECISION INSIDE THE GROUPING

  The band SLIDES. It does not STRETCH.

  Given a thickness, the finder tries the band at every useful position and
  keeps the position that catches the most swing points. It never widens the
  band to reach one more.

  Why that matters: a band that stretches to swallow whatever is near it
  keeps growing. Eventually it is wide enough to hold half the chart, every
  price is inside every level, and the level check always says yes. A check
  that always says yes is not a check.

  This is also how these lines get drawn by hand — see docs/worksheets/
  levels.md, which came off four annotated charts.


TIE-BREAKS, AND WHY THEY ARE WRITTEN DOWN

  Two positions can catch the same number of swings. Then:

    1. the tighter group wins — the one whose swings sit closest together
    2. if they are equally tight, the lower one wins

  Rule 2 is arbitrary. It is here so the answer cannot depend on the order
  the swings happened to arrive in. Without it the same history could produce
  different levels on different runs, and a backtest you cannot repeat tells
  you nothing.


HIGHS AND LOWS GO IN THE SAME POT

  A price that capped a rally in March and held a fall in June is ONE level
  tested twice. Not a resistance and a support that happen to share a price.

  So swing highs and swing lows are grouped together, and Level has no side.


WHAT THIS FOLDER DOES NOT DECIDE

  Whether a level will hold or break.

  That needs the trend, the timeframe, the candle printing into it, how price
  arrived and what the weekly is doing. It is a judgement, it belongs in
  nsc-strategy, and it is set in config/strategy.toml — including
  strong_touches, which says how many touches counts as a lot.

  This folder reports the touch count. It has no opinion about it.


THE TWO WAYS THIS COULD QUIETLY LIE

  1. A swing that has not confirmed yet.

     A swing high at candle 100 is not knowable until a few candles later.
     Levels built from one are levels you could not have drawn at the time,
     and they make a backtest look better than anything you could trade.

     Guarded twice. The finder drops any swing not confirmed by the last
     candle, and Level::new refuses a level whose confirmed_at is not later
     than its last touch.

  2. A candle that has not finished forming.

     Its high and low have not happened yet. find_levels refuses one rather
     than drawing a band around a price that may never print.


SETTINGS IT READS

  From [levels] in config/ta.toml:

    band_atr_multiple   how thick, as a fraction of a normal candle
    min_touches         below this, no level is reported at all
    max_age_bars        how far back to look, counted in candles
    absorb_when         when a smaller level loses its line to a bigger one

  max_age_bars is counted in CANDLES, not days. Counting days would let a
  weekend or a market holiday quietly shorten how far back the bot looks.

  See docs/worksheets/levels.md for where these came from, and
  docs/diagrams/level-touches.html for the picture that settled the argument
  about what repeated touches do.


WHEN A LEVEL DOES NOT GET A LINE

  There is ONE set of levels, not a set per chart. A daily level is still a
  daily level when you are looking at the 4-hour, and that is exactly why it
  matters there.

  So all three timeframes get drawn on every chart, each in its own colour:

      black    weekly
      blue     daily
      yellow   4-hour

  Those colours are the trader's, from docs/worksheets/levels.md. They are a
  specification, not a design choice.


  THE RULE. When two levels land on the same price, only the bigger timeframe
  gets a line.

      a daily on top of a weekly     draw the weekly, hide the daily
      a 4-hour on top of a daily     draw the daily, hide the 4-hour

  Why: the weekly line is the strongest thing on the chart. Look at it and you
  already know the price matters. A second line on the same spot says nothing
  new, and three lines in one place is how a chart becomes unreadable.


  NOTHING IS DELETED. A hidden level keeps its band, its touches, its dates
  and its own timeframe tag. It is marked, not dropped.

  That matters more than the drawing does. Two timeframes turning at one price
  is CONFLUENCE, and confluence is the reason the price is worth trading.
  Delete the daily level and the strategy loses the reason the weekly one is
  good.

  Ask Level::is_drawn() before putting one on a chart. Ask Level::absorbed_by()
  to find out what is sitting over it.


  ONLY A DRAWN LEVEL CAN HIDE ANOTHER. If a daily is already hidden behind a
  weekly, it cannot then hide a 4-hour level of its own.

  Without that, a price could end up with no line at all — hidden behind
  something that is itself hidden. There is a test called
  a_hidden_level_cannot_hide_another.


  THE BIGGER TIMEFRAME ALWAYS WINS, and that is enforced rather than
  remembered. Level::absorbed_into refuses a timeframe that is not bigger, so
  a 4-hour level cannot swallow a weekly one however the calling code is
  written. Worth having: the rule got stated backwards twice while it was
  being agreed.


  WHAT COUNTS AS THE SAME PRICE is the absorb_when setting in ta.toml:

      bands_overlap   the bands touch at all               (the trader's answer)
      centre_inside   the smaller level's middle is inside the bigger band

  A weekly band is thick — half a normal WEEKLY candle — so bands_overlap will
  swallow a fair number of daily levels. If that hides more than you want,
  centre_inside is the looser reading. One setting, no rewrite.
