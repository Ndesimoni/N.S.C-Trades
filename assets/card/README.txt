card/ — what the bot actually sends
===================================


WHAT THIS FOLDER IS FOR

  The pictures. Each file is one card, drawn by Chrome and sent to Telegram.

  THE DESIGN LIVES HERE, NOT IN RUST. Open a file, change it, run the bot —
  the next message picks it up. No rebuild, no code.


WHY PICTURES AND NOT TEXT

  Telegram gives text no font size, no colour and no layout. There is a
  ceiling on how good a message can look, and it is low.

  A picture has all three. So anything that has to look good goes on a card,
  and the Telegram caption is one line — which is really the notification
  banner rather than the message.


THE FILES

  style.css     THE PALETTE, THE TYPEFACES AND THE PAGE BOX — what every card
                agrees on. Dropped in where a template says __STYLE__, so a
                colour is changed in ONE place and every card follows.

                Inlined rather than linked, because the filled page is written
                next to the picture and a link would break the moment the two
                are not in the same folder.

  <name>.css    EACH CARD'S OWN STYLING, beside its template. Dropped in where
                the template says __OWN__.

                Split out because a card was a 350-line file — a thing you
                scroll rather than read. The markup and the script are what
                change; the CSS mostly sits still.

                A card with nothing of its own needs no file. Missing is not
                an error.

                A CARD'S HEIGHT LIVES HERE TOO, and it comes after style.css,
                so it wins.

  chart.html    The candle chart. Header band with the pair and the price, an
                accent bar tinted with the move, the candles with the price
                scale on the right, and the open, high, low and range along
                the bottom. THIS ONE STANDS UP ALONE.

                It draws his levels as bands when it is given any, and nothing
                when it is not — so the hourly card the bot sends and the
                weekly one with his levels on it are THE SAME FILE. They were
                two, and 87% of the second was a copy of the first.

  alert.html    PRICE HAS REACHED ONE OF HIS ZONES. The pair, a chip saying
                whether price is approaching, in it, or was already in it, and
                the zone drawn TWICE — see below.

                Shorter than the others and says so in its own CSS. An alert
                has one thing to say.

                The zone drawing is the whole reason this is a picture. Three
                numbers in a message have to be compared in his head; a band
                with a dot on it does not.

  close.html    A CANDLE THAT FINISHED OUTSIDE ONE OF HIS ZONES. The chip
                names WHAT IT DID — kissed it, pushed back, cut through — and
                the drawing shows it against the band.

                ONLY WHEN IT CLOSED OUTSIDE THE BAND, since 26 August 2026. A
                candle that settled inside the zone says nothing he does not
                already know from the approach alert. The REJECTION survives:
                a wick into the zone that closed back out finishes above or
                below the band. `only_breaks` in config/levels.toml.

                IT USED TO DRAW TWO THINGS — a finished candle, and one still
                running twenty minutes in, with a hollow chip and a hollow
                body so it could not be mistaken for a close.

                That second card went on 27 August 2026, and the hollow
                styling with it. NOTHING IN THIS PROJECT READS AN UNFINISHED
                CANDLE ANY MORE, so there is no longer a case to draw
                differently.

  heartbeat.html  THE ONLY CARD THAT SAYS NOTHING IS WRONG. Sent on a day
                nothing else was, at 07:00 UTC. Every pair, its levels as
                dots in HIS COLOURS, and how far price is from the nearest
                zone on each.

                The dots earn their place: a pair that quietly lost its daily
                levels shows up as a missing blue dot, where a count of 16
                would still look fine.

                ITS HEIGHT IS WORKED OUT, not typed — see below.

