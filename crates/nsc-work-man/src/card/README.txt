card/ — turning candles into a picture
======================================


WHAT THIS FOLDER IS FOR

  Putting real numbers into an HTML template and asking Chrome to take a
  photograph of it.

  THE DESIGN IS NOT IN HERE. It is in assets/card/*.html. Open one, change
  it, run the bot — the next message picks it up. No rebuild, no Rust.

  This folder only does the plumbing between the two.


THE FILES

  mod.rs      The front door.

  fill.rs     Reads a template, puts the numbers in, works out how tall it
              is, and hands it to Chrome. Start here. Every card goes through
              its `draw`.

  alive.rs    The heartbeat card. Works out its own height, because it grows
              a row per pair.

  live.rs     The armed card — a level he just sent is being watched.

  wrong.rs    The trouble card — the bot itself, not the market.

  zone.rs     THE ZONE CARDS, and the facts they are told. What a zone card
              needs is nothing like a candle chart — a band, and either a
              price or one candle — so it keeps its own.

              THE ZONE CARDS. Price arriving at one of his zones (rung 1),
              and a candle that touched one having finished (rung 2). Their
              own file because what they are told is nothing like a chart: a
              band, and either a price or one candle.

  facts.rs    Turns CANDLES into the numbers a template can read. Nothing in
              here knows about colours or layout, and nothing in a template
              works out a price.

  chrome.rs   Runs Chrome headless, then cuts the white strip off the
              bottom.

  tests.rs    Ten tests, on the two things that have actually gone wrong.

  README.txt  This file.


TWO THINGS THAT HAVE CAUGHT US, BOTH NOW PINNED BY TESTS

  1. THE HEIGHT LIVES IN THE TEMPLATE

     At the top of each card's CSS:

         --card-height:647px;

     It is in the card's OWN CSS — assets/card/<name>.css — which is dropped
     in after style.css and therefore wins.

     fill.rs reads it AFTER the facts have gone in, which is what lets the
     heartbeat have a height that depends on how many pairs are on it.

     The tests assemble a card the same way fill.rs does, style then own then
     markup. They used to read the .html alone, and when the styling moved
     into <name>.css they both passed on a file that no longer held the number
     they were checking. Two failed the moment the move happened, which is
     what they are for.

     Chrome screenshots a WINDOW, not a page, so something has to say how
     tall. The file being designed is the honest place for it — two numbers
     in two files drift apart, one does not.

     THE LAST ONE WINS, which is what the browser does. style.css sets a
     shared height and is dropped in at the top of every template, so a card
     that wants its own says so further down. alert.html is the first to do
     it — an alert has one thing to say and chart height would send half a
     screen of white.

     The height was held in Rust first. It clipped the footer four times.

  2. ROUNDING HAPPENS ON THE WAY OUT, AND NOWHERE ELSE

     The feed sends gold as 4385.59525. Gold is quoted to two decimals.

     facts.rs rounds every price to the instrument's own precision as it
     hands it over. Let all five through and a card reads like a debug dump
     rather than a signal.

     That is also the only place a price becomes a float, because JSON has no
     other kind of number. Everything before it is Decimal.


THE TRAP IN chrome.rs

  CHROME ALWAYS LEAVES 87 PIXELS OF WHITE.

  It hands the page a viewport 87px shorter than the window asked for, and
  paints the rest white. Measured, not guessed: ask for 600 and the page gets
  513; ask for 900 and it gets 813.

  So the window is asked for 87 taller and the strip is cut off afterwards.
  The old headless mode did not do this, and it has been removed from Chrome.

  BOTH PATHS MUST BE ABSOLUTE. Chrome runs with its own working folder. Give
  it file://preview/chart.html and it reads "preview" as a HOSTNAME, fails to
  reach it, and quietly screenshots its own error page — which then goes to
  Telegram looking like a real card. That has happened.

  And Chrome answers 0 whether it drew your card or its own error page. The
  only honest check is whether a file appeared.

  WHICH MEANS THE LAST PICTURE HAS TO BE CLEARED FIRST. One was already there,
  left by the last card of the same kind — so a failed draw left the old
  picture in place, passed the check, and would have gone out with today's
  caption on yesterday's chart.


THE COST OF ALL THIS

  Whatever machine runs the bot needs Chrome installed. Fine on a Mac. A real
  dependency on a server, and worth remembering before it goes anywhere but a
  laptop.
