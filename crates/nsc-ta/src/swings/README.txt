swings/ — finding the peaks and troughs
=======================================


WHAT THIS FOLDER IS FOR

  Turning candles into swing points.

  This is the foundation of everything else. Levels, trendlines, Fibonacci,
  trend direction and chart patterns are all built on what comes out of here.

  Get the sensitivity wrong and every feature downstream is quietly rubbish,
  in a way that is very hard to trace back to this folder.


THE FILES

  mod.rs              The front door. Lets the outside world see two things:
                      SwingFinder and find_swings.

  finder.rs           SwingFinder. Takes candles one at a time. This is what
                      the live bot uses.

  series.rs           find_swings. Takes a whole history at once. This is
                      what the backtester uses. It runs the same struct.

  tests/              Twelve tests.
    helpers.rs        building candles to test with
    detection.rs      does it find the right swings
    guards.rs         does it refuse what it should refuse

  README.txt          This file.


HOW THEY FIT TOGETHER

      finder.rs ─►  series.rs         series runs the same struct in a loop

      finder.rs ─►  indicators/atr/   the noise filter is measured in ATR
      finder.rs ─►  config/           lookback and the filter setting
      finder.rs ─►  nsc-core swing/   the Swing type it produces

      mod.rs    ─►  lets the outside world see SwingFinder and find_swings


HOW A SWING IS FOUND

  A swing high is a candle whose high beats the highs of a few candles on
  either side.

  How many candles either side is the "lookback" setting in ta.toml. With a
  lookback of 3, the finder looks at a window seven candles wide and asks
  whether the middle one beats the three before it and the three after it.

  A swing low is the same thing upside down.


WHY THE ANSWER IS ALWAYS ABOUT AN OLDER CANDLE

  To know candle 100 was a peak, you have to see the candles after it.

  So when candle 103 arrives, the finder decides about candle 100. It is
  always three candles behind.

  That lag is not a limitation to work around. It is the honest answer.
  Anything faster is reading the chart backwards.

  This is also why the last few candles of any history produce no swings.
  Correct, not a gap — tomorrow they might be swings.


CONFIRMATION IS BUILT IN, NOT REMEMBERED

  Every swing carries two times: where it sits, and when it became knowable.

  The important part is that this folder CANNOT produce an unconfirmed swing.
  The finder has no opinion about a candle until the candles after it have
  arrived. So there is no unconfirmed swing anywhere for someone to
  accidentally use.

  That is better than a rule people have to remember. Rules get forgotten;
  this cannot be forgotten because the thing does not exist.

  A note on ta.toml: there is a "require_confirmed" setting in the [swings]
  section. Given the above, it does nothing — an unconfirmed swing is not
  possible here. Worth deleting so nobody assumes it is protecting them.


DECISION ONE: A TIE IS NOT A SWING

  The middle candle must STRICTLY beat its neighbours. Equal is not enough.

  So a flat double top — two candles at exactly the same high, next to each
  other — produces no swing at all.

  That misses a real level occasionally. The alternative is worse: loosen it
  and a flat stretch produces several swings at the same price, and every
  level and trendline built from them is wrong in a way that looks fine.

  Missing a level is safer than inventing one. You can see a missed level on
  your own chart. An invented one you cannot see at all.

  Exact ties are rare in forex, where prices have five decimal places. They
  are more likely on the indices, which move in whole points.

  Test: a_flat_top_produces_no_swing


DECISION TWO: HOW FAR IS FAR ENOUGH

  In a choppy market almost every candle beats its immediate neighbours by a
  hair. Without a filter you get hundreds of "swings" that are just noise.

  So a swing must also stand out by a minimum amount: the "min_atr_multiple"
  setting.

  It is measured in ATR — the size of a normal candle — not in pips. With the
  setting at 0.5, a swing must stand out from its neighbours by half a normal
  candle.

  That same 0.5 works on EURUSD and gold. A pip threshold would need
  retuning for every instrument, and again every time volatility changed.

  Tests: a_bump_smaller_than_the_filter_is_ignored
         turning_the_filter_off_finds_the_same_bump


NOTHING IS FOUND UNTIL ATR EXISTS

  The filter needs ATR, and ATR needs 14 candles.

  So the first 14 candles of any history produce no swings, even if there is
  an obvious peak among them.

  That is deliberate. Without ATR there is no idea what a normal candle looks
  like on this instrument, so there is no way to tell a real swing from how
  this thing always behaves. Finding nothing beats guessing.

  Test: nothing_is_found_before_atr_has_warmed_up


A CANDLE CAN BE BOTH

  An outside bar — one that makes the highest high AND the lowest low in the
  window — is both a swing high and a swing low.

  That is why update() returns a list rather than a single swing. Usually the
  list is empty, which costs nothing.

  Test: an_outside_bar_is_both_a_high_and_a_low


ONE AT A TIME, AND ALL AT ONCE

  The live bot gets candles one at a time. The backtester has the whole
  history.

  Both go through the same struct. find_swings feeds candles into the same
  SwingFinder the bot uses.

  Not similar code. The SAME code.

  If those two ever gave different answers, your backtest would stop
  describing what the bot does — and you would not notice, because that kind
  of mismatch makes backtests look better rather than broken.

  Test: one_at_a_time_matches_all_at_once


IF YOU CHANGE THE LOOKBACK

  Everything downstream changes. Every level, every trendline, every
  Fibonacci anchor, every trend reading.

  So do not nudge it because one chart looks nicer. Test it in the
  backtester, read what changed, and decide whether you agree with the new
  levels before accepting them.

  And if a small change to the lookback makes a big change to your results,
  that means the strategy was fitted to the old setting. Not that the new
  setting is better.
