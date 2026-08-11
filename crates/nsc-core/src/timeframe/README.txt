timeframe/ — which candle does this moment belong to
====================================================


WHAT THIS FOLDER IS FOR

  Two questions.

  One is easy:
      It is 14:37. Which 15-minute candle is that?      Answer: 14:30

  One is hard:
      When does the trading day start?                  Answer: 5pm New York

  The hard one decides more than it sounds like.

  Where the day starts decides where every daily candle opens and closes.
  That decides the daily high and low. Which decides every daily level, every
  trendline you draw off a daily swing, every Fibonacci you pull.

  Get it wrong and the bot draws levels at prices your chart never showed.


THE FILES

  mod.rs        The front door.
                Says what the folder does, and lets the rest of the program
                see two things: Timeframe and DayBoundary.

  kind.rs       The list of timeframes: M15, M30, H1, H4, D1, W1.
                The names match config/app.toml exactly, so they can be read
                straight out of your settings.
                Also says how many minutes each one lasts — except daily and
                weekly, which say "no answer". See below for why.

  boundary.rs   Where the day and the week start.
                Stores "5pm, New York" — the place, not a fixed offset from
                UTC. That is the whole reason this file exists. See below.

  snap.rs       Given any moment, which candle is it in, and when does that
                candle end.
                Counts from the daily close, not from midnight.

  tests.rs      Eleven tests.
                Two of them matter more than the rest: one checks summer, one
                checks winter. If either ever fails, every daily level in the
                system has quietly moved.


HOW THEY FIT TOGETHER

      kind.rs     ─┐
                   ├─►  snap.rs    needs to know how long a candle is,
      boundary.rs ─┘               AND where the day starts

      boundary.rs ─►  chrono-tz    the timezone rules

      mod.rs      ─►  lets the outside world see Timeframe and DayBoundary


WHY WE STORE "NEW YORK" AND NOT "21:00 UTC"

  New York changes its clocks twice a year.

      Summer:   5pm New York  =  21:00 UTC
      Winter:   5pm New York  =  22:00 UTC

  Same time on the wall. Different time in UTC.

  If we stored 21:00 UTC, then for about five months of the year every daily
  candle would start an hour late. Each one would swallow an hour that
  belongs to the day before. Your daily high might come from yesterday.

  Nothing would error. The numbers would just stop matching your chart.


WHY DAILY AND WEEKLY SAY "NO ANSWER"

  M15 is always 15 minutes. H4 is always 240 minutes.

  A day is not always 24 hours. On the two days a year the clocks change, it
  is 23 or 25.

  A week is not 7 days. The market shuts at the weekend, so a trading week is
  5 days.

  So there is no honest number to give. Saying "no answer" forces anything
  that wants a daily boundary to go and ask boundary.rs, which knows about
  all of that. It is the computer refusing to let you take a shortcut that is
  wrong twice a year.


EVERY TIMEFRAME STARTS FROM THE DAILY CLOSE

  Not just the daily candle. 4-hour candles too.

  The day opens at 21:00 UTC in summer, so the 4-hour candles run 21:00,
  01:00, 05:00, and so on. Not 20:00 or midnight.

  Why: six 4-hour candles have to make exactly one daily candle. Otherwise
  the program can never tell when a daily candle has finished.

  WORTH CHECKING ONCE: different platforms line their 4-hour candles up
  differently. Compare one against your own chart. Better to find out now
  than after you have traded a level off it.


SUNDAY EVENING IS ALREADY MONDAY

  The market opens Sunday at 5pm New York.

  So 6pm on a Sunday is inside the session that ends Monday afternoon. The
  one you would call Monday's session — even though the calendar still says
  Sunday.

  That is why "no trading on Mondays" in config/strategy.toml has to mean the
  trading day, not the calendar day.


WHY THIS FOLDER NEVER OPENS THE CONFIG FILE

  nsc-core is not allowed to read files or check the clock. That rule is what
  lets the backtester and the live bot run the exact same code.

  So the daily close is handed IN, already built, by whoever loaded
  app.toml. This folder just does the maths on what it is given.

  That also makes it easy to test. Hand it a made-up boundary and check the
  answers. No config file involved.
