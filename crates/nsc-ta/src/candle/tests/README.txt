tests/ — what a candle is, and what it gets called
==================================================


THE FILES

  mod.rs      The front door.

  real.rs     EIGHTEEN REAL GOLD CANDLES, one per shape -- the clearest
              example of each found in 4,165 four-hour candles between
              October 2023 and August 2026. Every one of them printed.

  shape.rs    The four measurements. Seven tests.

  naming.rs   Which name each shape gets. Five tests.

  README.txt  This file.


WHY THE CANDLES ARE REAL ONES

  A made-up candle proves the code does what you already thought. A real one
  tells you what the thresholds actually do.

  Both bugs found while building the naming were found this way, and neither
  would have shown up against invented candles:

    THE DRAGONFLY AND THE GRAVESTONE both came back as plain dojis. Their
    short end is 0.095 and 0.08, and "no wick" was set at 0.05 -- a number
    that is right for the end of a marubozu and far too tight beside a tail of
    0.90. That is what `stub` is for.

    THE CLEAREST CLOSING MARUBOZU read as a full marubozu, a shape found seven
    times in three years. "No wick" came down to 0.02 and the two separated.


THE COUNTS ARE CARRIED WITH THE CANDLES

  Because how often a shape turns up is part of what it means.

  A spinning top appears 408 times in 4,165 candles -- one in ten. A shape
  that common cannot carry a decision on its own.

  A high wave appeared exactly ONCE in three years. That is the opposite
  danger: a rule nothing ever triggers looks like a rule that never misfires.

  Anyone loosening a threshold has to walk past these numbers to do it.


WHERE THEY DIFFER FROM THE TAXONOMY, IT IS ON PURPOSE

  The taxonomy in docs/diagrams/candle-taxonomy.html names twenty-two things.
  This crate names twelve, and the difference is always the same reason: two
  textbook names, one shape.

  Standard doji, rickshaw man and long-legged doji all come back as one name,
  because nothing about the CANDLE separates them.

  Hammer and takuri come back as one name -- and the taxonomy's clearest
  example of each is literally the same candle, 17 October 2025.

  Long bull and bullish belt-hold, likewise -- 29 January 2026, one candle.
