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

  style.css     The palette, the typefaces and the page box. Rust drops it
                into every template where the template says __STYLE__, so a
                colour is changed in ONE place and every card follows.

                Inlined rather than linked, because the filled page is written
                next to the picture and a link would break the moment the two
                are not in the same folder.

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

  close.html    A CANDLE THAT TOUCHED ONE OF HIS ZONES HAS FINISHED. Named
                by WHAT IT DID — kissed it, pushed back, closed inside, cut
                through — with the candle drawn on the band and its open,
                high, low and close.

  readout.html  Where price sat inside the candle. The candle drawn tall on
                the left, with leader lines out to High, Open, Close and Low.
                Sent when the detail is wanted, not every hour.

  README.txt    This file.


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


TWO PICTURES OF THE SAME THING, ON PURPOSE

  Both zone cards draw the band twice, and the two drawings are not
  duplicates. They answer different questions.

  ON THE LEFT, A PICTOGRAM. Fixed size, fixed positions, NOT to scale. A kiss
  looks like a kiss on gold and on the euro and on the card he saw last week.
  That is the whole point: the eye reads it without measuring anything.

  ON THE RIGHT, THE MEASURED DRAWING. Real proportions, leader lines out to
  real numbers. How far, how deep, exactly where.

  What happened, then how much. Try to make one drawing do both and it does
  neither — either the shapes stop being recognisable or the numbers stop
  being true.

  The left one is drawn from ONE FACT: the action name, worked out in
  nsc-core::levels::close. Nothing on the card decides what happened.


TWO THINGS THAT WILL CATCH YOU

  1. EACH CARD SAYS HOW TALL IT IS.

     At the top of the CSS:

         --card-height:647px;

     Rust reads that line straight out of the file. Chrome screenshots a
     WINDOW, not a page, so something has to say how tall — and the file
     being designed is the honest place for it. Two numbers in two files
     drift apart; one does not.

     Change the design, change that number. It is measured by hand today,
     which is a known rough edge.

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
