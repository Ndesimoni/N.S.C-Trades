swing/ — swing highs and lows
=============================


WHAT THIS FOLDER IS FOR

  A swing high is a peak. Price came up to it and turned back down.
  A swing low is a trough. Price came down to it and turned back up.

  That is all. But almost everything else in this project is built from them:

      support and resistance levels
      trendlines
      Fibonacci anchors
      trend direction (higher highs, lower lows)
      chart patterns

  Nothing else in this codebase affects as much.


THE FILES

  mod.rs        The front door. Says what is in here and lets the outside
                world see two things: SwingKind and Swing.

  kind.rs       SwingKind. High or Low, and a few small helpers.

  point.rs      The Swing itself. Where it sits, when it became knowable,
                and what price it is at.

  tests.rs      Five tests.

  README.txt    This file.


HOW THEY FIT TOGETHER

      kind.rs  ─►  point.rs        a Swing has a kind

      point.rs ─►  price/          the price it sits at
      point.rs ─►  error.rs        refuses a swing known too early

      mod.rs   ─►  lets the outside world see SwingKind and Swing


THE THING TO UNDERSTAND: THERE ARE TWO TIMES

  A Swing carries two timestamps, and the difference between them is the
  whole point of this folder.

      bar_time       where the swing sits on the chart

      confirmed_at   the first moment you could have KNOWN it was a swing

  They are never the same.


WHY THEY ARE NEVER THE SAME

  Scroll back over any chart. The swing highs are obvious. Right there.

  That is the trap.

  At the moment candle 100 printed, nobody knew it was a high. Price could
  have carried straight on up. It only became a high once price turned away
  and a few more candles printed.

  How many candles? That is the lookback setting in config/ta.toml. With a
  lookback of 3, a swing at candle 100 is confirmed at candle 103.

  So bar_time is candle 100. confirmed_at is candle 103.


WHY THIS MATTERS SO MUCH

  Say you draw a support level using a swing low, and you use it from
  bar_time instead of confirmed_at.

  Your backtest now buys at a level three candles before that level existed.
  Every time. It will look excellent.

  Nothing errors. Nothing warns you. The trades just look better than they
  could ever have been in real life, and you find out with real money.

  This is the single easiest way to build a backtest you cannot trade.


THE FUNCTION THAT KEEPS IT HONEST

      swing.is_known_at(now)

  Call it before using a swing for ANYTHING.

  It answers one question: at this moment, could I have known about this
  swing yet?

  There is a test called a_swing_is_invisible_until_it_is_confirmed that
  spells out the behaviour.


THE CHECK IN new()

  Swing::new refuses any swing that claims to be known BEFORE or AT the
  candle it sits on.

  Not just before. At, as well.

  You cannot tell a peak is a peak while it is still printing — price could
  keep going. So confirmed_at must be strictly later than bar_time.

  If that check ever fires, whatever built the swing has a lookahead bug. It
  is worth knowing that lookahead bugs have no other symptom. They do not
  crash, they do not warn, and they make your results better. This check is
  one of the few places one can be caught.


LATER

  nsc-backtest::guards will also watch for this while a backtest runs, and
  kill the whole run if a swing is used before it was confirmed.

  A bad number with a warning attached still gets read and acted on weeks
  later. Killing the run is the only thing that works.