\1
                THREE PICTURES SINCE 30 AUGUST, sent as one bundle:

                  the run      200 candles, no ring   where price CAME FROM
                  the close-up   45 candles, red ring  where it PRINTED
                  this card     the shape itself       WHAT it was

                Widest first, then in. Both charts are chart.html -- the same
                template /chart uses -- so his levels are drawn in his own
                colours and one picture cannot disagree with the other.

                AND IT WEARS ITS TIER. Red for a shape in the zone, amber for
                one within half a band, plain for one with no zone near it.
                Red is reserved: if the weaker two ever wear it, the strongest
                thing the bot says stops looking any different.
                candles drawn on the band, and the one sentence that explains
                them.

                THE SENTENCE IS WRITTEN BY nsc-strategy, not by the card. If
                the rules cannot write it in one line, the rules are too loose
                -- that is a test of the rules and it belongs beside them.

                THE BAND AND THE CANDLES SHARE ONE SCALE. Drawn separately
                they would agree in the numbers and disagree in the picture,
                and a picture with a price on it is believed exactly like a
                number.

                THE viewBox AND THE CSS BOX ARE THE SAME SHAPE. They disagreed
                once and the chart floated in a field of white, drawn smaller
                than it should have been.

                IT NEVER SAYS BUY. Where the stop goes has not been settled, so
                the footer says what it is: a reading, not a trade.


  calendar.html WHAT IS ON THE CALENDAR — today, or the rest of the week.
                Sent when he asks with /news, never on its own.

                A LIST, NOT A RELEASE. Every row carries its own time, and the
                forecast and previous are left off: eighteen rows of numbers
                is a spreadsheet, and he is reading it on a phone.

                The week grows a heading per day. Today does not -- a heading
                over a list that is all one day is a line saying nothing.

                NOTHING IS LEFT OUT -- EVERY ROW SAYS WHICH SIDE OF NOW IT
                IS ON. Gone ones read PASSED and are greyed; the rest carry
                how long they have: in 45m, in 10h 53m, in 3d 10h.

                A week with its first three days missing does not read as a
                week, it reads as a quiet one. And "nothing left today" and
                "three already gone" are different afternoons.

                The header counts both halves -- "1 gone / 17 to come" --
                because "18 releases" does not say whether the day is over.

                THE COUNTDOWN IS NOT COLOURED. The stripe already carries
                impact, and a second colour on the same row competes with it.

                ITS HEIGHT IS WORKED OUT, not typed — see below.

  news.html     WHAT IS ABOUT TO PRINT ON THE ECONOMIC CALENDAR. Sent thirty
                minutes ahead, so a level sitting in front of a rate decision
                is not read like one on a quiet Thursday.

                RED IS HIGH, ORANGE IS MEDIUM, YELLOW IS LOW -- and that is
                FOREXFACTORY'S SPELLING, not a design choice. He reads that
                calendar every day. Red meaning anything else here would make
                him translate the card, every time, under time pressure.

                The rule under the header takes the HEAVIEST rating on the
                card, not the first one listed. One high and two mediums is a
                high-impact card.

                ONE CARD PER RELEASE, NOT PER LINE. Three Australian CPI
                numbers print in the same second and share a card. Sent apart
                they would buzz his phone three times for one event.

                ITS HEIGHT IS WORKED OUT, not typed — see below.

  armed.html    A LEVEL HE JUST SENT IS BEING WATCHED. A receipt, not a
                report: a tick, "Got it", and the count.

                IT NAMES NOTHING. The inbox has already sent him a picture of
                where the bands landed with the pair on it in his colours —
                repeating that is a second message telling him what he already
                had.

                The count is the useful part: he sees the number went up
                without being told which pair, which he knows.

  trouble.html  SOMETHING HAS GONE WRONG WITH THE BOT. No pair, no price, no
                chart — it must never be mistaken for a signal.

                THE COLOUR ANSWERS THE ONLY QUESTION HE HAS: do I need to get
                up? Amber, the line is down and it is trying — no. Green, it
                is back — no. Red, it has stopped and will not restart — yes.

                A heavy stripe down the left and a wash of the same colour
                behind the header, because a thin line on an 860-wide card is
                not an alarm.

                Then: how long, what it means, what happens next, and the raw
                wording the code used — small, at the bottom, where it does
                not shout. He is not going to debug it, but "connection
                refused" and "invalid API key" are different evenings.



THEY ARE PIECES, NOT A SET

  A message picks the cards it needs:

    a candle closed     chart.html, on its own
    a price alert       alert.html, on its own. No chart, because nothing
                        has formed yet
    a candle closed     close.html, on its own. What that one candle did at
                        the zone — still not a trade
    a signal            the chart with your levels and the entry, stop and
                        target on it, plus a card carrying the reasoning

  Several pictures go as one media group. The phone buzzes once and each
  picture still opens on its own when tapped.


