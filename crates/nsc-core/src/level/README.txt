level/ — support and resistance, as a shared type
================================================


WHAT THIS FOLDER IS FOR

  Saying what a level IS, so that every other part of the program means the
  same thing by the word.

  A level here is a band of price the market has turned at before, plus the
  plain facts about it: which timeframe it came from, how many touches, when
  the first and last one were, and the first moment you could have known
  about it.

  Finding levels is not done here. That is nsc-ta::levels. This folder only
  holds the shape.


THE FILES

  mod.rs      The front door. Module docs, and what the outside world sees.

  band.rs     Band. A bottom and a top, and the four questions you ask of
              one: how thick, where is the middle, is this price inside, and
              how far away is this price.

  zone.rs     Level. A Band, plus the timeframe it was found on, the touch
              count, the two touch times, and confirmed_at.

  tests.rs    Nine tests. Five of them are about what gets REFUSED.

  README.txt  This file.


WHY IT IS A BAND AND NOT A PRICE

  Price does not turn at an exact number. It turns somewhere in a small area.

  And if a level were one number, every single comparison against it would
  need its own tolerance bolted on — which is the fixed-pip trap again. A
  tolerance that works on EURUSD is meaningless on gold.

  The thickness is decided once per timeframe and never stretched. The band
  slides up and down to catch the most touches; it does not grow to swallow
  whatever is nearby. A band that grows ends up wide enough that every price
  is at every level, and then the level check always says yes.


ONE TYPE, NOT A SUPPORT AND A RESISTANCE

  When a support breaks and later holds price down, that is the same line on
  the chart doing a different job. You would not rub it out and draw a new
  one.

  So which side price is on is not part of what a level is. It is just where
  price happens to be today, and Band::distance_to keeps its sign so you can
  ask.


WHAT IS DELIBERATELY MISSING

  There is no strength score. There is no exhausted flag. Nothing here says
  whether the level will hold or break.

  That is a judgement and it needs much more than the level: the trend, the
  timeframe, the candle printing into it, how price arrived, what the weekly
  is doing. It belongs in nsc-strategy, where it can be switched on and off
  and measured against real signals.

  Keeping it out is what makes this type reusable. The chart-reading code
  reports what is there; the rules decide what it is worth.

  The one judgement that IS settled: more touches means a level matters more,
  not less. That belief lives in config/strategy.toml as strong_touches, not
  in this code, so it can be measured once there are 200 labelled signals
  rather than argued about. See docs/worksheets/levels.md.


THE CHECK THAT MATTERS

  Level::new refuses a level whose confirmed_at is not later than its last
  touch.

  A touch is a swing point. A swing at candle 100 is not knowable until a few
  candles later. So a level finished by that touch cannot be known at candle
  100 either.

  Get this wrong and nothing breaks. The backtest just quietly gets better
  than anything you could have traded. That is why it is a hard refusal here
  rather than a rule to remember.

  Call is_known_at before using a level for anything.


WHERE THIS CAME FROM

  docs/worksheets/levels.md, off four annotated charts, and
  docs/diagrams/level-touches.html for the picture that settled what repeated
  touches do.
