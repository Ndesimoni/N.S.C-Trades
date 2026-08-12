candles/ — spotting candlestick patterns
========================================


WHAT THIS FOLDER IS FOR

  Eight shapes, and they are the ones the trader named:

      pin bar        a long wick with a small body at the far end
      engulfing      one body swallowing the one before it
      doji           open and close in nearly the same place
      belt-hold      a long candle opening at one extreme, no wick there
      tweezers       two candles reaching the same high, or the same low
      inside bar     a candle wholly inside the one before it
      star           a push, a stall, then a push back the other way

  Hammer and inverted hammer are not separate detectors. A hammer IS a bullish
  pin bar. Textbook separates them by what came before — a hammer follows a
  downtrend — and that is context, not shape.

  A pattern in the code nobody would act on is noise in every backtest it
  appears in, so nothing gets added that was not asked for.


THE LIST IS NOT FINISHED, AND IS NOT MEANT TO BE

  The trader reads more shapes than he can name. The plan is that training on
  his labelled trades picks them up and names them, rather than anyone
  guessing now.

  That is why every sighting carries the MEASUREMENTS of the candle it was
  found on and not just a name. A model cannot learn a shape from the word
  "engulfing"; it learns from proportions.

  There is a gap in that, and it is written down in docs/worksheets/candles.md
  rather than hidden here: those measurements are only recorded for candles
  that matched one of the eight. A shape with no name yet produces nothing at
  all, so there is nothing for a model to find it in. Storing every candle's
  proportions is what fixes it, and it has to be done BEFORE the data is
  collected rather than after.


IT NEVER LOOKS LEFT

  Knowing a candle engulfed the one before it does not tell you to buy.

  Whether it matters needs the level it happened at, the trend it happened in,
  and the timeframe it happened on. All three are nsc-strategy.

  Textbook descriptions bolt the context onto the pattern — "a hammer after a
  downtrend". That half is the rules'. The same candle in open space is still
  a hammer; it is simply not a trade.


TWO YARDSTICKS, FOR TWO DIFFERENT QUESTIONS

  SHAPE is measured as shares of the candle's own height. A body that is a
  fifth of its candle is a fifth on EURUSD and a fifth on gold, so no ATR and
  no pip size come into it.

  SIZE — whether a candle is big at all — is measured in ATR.

  Only two settings need ATR, and both are about size rather than shape: the
  belt-hold has to be a long candle, and the tweezer tolerance is about two
  prices being near enough to call the same. Without ATR those two go quiet
  rather than guess.


ONE CANDLE CAN BE SEVERAL THINGS

  A candle with no body and a long tail is a pin bar AND a doji. Both are
  reported.

  They are two true statements about one candle. Deciding which matters needs
  the level and the trend, and that is not this folder's question.


THE NUMBERS ARE TEXTBOOK, NOT THE TRADER'S

  Standard measurements, taken as defaults on 12 Aug 2026 so the detectors
  could be built. They are marked as borrowed in config/ta.toml and in
  docs/worksheets/candles.md.

  What replaces them is a pair of charts: one setup taken and one passed that
  looked the same. Until then nobody should read 2.0 and assume somebody chose
  it.


THE FILES

  mod.rs        The front door.

  pin_bar.rs      One shape each. Every one takes the candles it needs and
  doji.rs         gives back a sighting or nothing. None of them looks at
  engulfing.rs    anything but the candles it was handed.
  belt_hold.rs
  tweezers.rs     inside_bar.rs needs no settings at all — either the range is
  inside_bar.rs   inside or it is not, and there is no threshold to get wrong.
  star.rs
                  star.rs is the only three-candle shape in the project.

  finder.rs     Asks all seven about the newest candle. Takes the newest and
                the two before it — no shape here is more than three candles.

  series.rs     A whole history at once, for the backtester. Works ATR out as
                it goes, so each candle is judged against how big a normal
                candle was AT THE TIME. Judging a quiet week by this week's
                volatility would find shapes that were not there.

  tests/        Thirty-two tests. Read guards.rs first.

  README.txt    This file.


THE TWEEZER AND THE SWING FINDER

  A tweezer top is two candles with the same high, and that is the exact shape
  swing detection used to throw away — the old finder refused ties, because
  neither candle strictly beat the other.

  The rewritten finder tracks a running extreme instead, so a tweezer top can
  be a swing high as well. The two no longer disagree.


SETTINGS IT READS

  From [candles] in config/ta.toml. Thirteen numbers: eleven are shares of a
  candle, two are in ATR, and the inside bar needs none.

  See docs/worksheets/candles.md for where they came from, and
  docs/diagrams/candles.html for every shape drawn with the measurement that
  makes it one.
