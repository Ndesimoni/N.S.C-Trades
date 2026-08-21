pattern/ — what a RUN of candles does
=====================================


WHAT THIS FOLDER IS FOR

  One candle at a time is candle/. This is two or three of them together.

  IT DESCRIBES, IT NEVER DECIDES. "This is a bullish engulfing" belongs here.
  "This is a buy" belongs in nsc-strategy.


THE FILES

  mod.rs      The front door.

  named.rs    Pattern -- what there is to find.

  rules.rs    The thresholds, read out of config/patterns.toml.

  body.rs     A candle's body as PRICES, not as a share. Patterns compare
              bodies on the chart; two candles of different heights can have
              the same body share and not overlap at all.

  two.rs      Engulfing, harami, tweezers, piercing line, dark cloud cover.

  three.rs    The star with the abandoned baby inside it, and the march --
              three white soldiers and three black crows.

  finding.rs  The one way in, and the order they are tested.

  tests/      Seventeen tests, on runs that actually printed.

  README.txt  This file.


THE FUNCTION IS CALLED ending_at, AND THE NAME IS THE SAFETY

  You hand over the candles up to AND INCLUDING the one being judged, and it
  looks backwards from the end.

  There is no argument it could use to see forwards. So the rule that matters
  most in this project -- never use price the market had not printed yet -- is
  not a discipline here, it is the shape of the function.

  `normal` is how big a normal candle was AT THAT MOMENT, not today. Hand over
  today's and a run from 2023 gets judged against a market that had not
  happened yet.


ONE PATTERN PER CANDLE, LONGEST FIRST

  Three candles beat two. A star whose last two candles also engulf is a star.

  Within two candles, the strictest wins: engulfing, then piercing, then
  harami, then tweezers.

  THIS IS NOT THEORY. The 7 June 2024 gold candles are the clearest bearish
  engulfing in three years AND the clearest tweezer top. One run, two true
  statements. Reporting both would let a backtest count one setup twice.


THE ENGULFING RULE THAT IS NOT IN THE TEXTBOOK

  The textbook says the second body must cover the first. That rule assumes a
  market that GAPS -- the second candle can open away from where the first
  closed, so covering it is a real thing to ask.

  SPOT FOREX DOES NOT GAP. Measured on the clearest bullish engulfing in three
  years: first close 2030.36, second open 2030.36. Identical. So one end of
  the covering is free, every single time, and "engulfing" collapses into "did
  it close past the first candle's open".

  Left alone that found 54 engulfings in 300 gold candles -- one in six. That
  is not a reversal signal, it is a description of an ordinary afternoon.

  So min_second_of_first asks the second body to be half again the first. The
  clearest real example is 2.5x. It brought 54 down to 39.


THE GAP IS REPORTED, NOT REQUIRED

  The classic Japanese star gaps away from the candle before it and the one
  after. Insist on that and the pattern can only form at the Sunday open.

  So require_gap is false, and whether the gap was there comes back on the
  pattern itself as `abandoned`.

  AN ABANDONED BABY IS THE STRICT CASE, not a separate detector: a star whose
  middle candle's whole RANGE -- wicks included, not just its body -- cleared
  both neighbours.

  It is the best-evidenced pattern on the whole list at around 70%, AND it
  will effectively never fire on his instruments. Both are true at once, and
  that is why it is a flag rather than a detector of its own. None was found
  in 300 candles of gold, and none can be.


A STAR AND A MARCH CAN NEVER BOTH BE TRUE

  A star TURNS -- its first and third candles disagree. A march does not --
  all three go the same way. So the order between them decides nothing and
  neither can steal the other's candles.


THE MARCH DROPS A TEST THE TEXTBOOK KEEPS

  The textbook asks each of the three to open inside the last one's body.

  ON SPOT FOREX THAT ASKS NOTHING. A candle opens exactly ON the last close,
  so it sits on the boundary and passes every single time. Kept, it would look
  like a rule and do no work.

  The wick AGAINST the move does the work instead. A long one means they were
  pushed back and came again -- a fight, not a march.


HOW OFTEN THESE TURN UP, MEASURED

  Gold, 2 January to 20 August 2026. 3,740 hourly, 1,137 four-hour and 164
  daily candles:

                          1h     4h     1d
      bearish engulfing  269     71      7
      bullish engulfing  254     58      6
      bearish harami     205     62      8
      tweezer top        200     74      9
      bullish harami     194     52      7
      tweezer bottom     153     59      7
      dark cloud cover   101     27      3
      piercing line       98     29     10
      morning star        52     13      3
      evening star        50     10      6
      the march           32     12      0
      abandoned baby       0      0      0

  FOUR CANDLES IN TEN end a pattern -- 42% hourly, 40% four-hour, 40% daily.
  Three charts, eight months, the same answer.


TWO NOUGHTS, AND THEY MEAN DIFFERENT THINGS

  THE ABANDONED BABY IS A REAL NOUGHT. It needs a gap and spot forex has none.
  It cannot appear, so nought is the correct answer and always will be.

  THE DAILY MARCH IS A KNIFE EDGE, AND THAT IS WORSE. There were 35 runs of
  three same-way candles on the daily this year. The five closest all peak at
  a weakest body of 0.48 or 0.49 against a threshold of 0.50.

  Drop min_body to 0.45 and five appear. THAT IS EXACTLY WHY IT WAS NOT
  DROPPED -- loosening a rule until a shape turns up is fitting the rule to
  the wish, the same reason `high wave` is left at one example in three years.

  A pattern whose count swings from 0 to 5 on a hundredth of a threshold is
  not measuring anything on that timeframe. Say so rather than tune it.

  Which is the same lesson the single candles taught: a pattern is a
  description, not a signal. What makes one worth anything is the level it
  printed at, and that is not this crate's question.
