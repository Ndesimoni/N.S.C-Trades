levels/ — the lines he drew
===========================


WHAT THIS FOLDER IS FOR

  A LEVEL IS A BAND, NOT A LINE.

  Price does not stop at a number. It turns somewhere near one. So "price is
  at the level" means price is INSIDE a band, and this folder is about
  turning the one price he sends into that band.


THE FILES

  mod.rs      The front door.

  band.rs     Band and Timeframe. A line plus a share of a normal candle
              becomes a top and a bottom. Also his colours, which are a
              specification and not ours to choose.

  read.rs     Reading config/levels.toml and config/pairs/*.toml.

  write.rs    Putting a level into a pair's file, starting the file if the
              pair is new, and taking levels back off again.

  naming.rs   What a pair's name tells you — how many decimals, whether it
              shuts at night. Worked out, not checked.

  watch.rs    Watching bands for price ARRIVING at one. Holds one fact per
              band — is price at it now — and an alert is the moment that
              turns from no to yes.

  tests/      Nineteen tests.
                bands.rs    does our band land where his landed
                saving.rs   levels arriving from his phone, and going back off
                support.rs  what both of those share

  README.txt  This file.


THE THICKNESS IS A SHARE OF A CANDLE, NEVER A PRICE

      weekly   0.35 of a weekly candle
      daily    0.46 of a daily candle
      4-hour   0.55 — A GUESS, never measured

  Written as a price it would be right on one pair and absurd on the rest.
  78 points is a normal week on gold and about a year on EURUSD.

  As a share of a normal candle it travels: 0.35 gives 78 on gold and about
  0.004 on the pound, and both look the same on a chart. Which is what he is
  actually doing when he draws one — sizing it by eye against the candles in
  front of him.

  The numbers are in config/levels.toml with the evidence, and the working is
  in docs/worksheets/levels.md.


THE RULE WAS CHECKED AGAINST HIS OWN HAND

  He drew these on gold. We build these from the line price alone:

      his                        ours
      4055.913 - 4132.020        4055.42 - 4132.57
      3303.553 - 3383.480        3305.42 - 3382.57
      2968.181 - 3000.463        2967.81 - 3000.18

  Within two points on every edge. tests/bands.rs pins it against those exact
  numbers, so a change to the rule that stops matching his hand fails.


A PAIR EXISTS BECAUSE ITS FILE EXISTS

  config/pairs/XAUUSD.toml is why gold is watched. Delete it and it stops.

  There is no second list. The buttons in Telegram are these files, and a
  pair with no levels is a pair the bot has nothing to say about anyway.


TWO THINGS ABOUT THE FILES THEMSELVES

  LEVELS ARE APPENDED, AND UNDO CUTS. Neither rewrites the file. These files
  carry comments explaining what a level is and where the numbers came from,
  and rewriting would delete all of it without a word. He is meant to be able
  to open one and read it.

  PRICES ARE STORED AS TEXT — price = "1.21279". Written as a TOML number it
  would go through a float and stop being exactly that. There is a test that
  saves one and reads it back.


"AT IT" IS WIDER THAN "INSIDE IT"

  A BAND IS A ZONE, NOT A WALL. Price at 4133 when the top is 4132.57 is at
  that level in every way that matters, and waiting for it to cross by half a
  dollar is arbitrary.

  There is a second reason and it may be the bigger one: AN ALERT THAT ONLY
  FIRES ONCE PRICE IS INSIDE ARRIVES TOO LATE TO USE. Told it is coming, he
  can get to his chart before the reaction starts.

  How close counts is `approach` in config/levels.toml — a share of the band's
  own thickness, so it travels between instruments the same way the band does.

      0.25 on gold weekly   fires about 19 points out
      0.25 on gold daily    fires about 8 points out
      0.25 on the pound     fires about 15 pips out

  The alert says which it is — price IS IN the zone, or is COMING UP ON it.
  Different things, and he should not have to work it out from the numbers.


FIRE ONCE PER TOUCH, NOT ONCE PER PRICE

  Prices come down the websocket about once a second and barely move —
  4375.35, 4375.36, 4375.35. Without a rule, one visit to a level becomes
  twenty alerts and he stops reading them.

  So an alert is a CHANGE: price was outside, now it is inside. Sitting there
  says nothing more.

  Two things that took thinking:

  THE FIRST PRICE NEVER FIRES. It says where price IS. It cannot say price has
  ARRIVED — it may have been sitting in that band for hours before the bot
  started, and an alert for that is a lie about when it happened.

  HOVERING ON THE EDGE DOES NOT FIRE REPEATEDLY. 4131.99, 4132.01, 4131.99
  against a top of 4132.00 crosses three times and describes one moment where
  nothing happened. Price has to get clear of the band by a tenth of its
  thickness before that band can fire again.

  Both have tests that fail without the rule.


WHAT IS NOT HERE YET

  Removing one particular level. Undo takes off what the last message added,
  which covers a typo but not "that 3800 from last week was wrong".

  Nothing watches these. Prices arrive on the websocket and nothing yet asks
  whether one has entered a band. That is the price watcher.
