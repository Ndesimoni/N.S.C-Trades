candles/ — spotting candlestick patterns
========================================


WHAT THIS FOLDER IS FOR

  Six shapes, and they are the six the trader actually uses:

      pin bar        a long wick with a small body at the far end
      engulfing      one body swallowing the one before it
      doji           open and close in nearly the same place
      belt-hold      a long candle opening at one extreme, no wick there
      tweezers       two candles reaching the same high, or the same low

  Hammer and inverted hammer are not separate detectors. A hammer IS a bullish
  pin bar. Textbook separates them by what came before — a hammer follows a
  downtrend — and that is context, not shape.

  A pattern in the code nobody would act on is noise in every backtest it
  appears in, so this list is short on purpose.


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

  pin_bar.rs    One shape each. Every one takes a candle and settings and
  doji.rs       gives back a sighting or nothing. None of them looks at
  engulfing.rs  anything but the candles it was handed.
  belt_hold.rs
  tweezers.rs

  finder.rs     Asks all five about the newest candle. Takes the newest and
                the one before it — no shape here is more than two candles.

  series.rs     A whole history at once, for the backtester. Works ATR out as
                it goes, so each candle is judged against how big a normal
                candle was AT THE TIME. Judging a quiet week by this week's
                volatility would find shapes that were not there.

  tests/        Twenty-four tests. Read guards.rs first.

  inside_bar.rs Stubs from the original scaffolding. NEITHER IS ON THE
  star.rs       TRADER'S LIST. They are left untouched pending a decision
                rather than deleted or quietly built — deleting a stub is
                cheap, and finding out in six months that the bot never looked
                for a pattern he trades is not.

  README.txt    This file.


THE TWEEZER AND THE SWING FINDER

  A tweezer top is two candles with the same high, and that is the exact shape
  swing detection used to throw away — the old finder refused ties, because
  neither candle strictly beat the other.

  The rewritten finder tracks a running extreme instead, so a tweezer top can
  be a swing high as well. The two no longer disagree.


SETTINGS IT READS

  From [candles] in config/ta.toml. Ten numbers, eight of them shares of a
  candle and two in ATR.
