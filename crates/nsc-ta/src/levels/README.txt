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

  finder.rs     The whole job. Works out the thickness from ATR, throws away
                swings that are too old or not confirmed yet, groups what is
                left, and turns each group into a Level.

  tests/        Twenty-two tests. Read tests/guards.rs first.

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

  max_age_bars is counted in CANDLES, not days. Counting days would let a
  weekend or a market holiday quietly shorten how far back the bot looks.