ONE DRAWING, ZOOMED TO THE CANDLE

  A 1-hour candle inside a weekly zone can be a hundredth of its height. Drawn
  to the band's scale it is a smudge, and a smudge says nothing about what the
  candle did — which is the only thing the card is for.

  So close.html zooms to the CANDLE, exactly like zooming in on his own chart.
  Everything stays true; there is just more band than fits.

  Three things keep that honest:

    - HIS LINE IS KEPT IN VIEW whenever it is anywhere near. A picture of a
      candle with no line in it answers nothing.

    - A BOUNDARY THAT FELL OFF THE VIEW GETS AN ARROW — "↑ top 4,414.17". It
      must never be silently missing, because the candle is being judged
      against it.

    - THE STRETCH OF WICK INSIDE THE ZONE IS PICKED OUT in the level's own
      colour. How deep it went is the difference between a graze and a
      rejection, and this makes it a length rather than a number to look up.

  A pictogram was tried on the left of both cards and taken back out. Two
  drawings of one thing is one too many — the measured drawing does the job
  once it can actually be seen.


TWO THINGS THAT WILL CATCH YOU

  1. EACH CARD SAYS HOW TALL IT IS.

     At the top of the CSS:

         --card-height:647px;

     Rust reads that line straight out of the file. Chrome screenshots a
     WINDOW, not a page, so something has to say how tall — and the file
     being designed is the honest place for it. Two numbers in two files
     drift apart; one does not.

     Change the design, change that number. It is measured by hand on every
     card but one, which is a known rough edge.

     THREE EXCEPTIONS: heartbeat.html, news.html and calendar.html. They grow
     a row each -- per pair, per release -- so they write `--card-height:/*__TALL__*/px` and
     Rust fills the number in. That works because fill.rs reads the height
     AFTER the facts go in, and if the marker is ever left unfilled the card
     FAILS rather than falling back on the shared height and clipping the last
     row off.

     news.css AND calendar.css GO FURTHER AND PIN THEIR PARTS. The header, the row
     and the footer are given fixed heights rather than being left to grow
     with their content, and card/soon.rs adds those same numbers up.

     That is there because it went wrong: the row constant had been copied
     from heartbeat.html, whose rows are ONE line where these are two. Chrome
     shoots a window, not a page, so the fourth release was simply cut off --
     a card headed "4 releases" with three on it, and nothing failed. It reads
     as a quieter week, not as a bug.

     card/tests/growing.rs now reads news.css and checks the two still agree.

     style.css carries a shared one and every template gets it dropped in at
     the top. A card wanting its own puts it FURTHER DOWN, because the last
     one wins — the same way the browser resolves it. alert.html does.

  2. CHROME ALWAYS LEAVES 87 PIXELS OF WHITE.

     It hands the page a viewport 87px shorter than the window asked for, and
     paints the rest white. Measured, not guessed: ask for 600 and the page
     gets 513; ask for 900 and it gets 813.

     So Rust asks for 87 extra and cuts them off afterwards. The old headless
     mode did not do this, and it has been removed from Chrome.


COLOURS THAT ARE NOT OURS TO CHOOSE

  When levels arrive, they are drawn in the trader's own colours:

      black    weekly
      blue     daily
      yellow   4-hour

  Drawing every level in one colour was done once already and the chart
  looked nothing like his.


AND ONE HONEST LIMIT

  This is OUR DRAWING of the broker's candles, not a photo of his platform.
  The spacing and styling will never match exactly.

  If a PRICE differs from his chart, that is a real bug worth chasing.
  If the LOOK differs, that is just us.


THE PRICE SCALE ON chart.html

  THE CANDLES DECIDE IT. Nothing else does.

  A level is drawn if it lands on that scale, and simply is not on the chart
  if it does not. What is in the screen is in the screen.

  It used to stretch the scale to reach every level. Gold has a weekly line at
  2,984 with price at 4,352, so 45 candles with a 6-point range came out as a
  smear one pixel tall -- a picture of the levels with the price accidentally
  in it. His call, 1 September 2026, and the same on every timeframe.
