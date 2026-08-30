nsc-strategy — the rules, applied in one place
==============================================


WHAT THIS CRATE IS FOR

  ONE RULE, AND IT IS A SENTENCE: a shape he trades, sitting at a level he
  drew.

  Three "strategies" were described on 25 August 2026. They collapsed into one
  rule with four kinds of shape, because the PLACE test turned out to be
  identical for all of them.

  FOUR SHAPES SINCE 29 AUGUST: his push-then-pin, the engulfing, the harami
  and marching. Harami is measured from its BIG first candle and marching from
  the candle its run started on -- not from the last one, which would report
  the setup a whole shape away from the level that caused it.

  AND THREE TIERS, the same day:

    Inside   in the zone          RED. The only one that asks him to act.
    Close    within half a band   amber. It almost touched and did not.
    Bold     no zone near it      plain. 2x a normal candle or it is silence.

  A LEVEL BEATS SIZE, ALWAYS. A shape at a zone is a setup whatever its reach;
  a shape away from every zone is only ever a remark. Testing size first would
  put a big candle in open water above a modest one sitting exactly where he
  was watching.

  The specification is docs/worksheets/strategies.md. When it and this code
  disagree, IT WINS -- the worksheet is the spec and the code is the guess.


THE FILES

  lib.rs       The front door.
  shape.rs     Which shapes count, where each is measured from, and how big
               each one is.
  standing.rs  THE THREE TIERS -- in the zone, extremely close, or bold and
               away from every one of them.
  place.rs     THE TEST -- is it at the level.
  rules.rs     The settings, out of config/strategy.toml.
  finding.rs   The one way in.
  reasons.rs   The one sentence that explains it.
  tests/       Eighteen tests, on the two gold candles off his screenshot.
  README.txt   This file.


IT CANNOT REACH ANYTHING

  No tokio, no reqwest, no sqlx and no clock in Cargo.toml, so nothing here
  CAN fetch. The compiler refuses.

  That is what lets the backtester and the live bot run these exact rules and
  agree. THERE IS NO "IF WE ARE BACKTESTING" IN HERE AND THERE NEVER MAY BE --
  the moment there is, the backtest is testing something else, and the mismatch
  makes results look BETTER rather than broken.


WHY THE LEVEL IS THE WHOLE POINT

  nsc-bull and nsc-bear were measured across five pairs and five timeframes on
  22 August. Followed for ten candles they reached +1 normal candle before -1
  in 29 OF 75 -- 38%, where a coin flip is 50%.

  NONE OF THOSE HAD A LEVEL UNDER THEM.

  So this crate is the test of the sentence pattern/README.txt already ends on:
  a pattern is a description, and what makes one worth anything is the level it
  printed at.

  If these come back at 38% as well, the level does not save it. THAT IS A
  FINDING, NOT A FAILURE, and it is worth more than another rule would be.


THE PLACE TEST IS MEASURED FROM THE TAIL TIP

  A push is measured from its pin's tail -- the low on an nsc-bull, the high on
  an nsc-bear.

  ARGUED FROM WHAT THE PATTERN IS. The tail is a pullback that failed. If it
  reached into the level, the level is what stopped it, and that is the whole
  story of the setup. Measuring from the body would pass a shape whose
  rejection happened somewhere else and whose body merely ended up nearby.

  An engulfing has no tail to speak of, so it is measured from its close.


HALF A BAND, AND NO TOUCH RULE

  Inside the band is inside -- no depth rule.

  Outside it, half of THAT BAND'S OWN THICKNESS. Never a distance: a band on
  gold is about 78 points and on the euro about 0.004, and a number in points
  works on the pair it was set on and quietly stops working on every other.

  Asked whether the pin had to touch the band he said it need not, and that
  touching was no problem either. So there is NO TOUCH RULE AT ALL -- distance
  is the only test, and a pin that pokes inside measures as nought.


IT REPORTS, IT DOES NOT ENTER

  Version 1 sends signals and places no trades.

  Where the stop goes has not been settled, so a signal with no stop is a
  reading rather than a trade. reasons.rs never writes buy, sell, entry, target
  or stop, and a test pins that.
