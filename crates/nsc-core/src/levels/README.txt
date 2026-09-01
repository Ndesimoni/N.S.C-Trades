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

  write.rs    ADDING. Putting a level into a pair's file, and starting the
              file if the pair is new.

  remove.rs   TAKING AWAY. Undoing the last levels, and stopping a pair
              altogether.

  naming.rs   What a pair's name tells you — how many decimals, whether it
              shuts at night. Worked out, not checked.

  alert.rs    What an alert SAYS — the one-line caption under the picture,
              and the sentence on the card. Here, in the crate that cannot
              reach anything, so the words can be read and tested without a
              network or a browser.

  close.rs    WHAT A FINISHED CANDLE DID at a zone. Two answers, and they
              are different questions:

                AtZone  where it ENDED — above, below, inside
                Action  what KIND of thing happened — kissed it, pushed
                        back, closed inside, cut through

              A wick that grazed the edge and a candle that drove a third of
              the way in both "closed above". They are not the same event,
              and the one he acts on is the second.

  watch.rs    Watching bands. Holds one fact per band -- the last close each
              timeframe reported there -- and says whether a candle closing
              at one is worth a card. Also `came_from`, which reads a break
              off the candle's own open.

  error.rs    LevelError. Which troubles are worth another go, and which are
              settled — a file that will not parse, or a pair already being
              watched.

  tests/      Seventy-eight tests.
                bands.rs    does our band land where his landed
                closing.rs  what a finished candle did at a zone
                pips.rs     one pip from the pair's digits, and a pair
                            asking for its own approach distance
                bands.rs    does our band land where his landed
                closing.rs  where a finished candle ENDED at a zone
                acting.rs   what KIND of thing it did there
                saving.rs   levels arriving from his phone
                removing.rs undo, taking one off, stopping and restoring
                watching.rs firing once per touch, not once per price
                support.rs  what they share

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

  So STOPPING a pair is moving its file — `retire` puts it in
  config/pairs/removed/, which `known` cannot see because it only looks at
  .toml FILES and that is a folder.

  MOVED, NOT DELETED. He does this by tapping a button on a phone, and it
  throws away every level he has drawn for that pair. Moving it back starts
  the pair again, and the reply tells him where it went.

  There is no second list. The buttons in Telegram are these files, and a
  pair with no levels is a pair the bot has nothing to say about anyway.


TWO THINGS ABOUT THE FILES THEMSELVES

  LEVELS ARE APPENDED, AND UNDO CUTS. Neither rewrites the file. These files
  carry comments explaining what a level is and where the numbers came from,
  and rewriting would delete all of it without a word. He is meant to be able
  to open one and read it.

  THE SAME PRICE IS ONE LEVEL, WHATEVER CHART IT ARRIVES ON. He sent three
  euro levels twice and got both copies: one line on his chart became two
  bands, two alerts, two closes, and a heartbeat card claiming seven levels
  where he had drawn four.

  Compared as NUMBERS, not as text — 1.15 and 1.15000 are the same line and he
  may type either. Repeats inside one message are dropped too, because tapping
  send twice is the commonest way it happens.

  AND THE TIMEFRAME IS NOT PART OF WHAT MAKES IT UNIQUE. He draws one line at
  1.15000. Sending it again off his daily chart has not changed anything about
  it, and a second band round the same line is the same duplicate wearing a
  different label — a 62-pip band and a 29-pip band round one line, firing
  twice as price passes through.

  IT KEEPS THE CHART HE FIRST DREW IT ON, and with it the thickness. That is
  not a detail: weekly is 62 pips on the pound where daily is 29.

  So the reply NAMES what he already had and which chart it is on — "1.15000
  is already a weekly level". He may have re-sent it off the daily expecting
  it to move; saying nothing would leave him thinking it had.

  PRICES ARE STORED AS TEXT — price = "1.21279". Written as a TOML number it
  would go through a float and stop being exactly that. There is a test that
  saves one and reads it back.


PRICE REACHING A LEVEL SAYS NOTHING

  HIS CALL, 31 AUGUST 2026: "when price is getting to a level we do not want an
  alert, so remove the card. We should only get alerts if the price came from
  below the band level and closed above it, and vice versa."

  So rung 1 is gone. Two messages are left in the whole bot:

      a candle BREAKS a level    came from below and closed above, or
                                 came from above and closed below
      a shape he trades          at a level, or within half a band of one

  Price walking into a zone, sitting in it, wobbling at its edge, leaving and
  coming back -- all silent.

  WHY IT WENT. It had three goes at being quiet enough: once per touch, then
  once per visit, then once per level ever. Each cut the noise and none of them
  fixed the real thing, which is that HE DREW THE LINE AND KNOWS WHERE IT IS.
  What he cannot see without sitting at the screen is a candle finishing on the
  other side of it.

  WHAT WENT WITH IT. A "deepest price has got this visit" per band, a leaving
  distance measured wider than the arriving one, and a "has this level ever
  spoken" flag. All three existed so that one visit fired one alert. With no
  alert there is no visit to count. They are in git at 99ed9f1.


A BREAK IS READ OFF THE CANDLE, NOT OFF THE TICKER

  Came from below and closed above -- that is a break, and it is a card.
  Came from below and closed BELOW is a rejection, and it is silent.

  WHICH SIDE IT CAME FROM IS THE CANDLE'S OWN OPEN. It was remembered from the
  price stream for about an hour on 31 August, and the price stream and the
  candle poll are not the same clock: a 4-hour candle closes above at 12:00,
  the poll runs seconds later, and by then the ticker has already put price
  above. The break read as a rejection and went silent -- and the harder the
  break, the more certainly it did.

  The open cannot race. It is a fact about a candle that has finished. It is
  also THE ONLY VERSION A BACKTEST CAN RUN, and that is what really decided it.

  THE REJECTION IS NOT LOST. A rejection at a level is the reversal he trades,
  and if a shape printed there it reaches him as a SETUP -- which names the
  shape, where the close card could only say the candle ended below.

  A CANDLE THAT SETTLED INSIDE the band is not a break either. `only_breaks` in
  config/levels.toml is `true` since 31 August, and that is the one line to
  flip if the bot ever goes too quiet.


HOW CLOSE COUNTS AS BEING AT A BAND

  `approach_share` in config/levels.toml -- a twentieth of that band's own
  thickness. It was written in pips until 31 August 2026 and was the last
  distance in this project that was. Four pips is 22% of an AUD/USD daily band
  and 0.03% of a gold weekly one, so the same setting meant two different
  things.

  ONLY ONE THING READS IT NOW: the report sent when watching resumes, which
  says which zones price is already sitting in. Nothing else in the bot cares
  how near price is to a line.

  ANY PAIR CAN OVERRIDE IT with its own `approach_share`. There is a commented
  example in config/pairs/XAUUSD.toml.


WHAT IS NOT HERE YET

  Removing one particular level. Undo takes off what the last message added,
  which covers a typo but not "that 3800 from last week was wrong".

  A backtest. Nothing has measured whether any of this makes money, because
  nsc-backtest does not exist. `came_from` reading the candle rather than the
  ticker is what keeps that door open -- a backtest has no ticker.
