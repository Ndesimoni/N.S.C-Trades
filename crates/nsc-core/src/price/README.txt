price/ — prices, and how far apart two prices are
=================================================


WHAT THIS FOLDER IS FOR

  Stopping you mixing up a price with a distance.

  A price is a point on the chart. 1.0850.
  A distance is a gap. 0.0030.

  They are different things. But if both are just numbers, the computer
  cannot tell. So this would run:

      stop = entry - atr_multiple        1.0850 - 0.3

  You meant "put the stop 0.3 of a candle below entry". You got 1.0820.
  That looks like a real price. Nothing warns you. Every stop you place is
  in the wrong spot, and you never find out.

  This folder makes that a compiler error instead.


THE FILES

  mod.rs        The front door.
                Says what the folder is, and what the rest of the program is
                allowed to use. Everything else in here is hidden.

  point.rs      Price. A point on the chart.
                Uses Decimal instead of a normal decimal number. Normal
                decimals in computers are slightly wrong: 0.1 + 0.2 does not
                quite equal 0.3. That is fine for most things. It is not fine
                when you are asking "did price touch my level?", because the
                answer comes back "missed it by 0.0000000001".

  distance.rs   Three ways of measuring the same gap.

                    PriceDistance   0.0050    the raw gap
                    Pips            50        counted in pips
                    AtrMultiple     1.8       counted in normal candles

                Same gap. Three ways of saying it. They are separate types so
                you cannot use one where you meant another.

  ops.rs        What you can add and subtract.
                Also what you CANNOT — and that part is the point. See below.

  tests.rs      Five tests that prove the above actually works.


HOW THEY FIT TOGETHER

      point.rs  ─┐
                 ├─►  ops.rs      needs both, to define + and -
      distance.rs┘

      distance.rs ─► error.rs     because converting can fail

      mod.rs      ─► lets the outside world see the four types


WHAT ADDS UP AND WHAT DOES NOT

      price    - price      =  a gap
      price    + a gap      =  a price      (move up from here)
      price    - a gap      =  a price      (move down from here)

      price    + price      =  will not compile

  That last line is not missing by accident. Adding two prices together means
  nothing. What is 1.0850 plus 1.0900? Nothing useful.

  So we simply never wrote that code. The compiler refuses it. The missing
  code is the safety.


WHY YOU CANNOT JUST CONVERT PIPS AUTOMATICALLY

  To turn a gap into pips, you need to know the instrument.
  A pip is 0.0001 on EURUSD. It is 0.01 on USDJPY. Different sizes.

  To turn a gap into "normal candles", you need today's ATR.
  A normal candle is a different size every day.

  So you have to hand in that information. There is no automatic version,
  because there is no correct answer without it.


WHO USES THIS

  Everything does.

  symbol/ wraps the pip conversion so you do not have to remember the pip
  size each time.

  nsc-ta measures every level and swing using these types.

  nsc-strategy compares distances in normal candles, never in pips. A pip
  setting that works on EURUSD is meaningless on gold.
