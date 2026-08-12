aggregate/ — building bigger candles from smaller ones
=====================================================


WHAT THIS FOLDER IS FOR

  Turning 15-minute candles into 1-hour, 4-hour, daily and weekly ones.


WHY BUILD THEM RATHER THAN ASK THE BROKER

  Control over when the day ends.

  The daily close — 5pm New York by convention, set in config/app.toml —
  decides where every daily level sits. Brokers disagree with each other about
  it. A level that does not match the one on your own chart destroys your
  trust in the bot faster than a losing trade does.

  Building them also means one answer across every instrument and every feed,
  instead of whatever each one happened to send.


THE RULE THIS FOLDER EXISTS TO KEEP

  A PART-FORMED BIGGER CANDLE IS NEVER HANDED OUT AS FINISHED.

  A 4-hour candle made from three 15-minute candles is not a 4-hour candle.
  Its high and low have not finished happening. Signal on it and you are using
  prices the market has not printed — and the backtest gets BETTER rather than
  broken, which is what makes it dangerous.


FINISHED MEANS THE NEXT ONE STARTED

  Not "four hours have passed".

  A bigger candle is sealed only when a smaller candle belonging to the NEXT
  bucket arrives. The market can be shut, a feed can be late, a session can
  end early on Christmas Eve — and a candle that is merely expected is not a
  candle that happened.

  Two consequences, both correct:

    Four 15-minute candles fill an hour but do not finish it. The fifth one
    does.

    The last bucket of any history comes back missing. More candles may still
    be coming and nothing here can know they are not. A backtest that wants
    that last candle is a backtest reading the future.


THE FILES

  mod.rs        The front door.

  bucket.rs     One bigger candle under construction. Keeps the first open,
                the highest high, the lowest low and the newest close. It
                knows its own start, so an arriving candle is CHECKED against
                the bucket it belongs to rather than assumed into it.

  builder.rs    One candle at a time, the way the live bot works. Also hands
                back the candle currently forming, marked incomplete — for
                drawing a live chart and for nothing else.

  series.rs     A whole history at once, for the backtester. Same builder, so
                the two cannot drift apart.

  tests/        Seventeen tests. The one that matters most is
                guards.rs::an_hour_is_not_finished_until_the_next_one_starts.

  README.txt    This file.


TWO THINGS IT REFUSES

  CANDLES OUT OF ORDER. Arriving backwards, the bucket about to be sealed
  would be sealed by a candle from before it, and the history handed back
  would run backwards. Nothing downstream checks for that, so it is checked
  here. The same candle twice is refused for the same reason.

  BUILDING DOWNWARDS. Asking for 15-minute candles out of 4-hour ones is not
  a hard job, it is a meaningless one — and it would quietly produce a chart
  that looks perfectly fine. So the aggregator is told what it is being fed
  and refuses anything that is not smaller than what it is asked to build.

  Both were found by reading the code back against what it claimed to do. The
  tests were green with neither of them in.


A GAP IN THE DATA IS NOT A PROBLEM

  A weekend, a holiday or a dead feed leaves a hole. The bucket before it is
  still finished by the first candle that arrives afterwards, and the one
  after starts clean.

  Nothing is invented to fill the hole. An empty hour has no candle, which is
  the truth about it.


WHY EVERYTHING ANCHORS TO THE DAILY CLOSE

  So candles nest. Six 4-hour candles have to make exactly one daily candle,
  or there is no way to say "this daily candle is now finished".

  Anchoring 4-hour candles to midnight UTC instead would start the daily
  candle in the middle of one, and the two would drift apart twice a year when
  New York changes its clocks.

  The maths for that lives in nsc-core::timeframe, in one place, and this
  folder asks it rather than working it out again.


WORTH CHECKING ONCE, BY EYE

  Compare the 4-hour candles this produces against your own platform. Charting
  packages anchor them differently, and if yours disagrees you want to know
  now rather than after you have trusted a level built from them.
